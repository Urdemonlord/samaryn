use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::dashboard_aggregate::{
    build_models, build_recent_events, build_security, build_summary, build_traffic,
    filter_recent_chat_events, load_dashboard_events,
};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct DashboardEventsQuery {
    pub limit: Option<usize>,
}

pub async fn dashboard_summary(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let events = load_dashboard_events(state.dashboard_event_file.as_deref());
    let recent_events = filter_recent_chat_events(&events, 24);
    let ml_health = fetch_ml_health(&state).await;

    Json(build_summary(
        &recent_events,
        "healthy",
        "samaryn-gateway",
        env!("CARGO_PKG_VERSION"),
        &ml_health,
        !state.config.ml_service.url.trim().is_empty(),
        state.config.ml_service.fail_open,
    ))
}

pub async fn dashboard_traffic(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let events = load_dashboard_events(state.dashboard_event_file.as_deref());
    let recent_events = filter_recent_chat_events(&events, 24);
    Json(build_traffic(&recent_events, 60))
}

pub async fn dashboard_models(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let events = load_dashboard_events(state.dashboard_event_file.as_deref());
    let recent_events = filter_recent_chat_events(&events, 24);
    Json(build_models(&recent_events))
}

pub async fn dashboard_security(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let events = load_dashboard_events(state.dashboard_event_file.as_deref());
    let recent_events = filter_recent_chat_events(&events, 24);
    Json(build_security(&recent_events))
}

pub async fn dashboard_events(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DashboardEventsQuery>,
) -> impl IntoResponse {
    let events = load_dashboard_events(state.dashboard_event_file.as_deref());
    let recent_events = filter_recent_chat_events(&events, 24);
    Json(build_recent_events(&recent_events, query.limit.unwrap_or(20).min(100)))
}

async fn fetch_ml_health(state: &AppState) -> String {
    let url = format!("{}/health", state.config.ml_service.url.trim_end_matches('/'));
    match state.http_client.get(url).send().await {
        Ok(response) if response.status().is_success() => "healthy".to_string(),
        Ok(response) => format!("http_{}", response.status().as_u16()),
        Err(_) => {
            if state.config.ml_service.fail_open {
                "unreachable_fail_open".to_string()
            } else {
                "unreachable".to_string()
            }
        }
    }
}
