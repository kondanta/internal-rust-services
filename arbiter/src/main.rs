use std::collections::HashMap;
use std::net::SocketAddr;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use axum_server;
use color_eyre::Result;

use tracing_subscriber::FmtSubscriber;

use reqwest;

mod cli;
mod config;

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
        Some(cli::Command::Serve {
            config_file,
            host,
            port,
        }) => {
            tracing::info!("Starting arbiter at {host}:{port}...");
            serve(host, port, config_file).await?;
        }
        None => {
            tracing::error!(
                "No command provided. Use --help for usage information."
            );
        }
    }

    Ok(())
}

async fn serve(host: String, port: u16, config_file: String) -> Result<()> {
    render_config(config_file);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/healthz", get(|| async { "OK" }))
        .route("/readyz", get(|| async { "Ready" }))
        .route("/search", post(handle_search));

    // Convert String to Socket Addr
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn render_config(config_file: String) -> Result<config::Widgets> {
    config::Widgets::from_file(&config_file).map_err(|e| {
        color_eyre::eyre::eyre!(
            "Failed to read config file '{}': {}",
            config_file,
            e
        )
    })
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Request {
    tag: String,
    query: String,
}

async fn handle_search(
    Json(request): Json<Request>,
) -> impl axum::response::IntoResponse {
    tracing::debug!("Handling search request");

    let q = request.query;
    let e = request.tag;

    let mut kv = HashMap::new();
    kv.insert("g", "https://google.com/search?q=");
    kv.insert("yt", "https://www.youtube.com/results?search_query=");

    let mut url = "".to_string();

    // Search for a key
    if let Some(u) = kv.get(e.as_str()) {
        url = format!("{}{}", u, q);
    } else {
        println!("Key not found");
    }

    (StatusCode::OK, url)
}
