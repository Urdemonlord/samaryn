//! Chat completions proxy handler - the core of the Samaryn Gateway.
//!
//! This handler intercepts OpenAI-compatible chat completion requests,
//! scans them for security threats (PII, prompt injection), optionally
//! redacts sensitive content, and forwards the (possibly modified)
//! request to the appropriate upstream LLM provider.

use std::sync::Arc;
use std::time::Instant;

use axum::extract::State;
use axum::response::Response;
use axum::Json;
use chrono::Utc;
use tracing::{debug, info, warn};

use crate::dashboard_events::append_dashboard_event;
use crate::error::GatewayError;
use crate::models::dashboard::{DashboardEvent, DashboardVerdict};
use crate::models::openai::ChatCompletionRequest;
use crate::proxy::streaming;
use crate::security::rules::RuleResult;
use crate::state::AppState;

fn normalize_model_for_provider(provider_name: &str, model: &str) -> String {
    if provider_name == "openrouter" {
        if model == "openrouter/free" || model.ends_with(":free") {
            return model.to_string();
        }

        return "openrouter/free".to_string();
    }

    if provider_name == "meowlabs" {
        if let Some(stripped) = model.strip_prefix("meowlabs/") {
            return stripped.to_string();
        }
    }

    if provider_name == "gemini" {
        if let Some(stripped) = model.strip_prefix("gemini/") {
            return stripped.to_string();
        }
    }

    if model.contains('/') {
        return model.to_string();
    }

    model.to_string()
}

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
    let request_started = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    let model = payload.model.clone();
    let is_streaming = payload.stream.unwrap_or(false);
    let mut pii_entities_redacted = 0u32;

    info!(
        request_id = %request_id,
        model = %model,
        stream = is_streaming,
        message_count = payload.messages.len(),
        "Processing chat completion request"
    );

    let combined_content = extract_message_content(&payload);

    if state.config.security.injection_detection {
        match state.rules_engine.evaluate(&combined_content) {
            RuleResult::Blocked(reason) => {
                warn!(
                    request_id = %request_id,
                    reason = %reason,
                    "Request blocked by security rules"
                );
                write_dashboard_event(
                    &state,
                    build_dashboard_event(
                        &request_id,
                        "/v1/chat/completions",
                        "POST",
                        Some(model.clone()),
                        None,
                        Some(403),
                        Some(elapsed_ms(request_started)),
                        DashboardVerdict::BlockedRule,
                        pii_entities_redacted,
                        Some(reason.clone()),
                        None,
                        None,
                        vec!["prompt_injection".to_string()],
                    ),
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

    if state.config.security.pii_masking {
        let pii_detector = &state.pii_detector;

        for message in &mut payload.messages {
            if let Some(content) = &message.content {
                if let Some(text) = content.as_str() {
                    if pii_detector.contains_pii(text) {
                        let (redacted, entities) = pii_detector.redact(text);
                        pii_entities_redacted += entities.len() as u32;

                        if !entities.is_empty() {
                            info!(
                                request_id = %request_id,
                                entity_count = entities.len(),
                                entity_types = ?entities.iter().map(|e| &e.entity_type).collect::<Vec<_>>(),
                                "PII detected and redacted"
                            );
                        }

                        message.content = Some(serde_json::Value::String(redacted));
                    }
                } else if content.is_array() {
                    if let Some(arr) = content.as_array() {
                        let mut new_arr = arr.clone();
                        for part in &mut new_arr {
                            if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                if pii_detector.contains_pii(text) {
                                    let (redacted, entities) = pii_detector.redact(text);
                                    pii_entities_redacted += entities.len() as u32;
                                    if let Some(obj) = part.as_object_mut() {
                                        obj.insert(
                                            "text".to_string(),
                                            serde_json::Value::String(redacted),
                                        );
                                    }
                                }
                            }
                        }
                        message.content = Some(serde_json::Value::Array(new_arr));
                    }
                }
            }
        }

        if pii_entities_redacted > 0 {
            info!(
                request_id = %request_id,
                total_pii_entities = pii_entities_redacted,
                "Total PII entities redacted in request"
            );
        }
    }

    if state.config.security.injection_detection {
        let scan_types = vec!["injection".to_string()];
        match state.security_scanner.scan(&combined_content, scan_types).await {
            Ok(scan_result) => {
                if !scan_result.is_safe {
                    if let Some(classification) = &scan_result.classification {
                        if classification.action == "escalate" {
                            warn!(
                                request_id = %request_id,
                                label = %classification.label,
                                confidence = classification.confidence,
                                source = %classification.source,
                                "ML service flagged out-of-domain prompt"
                            );
                            write_dashboard_event(
                                &state,
                                build_dashboard_event(
                                    &request_id,
                                    "/v1/chat/completions",
                                    "POST",
                                    Some(model.clone()),
                                    None,
                                    Some(403),
                                    Some(elapsed_ms(request_started)),
                                    DashboardVerdict::Escalated,
                                    pii_entities_redacted,
                                    None,
                                    Some(classification.label.clone()),
                                    Some(classification.action.clone()),
                                    vec!["out_of_domain".to_string()],
                                ),
                            );

                            return Err(GatewayError::SecurityViolation {
                                reason: "Out-of-domain prompt requires escalation".to_string(),
                                threats: vec!["out_of_domain".to_string()],
                            });
                        }

                        if classification.action == "block" {
                            let threats = scan_result
                                .injection
                                .as_ref()
                                .map(|injection| injection.threats.clone())
                                .filter(|threats| !threats.is_empty())
                                .unwrap_or_else(|| vec!["prompt_injection".to_string()]);
                            warn!(
                                request_id = %request_id,
                                threats = ?threats,
                                label = %classification.label,
                                confidence = classification.confidence,
                                "ML service detected prompt injection"
                            );
                            write_dashboard_event(
                                &state,
                                build_dashboard_event(
                                    &request_id,
                                    "/v1/chat/completions",
                                    "POST",
                                    Some(model.clone()),
                                    None,
                                    Some(403),
                                    Some(elapsed_ms(request_started)),
                                    DashboardVerdict::BlockedMl,
                                    pii_entities_redacted,
                                    None,
                                    Some(classification.label.clone()),
                                    Some(classification.action.clone()),
                                    threats.clone(),
                                ),
                            );

                            return Err(GatewayError::SecurityViolation {
                                reason: "Prompt injection detected by ML scanner".to_string(),
                                threats,
                            });
                        }
                    }

                    if let Some(injection) = &scan_result.injection {
                        if injection.detected {
                            warn!(
                                request_id = %request_id,
                                threats = ?injection.threats,
                                severity = ?injection.severity,
                                "ML service detected prompt injection"
                            );
                        }
                    }
                }
            }
            Err(error) => {
                debug!(
                    request_id = %request_id,
                    error = %error,
                    "ML service scan returned error (may be handled by fail-open)"
                );
                return Err(error);
            }
        }
    }

    let provider = state.provider_router.resolve(&model)?;
    let provider_name = provider.name.clone();
    let upstream_model = normalize_model_for_provider(&provider_name, &model);
    if upstream_model != model {
        payload.model = upstream_model.clone();
    }

    let upstream_url = format!("{}/chat/completions", provider.base_url);

    info!(
        request_id = %request_id,
        provider = %provider_name,
        upstream_model = %upstream_model,
        upstream_url = %upstream_url,
        "Forwarding request to upstream provider"
    );

    let upstream_response = match state
        .http_client
        .post(&upstream_url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            warn!(
                request_id = %request_id,
                error = %error,
                "Failed to forward request to upstream"
            );
            write_dashboard_event(
                &state,
                build_dashboard_event(
                    &request_id,
                    "/v1/chat/completions",
                    "POST",
                    Some(model.clone()),
                    Some(provider_name.clone()),
                    Some(502),
                    Some(elapsed_ms(request_started)),
                    DashboardVerdict::UpstreamError,
                    pii_entities_redacted,
                    None,
                    None,
                    None,
                    vec![],
                ),
            );
            return Err(GatewayError::from(error));
        }
    };

    let upstream_status = upstream_response.status();
    info!(
        request_id = %request_id,
        upstream_status = %upstream_status,
        "Received upstream response"
    );

    let verdict = if upstream_status.is_success() {
        if pii_entities_redacted > 0 {
            DashboardVerdict::PiiRedacted
        } else {
            DashboardVerdict::Allowed
        }
    } else {
        DashboardVerdict::UpstreamError
    };

    write_dashboard_event(
        &state,
        build_dashboard_event(
            &request_id,
            "/v1/chat/completions",
            "POST",
            Some(model),
            Some(provider_name),
            Some(upstream_status.as_u16()),
            Some(elapsed_ms(request_started)),
            verdict,
            pii_entities_redacted,
            None,
            None,
            None,
            vec![],
        ),
    );

    if is_streaming && upstream_status.is_success() {
        Ok(streaming::forward_streaming(upstream_response))
    } else {
        streaming::forward_non_streaming(upstream_response).await
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u64::MAX as u128) as u64
}

fn build_dashboard_event(
    request_id: &str,
    path: &str,
    method: &str,
    model: Option<String>,
    provider: Option<String>,
    status: Option<u16>,
    latency_ms: Option<u64>,
    verdict: DashboardVerdict,
    pii_entities_redacted: u32,
    rule_reason: Option<String>,
    ml_label: Option<String>,
    ml_action: Option<String>,
    threat_types: Vec<String>,
) -> DashboardEvent {
    DashboardEvent {
        timestamp: Utc::now().to_rfc3339(),
        request_id: request_id.to_string(),
        path: path.to_string(),
        method: method.to_string(),
        model,
        provider,
        status,
        latency_ms,
        verdict,
        pii_entities_redacted,
        rule_reason,
        ml_label,
        ml_action,
        threat_types,
    }
}

fn write_dashboard_event(state: &AppState, event: DashboardEvent) {
    if let Some(path) = state.dashboard_event_file.as_deref() {
        append_dashboard_event(path, &event);
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
