use dd_wrt_wol_common::events::{Event, Response, Wakeup};
use tide::{Body, Error, Result, StatusCode};

use crate::Request;

pub async fn poll(request: Request) -> Result {
    let hosts = request.state().read().await;
    let machine_name = request.param("name")?;
    let since = request
        .param("time")?
        .parse()
        .map_err(|err| Error::new(StatusCode::BadRequest, err))?;
    let response = if let Some(host) = hosts.get(machine_name) {
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
