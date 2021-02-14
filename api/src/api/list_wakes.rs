use crate::Request;

use tide::{Body, Result};

pub async fn list_wakes(request: Request) -> Result {
    let hosts = request.state().read().await;
    let name = request.param("name")?;
    if let Some(host) = hosts.get(name) {
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
