// They have better implementation: https://docs.shuttle.rs/examples/axum-jwt-authentication
use std::env;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// implement IntoResponse for AuthError so we can use it as an Axum response type
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for crate::auth::Claims
where
    S: Send + Sync,
{
    type Rejection = crate::auth::AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| crate::auth::AuthError::MissingToken)?;
        tracing::debug!("JWT_SECRET: {}", jwt_secret);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| crate::auth::AuthError::InvalidToken)?;

        let token_data = bearer.token();
        if token_data != jwt_secret {
            return Err(crate::auth::AuthError::InvalidToken);
        }

        Ok(Self{token: token_data.to_string()})
    }
}


// the JWT claim
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub token: String,
}

// error types for auth errors
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}
