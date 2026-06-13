use axum::response::{Html, IntoResponse};

const LANDING_HTML: &str = include_str!("../assets/landing.html");

/// Public landing page for Samaryn.
pub async fn landing_page() -> impl IntoResponse {
    Html(LANDING_HTML)
}
