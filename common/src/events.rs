use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub event: Event,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "ignore")]
    Ignore,
    #[serde(rename = "machine_not_found")]
    MachineNotFound,
    #[serde(rename = "wakeup")]
    Wakeup(Wakeup),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wakeup {
    pub mac_address: String,
    pub broadcast_ip: String,
    pub time_of_occurrence: u64,
}
