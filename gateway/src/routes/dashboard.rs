use axum::response::{Html, IntoResponse};

const DASHBOARD_HTML: &str = include_str!("../assets/dashboard.html");

/// Operator dashboard backed by dashboard aggregation APIs.
pub async fn dashboard_page() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}
