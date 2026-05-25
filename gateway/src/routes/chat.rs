//! Chat completions proxy handler — the core of the Samaryn Gateway.
//!
//! This handler intercepts OpenAI-compatible chat completion requests,
//! scans them for security threats (PII, prompt injection), optionally
//! redacts sensitive content, and forwards the (possibly modified)
//! request to the appropriate upstream LLM provider.

use std::sync::Arc;

use axum::extract::State;
use axum::response::Response;
use axum::Json;
use tracing::{debug, info, warn};

use crate::error::GatewayError;
use crate::models::openai::ChatCompletionRequest;
use crate::proxy::streaming;
use crate::security::rules::RuleResult;
use crate::state::AppState;

/// Main chat completions proxy handler.
///
/// Pipeline:
/// 1. Extract all text content from the request messages
/// 2. Evaluate security rules (block if needed)
/// 3. Detect and redact PII in message content
/// 4. Optionally call the ML service for advanced scanning
/// 5. Resolve the upstream provider for the requested model
/// 6. Forward the (possibly modified) request upstream
/// 7. Return the response (streaming or non-streaming)
pub async fn chat_completions(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<ChatCompletionRequest>,
) -> Result<Response, GatewayError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let model = payload.model.clone();
    let is_streaming = payload.stream.unwrap_or(false);

    info!(
        request_id = %request_id,
        model = %model,
        stream = is_streaming,
        message_count = payload.messages.len(),
        "Processing chat completion request"
    );

    // Step 1: Extract all text content from messages for security scanning
    let combined_content = extract_message_content(&payload);

    // Step 2: Evaluate security rules
    if state.config.security.injection_detection {
        match state.rules_engine.evaluate(&combined_content) {
            RuleResult::Blocked(reason) => {
                warn!(
                    request_id = %request_id,
                    reason = %reason,
                    "Request blocked by security rules"
                );
                return Err(GatewayError::SecurityViolation {
                    reason,
                    threats: vec!["prompt_injection".to_string()],
                });
            }
            RuleResult::Allowed => {
                debug!(request_id = %request_id, "Security rules check passed");
            }
        }
    }

    // Step 3: Detect and redact PII in message content
    if state.config.security.pii_masking {
        let pii_detector = &state.pii_detector;
        let mut total_pii_found = 0;

        for message in &mut payload.messages {
            if let Some(content) = &message.content {
                if let Some(text) = content.as_str() {
                    if pii_detector.contains_pii(text) {
                        let (redacted, entities) = pii_detector.redact(text);
                        total_pii_found += entities.len();

                        if !entities.is_empty() {
                            info!(
                                request_id = %request_id,
                                entity_count = entities.len(),
                                entity_types = ?entities.iter().map(|e| &e.entity_type).collect::<Vec<_>>(),
                                "PII detected and redacted"
                            );
                        }

                        message.content =
                            Some(serde_json::Value::String(redacted));
                    }
                } else if content.is_array() {
                    // Handle array content (multi-modal messages)
                    if let Some(arr) = content.as_array() {
                        let mut new_arr = arr.clone();
                        for part in &mut new_arr {
                            if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                if pii_detector.contains_pii(text) {
                                    let (redacted, entities) = pii_detector.redact(text);
                                    total_pii_found += entities.len();
                                    if let Some(obj) = part.as_object_mut() {
                                        obj.insert(
                                            "text".to_string(),
                                            serde_json::Value::String(redacted),
                                        );
                                    }
                                }
                            }
                        }
                        message.content =
                            Some(serde_json::Value::Array(new_arr));
                    }
                }
            }
        }

        if total_pii_found > 0 {
            info!(
                request_id = %request_id,
                total_pii_entities = total_pii_found,
                "Total PII entities redacted in request"
            );
        }
    }

    // Step 4: Optionally call ML service for advanced scanning
    if state.config.security.injection_detection {
        let scan_types = vec!["injection".to_string()];
        match state
            .security_scanner
            .scan(&combined_content, scan_types)
            .await
        {
            Ok(scan_result) => {
                if !scan_result.is_safe {
                    if let Some(injection) = &scan_result.injection {
                        if injection.detected {
                            warn!(
                                request_id = %request_id,
                                threats = ?injection.threats,
                                severity = ?injection.severity,
                                "ML service detected prompt injection"
                            );

                            if state.config.security.action == "block" {
                                return Err(GatewayError::SecurityViolation {
                                    reason: "Prompt injection detected by ML scanner"
                                        .to_string(),
                                    threats: injection.threats.clone(),
                                });
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // Error is already logged inside the scanner
                debug!(
                    request_id = %request_id,
                    error = %e,
                    "ML service scan returned error (may be handled by fail-open)"
                );
                // If fail-open, we already got a safe response, so this
                // error only surfaces when fail-closed
                return Err(e);
            }
        }
    }

    // Step 5: Resolve upstream provider
    let (base_url, api_key) = state.provider_router.resolve(&model)?;

    let upstream_url = format!("{}/chat/completions", base_url);

    info!(
        request_id = %request_id,
        upstream_url = %upstream_url,
        "Forwarding request to upstream provider"
    );

    // Step 6: Build and send the upstream request
    let upstream_response = state
        .http_client
        .post(&upstream_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            warn!(
                request_id = %request_id,
                error = %e,
                "Failed to forward request to upstream"
            );
            GatewayError::from(e)
        })?;

    let upstream_status = upstream_response.status();
    info!(
        request_id = %request_id,
        upstream_status = %upstream_status,
        "Received upstream response"
    );

    // Step 7: Forward the response
    if is_streaming && upstream_status.is_success() {
        Ok(streaming::forward_streaming(upstream_response))
    } else {
        streaming::forward_non_streaming(upstream_response).await
    }
}

/// Extract all text content from the messages in a chat completion request.
///
/// Handles both string content and array content (multi-modal messages).
/// Returns a single combined string for security scanning.
fn extract_message_content(payload: &ChatCompletionRequest) -> String {
    let mut parts: Vec<String> = Vec::new();

    for message in &payload.messages {
        if let Some(content) = &message.content {
            if let Some(text) = content.as_str() {
                parts.push(text.to_string());
            } else if let Some(arr) = content.as_array() {
                // Handle array content (e.g., multi-modal messages)
                for part in arr {
                    if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                        parts.push(text.to_string());
                    }
                }
            }
        }
    }

    parts.join("\n")
}
