use std::collections::HashMap;
use std::env::{set_var, var, VarError};

use api::{list_wakes::list_wakes, poll::poll, wake::wake};
use async_std::sync::RwLock;
use cli_config::Config;
use log::info;
use structopt::StructOpt;

mod api;
mod cli_config;

const RUST_LOG: &str = "RUST_LOG";

pub struct Host {
    mac_address: String,
    broadcast_ip: String,
    entries: Vec<u64>,
}

type HostsMap = HashMap<String, Host>;
pub type Request = tide::Request<RwLock<HostsMap>>;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    if let Err(VarError::NotPresent) = var(RUST_LOG) {
        set_var(RUST_LOG, "trace")
    }
    env_logger::init();
    let config = Config::from_args();
    let mut wakes: HostsMap = HashMap::new();
    for host in config.hosts {
        info!("Adding machine {} with mac {}", host.name, host.mac_address);
        wakes.insert(
            host.name,
            Host {
                mac_address: host.mac_address,
                broadcast_ip: host.broadcast_ip,
                entries: vec![],
            },
        );
    }

    let bind_address = "0.0.0.0:8089";

    let mut app = tide::with_state(RwLock::new(wakes));
    app.at("/poll/machine/:name/since/:time").get(poll);
    app.at("/wake/machine/:name").get(wake);
    app.at("/wakes/machine/:name").get(list_wakes);
    app.at("/health").get(|_| async { Ok("OK") });

    app.listen(bind_address).await
}
