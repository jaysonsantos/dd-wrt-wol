use std::time::Duration;

use log::{debug, info};
use structopt::StructOpt;

mod wol;

use dd_wrt_wol_common::events::{Event, Response, Wakeup};

use crate::wol::Wol;

type OurResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(long, short, default_value = "10")]
    interval: u64,
    #[structopt(long, short = "u", default_value = "http://localhost:8089")]
    api_url: String,
    #[structopt(long, short = "n")]
    machine_name: String,
}

#[tokio::main]
async fn main() -> OurResult<()> {
    env_logger::init();
    let config = Config::from_args();
    let mut interval = tokio::time::interval(Duration::from_secs(config.interval));
    let base_url = format!(
        "{}/poll/machine/{}/since",
        config.api_url, config.machine_name
    );
    let mut last_entry: u64 = 0;

    loop {
        let url = format!("{}/{}", base_url, last_entry);
        debug!("Requesting {}", url);

        let response: Response = reqwest::get(&url).await?.json().await?;
        match response.event {
            Event::Ignore => debug!("Ignored"),
            Event::MachineNotFound => panic!("Machine not found {}", config.machine_name),
            Event::Wakeup(wakeup_data) => {
                wakeup(&wakeup_data).await?;
                last_entry = wakeup_data.time_of_occurrence;
            }
        }
        interval.tick().await;
    }
}

async fn wakeup(wakeup_data: &Wakeup) -> OurResult<()> {
    info!("Waking up {:?}", wakeup_data);
    let wol = Wol::from_str(&wakeup_data.mac_address)?;
    wol.send(format!("{}:9", wakeup_data.broadcast_ip)).await?;
    Ok(())
}
