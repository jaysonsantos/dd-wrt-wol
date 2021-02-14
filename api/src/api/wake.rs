use std::time::SystemTime;

use crate::Request;

use tide::{Body, Result, StatusCode};

pub async fn wake(request: Request) -> Result {
    let mut hosts = request.state().write().await;
    let machine_name = request.param("name")?;

    let response = if let Some(host) = hosts.get_mut(machine_name) {
        host.entries.push(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let mut response = tide::Response::new(StatusCode::Accepted);
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
