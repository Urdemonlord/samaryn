//! Optional API key authentication middleware.
//!
//! If `auth_keys` is configured and non-empty, validates that incoming
//! requests include a valid `Authorization: Bearer <key>` header.
//! If no auth keys are configured, all requests pass through.

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::warn;

/// Authentication middleware that checks for a valid API key.
///
/// This is designed to be used with `axum::middleware::from_fn_with_state`.
/// If `auth_keys` is empty, all requests are allowed through.
pub async fn auth_middleware(
    request: Request<Body>,
    next: Next,
    auth_keys: Vec<String>,
) -> Response {
    // If no auth keys configured, pass through all requests
    if auth_keys.is_empty() {
        return next.run(request).await;
    }

    // Skip auth for health check endpoint
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }

    // Extract and validate the Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok());

    match auth_header {
        Some(header_value) => {
            // Expect "Bearer <key>" format
            if let Some(key) = header_value.strip_prefix("Bearer ") {
                if auth_keys.contains(&key.to_string()) {
                    next.run(request).await
                } else {
                    warn!("Invalid API key provided");
                    unauthorized_response("Invalid API key")
                }
            } else {
                warn!("Malformed Authorization header (expected Bearer token)");
                unauthorized_response("Invalid authorization format. Expected: Bearer <key>")
            }
        }
        None => {
            warn!("Missing Authorization header");
            unauthorized_response("Missing API key. Provide via Authorization: Bearer <key>")
        }
    }
}

/// Build a 401 Unauthorized response in OpenAI error format.
fn unauthorized_response(message: &str) -> Response {
    let body = json!({
        "error": {
            "message": message,
            "type": "authentication_error",
            "code": "401"
        }
    });

    (StatusCode::UNAUTHORIZED, Json(body)).into_response()
}
