use std::env::var;
use std::{collections::HashMap, sync::Arc};

use async_std::sync::RwLock;
use color_eyre::{eyre::Context, Result};
use opentelemetry::sdk;
use opentelemetry_semantic_conventions as semcov;
use structopt::StructOpt;
use tide_tracing::TraceMiddleware;
use tracing::{info, info_span, Instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::api::{list_wakes::list_wakes, poll::poll, wake::wake};
use crate::cli_config::Config;

mod api;
mod cli_config;

pub struct Host {
    mac_address: String,
    broadcast_ip: String,
    entries: Vec<u64>,
}

type HostsMap = HashMap<String, Host>;
pub type Request = tide::Request<Arc<RwLock<HostsMap>>>;

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let config = Config::from_args();
    let tracer = opentelemetry_otlp::new_pipeline()
        .with_env()
        .with_trace_config(sdk::trace::config().with_resource(sdk::Resource::new(vec![
            semcov::resource::SERVICE_NAME.string("dd-wrt-wol-api"),
        ])))
        .with_tonic()
        .install_simple()?;

    let error = ErrorLayer::default();
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let stdout = tracing_subscriber::fmt::layer();
    let telemetry = match var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        Ok(_) => Some(tracing_opentelemetry::layer().with_tracer(tracer)),
        _ => None,
    };

    tracing_subscriber::registry()
        .with(error)
        .with(env_filter)
        .with(stdout)
        .with(telemetry)
        .init();

    let setup_span = info_span!("hosts_setup");
    let mut wakes: HostsMap = HashMap::new();
    {
        let _ = setup_span.enter();
        for host in config.hosts {
            info!(message="Adding machine", %host.name, %host.mac_address);
            wakes.insert(
                host.name,
                Host {
                    mac_address: host.mac_address,
                    broadcast_ip: host.broadcast_ip,
                    entries: vec![],
                },
            );
        }
    }

    let bind_address = "0.0.0.0:8089";

    let mut app = tide::with_state(Arc::new(RwLock::new(wakes)));
    app.with(TraceMiddleware::new());
    app.at("/poll/machine/:name/since/:time").get(poll);
    app.at("/wake/machine/:name").get(wake);
    app.at("/wakes/machine/:name").get(list_wakes);
    app.at("/health").get(|_| {
        async {
            info!("I'm alive!");
            Ok("OK")
        }
        .instrument(info_span!("health_check"))
    });

    app.listen(bind_address).await.wrap_err("failed to listen")
}
