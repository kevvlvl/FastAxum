use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Domain {
    name: String,
    desc: String,
}

#[instrument(name = "get_health")]
pub async fn get_health() -> (StatusCode, Json<String>) {
    (StatusCode::OK, Json("Healthy!".parse().unwrap()))
}

#[instrument(name = "get_domains")]
pub async fn get_domains(Json(payload): Json<Domain>) -> (StatusCode, Json<Domain>) {

    info!("START call");

    let d = Domain {
        name: payload.name,
        desc: payload.desc,
    };

    info!("DONE call");

    (StatusCode::OK, Json(d))
}
