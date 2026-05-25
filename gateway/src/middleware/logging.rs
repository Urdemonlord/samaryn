//! Audit logging middleware.
//!
//! Logs request metadata (method, path, request_id) and response
//! information (status code, latency) both via tracing and optionally
//! to a JSONL audit file.

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use tracing::info;

/// Audit logging middleware.
///
/// Captures request metadata before forwarding, then logs the response
/// status and latency after the handler completes.
pub async fn audit_logging(
    request: Request<Body>,
    next: Next,
    audit_file: Option<String>,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let timestamp = Utc::now().to_rfc3339();

    info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        timestamp = %timestamp,
        "Incoming request"
    );

    // Execute the actual handler
    let response = next.run(request).await;

    let latency_ms = start.elapsed().as_millis();
    let status = response.status().as_u16();

    info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = status,
        latency_ms = latency_ms,
        "Request completed"
    );

    // Write to audit file if configured
    if let Some(ref audit_path) = audit_file {
        let audit_entry = serde_json::json!({
            "timestamp": timestamp,
            "request_id": request_id,
            "method": method.as_str(),
            "path": path,
            "status": status,
            "latency_ms": latency_ms,
        });

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(audit_path)
        {
            let _ = writeln!(file, "{}", audit_entry);
        }
    }

    response
}
