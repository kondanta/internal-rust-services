use std::net::SocketAddr;

use axum::{
    routing::{get, post}, Router
};
use axum_prometheus::PrometheusMetricLayer;
use axum_server;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;

use clap::{Parser, Subcommand};

use lib::tracing as lib_tracing;

use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semcov;

mod http;
mod routes;
mod queue;

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
async fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    let cli = Cli::parse();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/", get(crate::routes::root))
        .route("/api/v1/command", post(crate::routes::command))
        .route("/healthz", get(crate::routes::healthz))
        .route("/readyz", get(crate::routes::readyz))
        .layer(prometheus_layer)
        .layer(OtelAxumLayer::default());

    let trace_resource = Resource::new(vec![
        KeyValue::new(semcov::resource::SERVICE_NAME, "collector")
        ]);
    let endpoint = std::env::var("OTLP_ENDPOINT").unwrap_or("http://localhost:4317".to_string());
    lib_tracing::init_tracing(trace_resource, endpoint);

    crate::queue::Queue::init().ok(); // create the queues

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));
    tracing::info!("listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

