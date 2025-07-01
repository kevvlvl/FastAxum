use std::env;
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tracing::info;
use crate::trace::otel::init_tracing_subscriber;

mod logic;
mod trace;

#[tokio::main]
async fn main() {

    let _guard = init_tracing_subscriber();

    info!("STARTING Fast Axum - Domains API");

    let router = Router::new() 
        .route("/", get(logic::domains::get_health))
        .route("/domains", post(logic::domains::get_domains));

    let addr = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("{}:{}", addr, port)).await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
