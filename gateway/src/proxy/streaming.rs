//! SSE stream forwarding and non-streaming response forwarding.
//!
//! Handles both streaming (SSE) and non-streaming responses from
//! upstream LLM providers, forwarding them transparently to the client.

use axum::body::Body;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;

use crate::error::GatewayError;

/// Forward a streaming SSE response from the upstream provider.
///
/// Converts the upstream reqwest response into an Axum SSE response,
/// preserving the stream semantics. Sets appropriate headers for
/// Server-Sent Events.
pub fn forward_streaming(upstream_response: reqwest::Response) -> Response {
    let stream = upstream_response.bytes_stream();
    let body = Body::from_stream(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, HeaderValue::from_static("text/event-stream"))
        .header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
        .header(header::CONNECTION, HeaderValue::from_static("keep-alive"))
        .header("X-Accel-Buffering", HeaderValue::from_static("no"))
        .body(body)
        .unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to build streaming response"))
                .unwrap()
        })
}

/// Forward a non-streaming response from the upstream provider.
///
/// Reads the full response body and forwards it with the original
/// status code and relevant headers.
pub async fn forward_non_streaming(
    upstream_response: reqwest::Response,
) -> Result<Response, GatewayError> {
    let status = upstream_response.status();
    let content_type = upstream_response
        .headers()
        .get(header::CONTENT_TYPE)
        .cloned();

    let body_bytes = upstream_response
        .bytes()
        .await
        .map_err(|e| GatewayError::Upstream(format!("Failed to read upstream response: {}", e)))?;

    let mut builder = Response::builder().status(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY));

    if let Some(ct) = content_type {
        builder = builder.header(header::CONTENT_TYPE, ct);
    } else {
        builder = builder.header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
    }

    builder
        .body(Body::from(body_bytes))
        .map_err(|e| GatewayError::Upstream(format!("Failed to build response: {}", e)))
}
