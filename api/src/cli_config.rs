use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        short,
        min_values = 1,
        required = true,
        help = r#"Example --hosts "name=my_machine,mac_address=...,broadcast_ip=...""#
    )]
    pub hosts: Vec<HostConfig>,
}

#[derive(Debug)]
pub struct HostConfig {
    pub name: String,
    pub mac_address: String,
    pub broadcast_ip: String,
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
