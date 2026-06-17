use axum::response::{Html, IntoResponse};

const DOCS_HTML: &str = include_str!("../assets/docs.html");

/// Integration documentation for Samaryn API.
pub async fn docs_page() -> impl IntoResponse {
    Html(DOCS_HTML)
}
