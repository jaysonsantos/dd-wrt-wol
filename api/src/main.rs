use std::collections::HashMap;
use std::env::{set_var, var, VarError};
use std::str::FromStr;
use std::time::SystemTime;

use async_std::sync::RwLock;
use log::info;
use structopt::StructOpt;
use tide::Body;

use dd_wrt_wol_common::events::{Event, Response, Wakeup};

const RUST_LOG: &str = "RUST_LOG";

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(
        long,
        short,
        min_values = 1,
        required = true,
        help = r#"Example --hosts "name=my_machine,mac_address=...,broadcast_ip=...""#
    )]
    hosts: Vec<HostConfig>,
}

#[derive(Debug)]
struct HostConfig {
    name: String,
    mac_address: String,
    broadcast_ip: String,
}

impl FromStr for HostConfig {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut config: HashMap<&str, &str> = HashMap::new();
        for config_item in s.split(',') {
            let parts: Vec<&str> = config_item.split('=').collect();
            if parts.len() != 2 {
                panic!(
                    "Invalid host configuration on line {} it has {} parts instead of 2",
                    config_item,
                    parts.len()
                )
            }
            config.insert(parts[0], parts[1]);
        }
        let name = config
            .get("name")
            .expect("Missing name on host config")
            .to_string();
        let mac_address = config
            .get("mac_address")
            .expect("Missing mac_address on host config")
            .to_string();
        let broadcast_ip = config
            .get("broadcast_ip")
            .expect("Missing broadcast_ip on host config")
            .to_string();

        Ok(HostConfig {
            name,
            mac_address,
            broadcast_ip,
        })
    }
}

struct Host {
    mac_address: String,
    broadcast_ip: String,
    entries: Vec<u64>,
}

type HostsMap = HashMap<String, Host>;
type Request = tide::Request<RwLock<HostsMap>>;

async fn poll(request: Request) -> tide::Result {
    let hosts = request.state().read().await;
    let machine_name = request.param::<String>("name")?;
    let since = request.param::<u64>("time")?;
    let response = if let Some(host) = hosts.get(machine_name.as_str()) {
        let response =
            if let Some(entry) = host.entries.iter().filter(|entry| entry > &&since).last() {
                Response {
                    event: Event::Wakeup(Wakeup {
                        mac_address: host.mac_address.clone(),
                        broadcast_ip: host.broadcast_ip.clone(),
                        time_of_occurrence: *entry,
                    }),
                }
            } else {
                Response {
                    event: Event::Ignore,
                }
            };

        let mut tide_response = tide::Response::new(tide::StatusCode::Ok);
        tide_response.set_body(Body::from_json(&response)?);

        tide_response
    } else {
        let mut response = tide::Response::new(tide::StatusCode::NotFound);
        response.set_body(Body::from_json(&Response {
            event: Event::MachineNotFound,
        })?);

        response
    };
    Ok(response)
}

async fn wake(request: Request) -> tide::Result {
    let mut hosts = request.state().write().await;
    let machine_name: String = request.param("name")?;

    let response = if let Some(host) = hosts.get_mut(machine_name.as_str()) {
        host.entries.push(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let mut response = tide::Response::new(tide::StatusCode::Accepted);
        response.set_body(Body::from_string(format!("Waking up {}", machine_name)));

        response
    } else {
        let mut response = tide::Response::new(tide::StatusCode::NotFound);

        response.set_body(Body::from_string(format!(
            "Machine {} not found",
            machine_name
        )));

        response
    };

    Ok(response)
}

async fn list_wakes(request: Request) -> tide::Result {
    let hosts = request.state().read().await;
    let name: String = request.param("name")?;
    if let Some(host) = hosts.get(name.as_str()) {
        let mut response = tide::Response::new(tide::StatusCode::Ok);
        response.set_body(Body::from_string(format!(
            "Wakes for {} {:?}",
            name, host.entries
        )));

        Ok(response)
    } else {
        let mut response = tide::Response::new(tide::StatusCode::NotFound);
        response.set_body(Body::from_string(format!("Machine {} not found", name)));

        Ok(response)
    }
}

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

    app.listen(bind_address).await
}
