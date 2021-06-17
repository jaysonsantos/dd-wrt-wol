use std::time::SystemTime;

use crate::Request;

use tide::{Body, Result, StatusCode};
use tracing::{info, info_span, instrument, Instrument};

#[instrument(skip(request))]
pub async fn wake(request: Request) -> Result {
    let mut hosts = request
        .state()
        .write()
        .instrument(info_span!("state_lock"))
        .await;
    let machine_name = request.param("name")?;

    let response = if let Some(host) = hosts.get_mut(machine_name) {
        host.entries.push(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let mut response = tide::Response::new(StatusCode::Accepted);
        info!(message = "Waking up machine", %machine_name);
        response.set_body(Body::from_string(format!("Waking up {}", machine_name)));

        response
    } else {
        let mut response = tide::Response::new(StatusCode::NotFound);

        response.set_body(Body::from_string(format!(
            "Machine {} not found",
            machine_name
        )));

        response
    };

    Ok(response)
}
