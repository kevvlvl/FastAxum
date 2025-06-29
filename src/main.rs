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
        .route("/", get(|| async {
            "Healthy!"
        }))
        .route("/domains", post(logic::domains::get_domains));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
