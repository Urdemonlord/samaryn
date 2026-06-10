use axum::response::{Html, IntoResponse};

const DEMO_HTML: &str = include_str!("../assets/demo.html");

/// Interactive demo page for trying the gateway with real requests.
pub async fn demo_page() -> impl IntoResponse {
    Html(DEMO_HTML)
}
