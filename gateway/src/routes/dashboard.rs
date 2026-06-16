use axum::response::{Html, IntoResponse};

const DASHBOARD_HTML: &str = include_str!("../assets/dashboard.html");

/// Honest MVP dashboard for the current v0.1 runtime.
pub async fn dashboard_page() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}
