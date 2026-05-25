//! Health check endpoint.

use axum::response::IntoResponse;
use axum::Json;

/// Simple health check handler.
///
/// Returns a JSON response indicating the service is healthy,
/// along with the service name and version.
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "samaryn-gateway",
        "version": "0.1.0"
    }))
}
