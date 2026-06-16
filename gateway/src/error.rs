//! Error types for the Samaryn Gateway.
//!
//! All errors are converted to OpenAI-compatible JSON error responses
//! so that clients receive a consistent error format regardless of the
//! failure source.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Central error type for the gateway.
#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    /// Error communicating with the upstream LLM provider.
    #[error("Upstream error: {0}")]
    Upstream(String),

    /// A security policy violation was detected.
    #[error("Security violation: {reason}")]
    SecurityViolation {
        reason: String,
        threats: Vec<String>,
    },

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// The requested model matched a provider route, but no matching provider
    /// has active credentials in the current runtime.
    #[error("Provider unavailable: {0}")]
    ProviderUnavailable(String),

    /// Error communicating with the ML scanning service.
    #[error("ML service error: {0}")]
    MlService(String),

    /// Invalid or malformed client request.
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            GatewayError::Upstream(msg) => {
                (StatusCode::BAD_GATEWAY, "upstream_error", msg.clone())
            }
            GatewayError::SecurityViolation { reason, threats } => {
                let detail = if threats.is_empty() {
                    reason.clone()
                } else {
                    format!("{}: {}", reason, threats.join(", "))
                };
                (StatusCode::FORBIDDEN, "security_violation", detail)
            }
            GatewayError::Config(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "config_error", msg.clone())
            }
            GatewayError::ProviderUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "provider_unavailable",
                msg.clone(),
            ),
            GatewayError::MlService(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, "ml_service_error", msg.clone())
            }
            GatewayError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "invalid_request_error", msg.clone())
            }
        };

        let body = json!({
            "error": {
                "message": message,
                "type": error_type,
                "code": status.as_u16().to_string(),
            }
        });

        (status, Json(body)).into_response()
    }
}

impl From<reqwest::Error> for GatewayError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            GatewayError::Upstream(format!("Request timed out: {}", err))
        } else if err.is_connect() {
            GatewayError::Upstream(format!("Connection failed: {}", err))
        } else {
            GatewayError::Upstream(format!("HTTP error: {}", err))
        }
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        GatewayError::BadRequest(format!("JSON error: {}", err))
    }
}
