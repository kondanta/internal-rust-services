use std::net::SocketAddr;

use axum::{Router, routing::get};
use axum_server;
use color_eyre::Result;

use tracing_subscriber::FmtSubscriber;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = cli::Cli::parse();

    // Determine log level from -v count or --log-level
    let log_level = if let Some(level) = &cli.log_level {
        level.clone()
    } else {
        match cli.verbose {
            0 => "error".to_string(),
            1 => "warning".to_string(),
            2 => "info".to_string(),
            3 => "debug".to_string(),
            _ => "trace".to_string(),
        }
    };

    // Set up tracing subscriber
    let subscriber = FmtSubscriber::builder().with_env_filter(log_level).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    match cli.command {
        Some(cli::Command::Serve { host, port }) => {
            tracing::info!("Starting arbiter at {host}:{port}...");
            serve(host, port).await?;
        }
        None => {
            tracing::error!(
                "No command provided. Use --help for usage information."
            );
        }
    }

    Ok(())
}

async fn serve(host: String, port: u16) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/healthz", get(|| async { "OK" }))
        .route("/readyz", get(|| async { "Ready" }));

    // Convert String to Socket Addr
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
