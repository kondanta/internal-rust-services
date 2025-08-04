use std::net::SocketAddr;

use axum::{Router, routing::get};
use axum_server;
use color_eyre::Result;

use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Set up tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    tracing::info!("Starting Spinner...");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/healthz", get(|| async { "OK" }))
        .route("/readyz", get(|| async { "Ready" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
