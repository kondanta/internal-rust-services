use std::{env, net::SocketAddr};
use axum::{
    routing::put,
    Router
};

use axum_server;
use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use lib::tracing as lib_tracing;

use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semcov;


mod auth;
mod http;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
}

#[tokio::main]
async fn main() -> Result<()>{
    color_eyre::install()?;
    let cli = Cli::parse();

    if env::var("JWT_SECRET").is_err() {
        return Err(eyre!("JWT_SECRET is not set"));
    }

    let app = Router::new()
        .route("/api/v1/command", put(crate::http::command))
        .layer(
            tower::ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(|err: axum::BoxError| async move {
                    (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", err),
                )
            }))
            .layer(tower::buffer::BufferLayer::new(100))
            .layer(tower::limit::RateLimitLayer::new(1, std::time::Duration::from_secs(10))),
        );

    let trace_resource = Resource::new(vec![
        KeyValue::new(semcov::resource::SERVICE_NAME, "quard"),
    ]);
    let endpoint = std::env::var("OTLP_ENDPOINT").unwrap_or("http://localhost:4317".to_string());
    lib_tracing::init_tracing(trace_resource, endpoint);

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));
    tracing::info!("listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Do heavy rate limiting if it is not a certain requestor :)
// Check the bearer token if the requestor is the right one