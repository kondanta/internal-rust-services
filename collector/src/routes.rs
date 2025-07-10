
use axum::{http::StatusCode, Json};
use axum_macros::debug_handler;

// todo(taylan): implement response struct
pub(crate) async fn healthz() -> impl axum::response::IntoResponse {
    tracing::info!("healthz");
    (StatusCode::OK, "ok")
}

// todo(taylan): implement response struct
pub(crate) async fn readyz() -> impl axum::response::IntoResponse {
    tracing::info!("readyz");
    (StatusCode::OK, "ok")
}

pub(crate) async fn root() -> &'static str {
    tracing::info!("root");
    "Hello, World!"
}

#[debug_handler]
pub(crate) async fn command(
    Json(request): Json<crate::http::Request>
) -> impl axum::response::IntoResponse {
    tracing::info!(event = "command", request = ?request, "command");
    let q = crate::queue::Queue::new();
    let _ = q.dispatch(request).unwrap();

    (StatusCode::OK, "ok")
}
