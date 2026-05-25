//! Samaryn Gateway — AI Security Gateway
//!
//! An OpenAI-compatible reverse proxy that scans requests for PII and
//! prompt injection before forwarding to LLM providers.

mod config;
mod error;
mod middleware;
mod models;
mod proxy;
mod routes;
mod security;
mod state;

use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderName;
use axum::middleware as axum_middleware;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::proxy::provider::ProviderRouter;
use crate::security::pii::PiiDetector;
use crate::security::rules::RulesEngine;
use crate::security::scanner::SecurityScanner;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // ─── Initialize tracing ───────────────────────────────────────────
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .json()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // ─── Print startup banner ─────────────────────────────────────────
    print_banner();

    // ─── Load configuration ──────────────────────────────────────────
    let config = config::load_config();

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);

    info!(
        host = %config.server.host,
        port = config.server.port,
        providers = config.providers.len(),
        pii_masking = config.security.pii_masking,
        injection_detection = config.security.injection_detection,
        "Configuration loaded"
    );

    // ─── Build shared state ──────────────────────────────────────────
    let http_client = reqwest::Client::builder()
        .pool_max_idle_per_host(20)
        .pool_idle_timeout(Duration::from_secs(90))
        .timeout(Duration::from_secs(120))
        .build()
        .expect("Failed to create HTTP client");

    let pii_detector = PiiDetector::new();
    let security_scanner = SecurityScanner::new(
        config.ml_service.url.clone(),
        config.ml_service.timeout_secs,
        config.ml_service.fail_open,
    );
    let provider_router = ProviderRouter::new(config.providers.clone());
    let rules_engine = RulesEngine::default_rules();

    let auth_keys = config.auth_keys.clone();
    let audit_file = config.logging.audit_file.clone();

    let state = Arc::new(AppState {
        config,
        http_client,
        pii_detector,
        security_scanner,
        provider_router,
        rules_engine,
    });

    // ─── Build router ────────────────────────────────────────────────
    let x_request_id = HeaderName::from_static("x-request-id");

    let app = Router::new()
        // API routes
        .route(
            "/v1/chat/completions",
            post(routes::chat::chat_completions),
        )
        .route("/health", get(routes::health::health_check))
        // Fallback 404
        .fallback(fallback_handler)
        // Shared state
        .with_state(state)
        // Middleware layers (applied bottom-to-top, so order matters)
        .layer(
            ServiceBuilder::new()
                // Request ID tracking
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MakeRequestUuid,
                ))
                .layer(PropagateRequestIdLayer::new(x_request_id))
                // HTTP tracing
                .layer(TraceLayer::new_for_http())
                // Request timeout (120 seconds)
                .layer(TimeoutLayer::new(Duration::from_secs(120)))
                // CORS (permissive for development)
                .layer(CorsLayer::permissive()),
        )
        // Auth middleware
        .layer(axum_middleware::from_fn(move |req, next| {
            let keys = auth_keys.clone();
            async move {
                crate::middleware::auth::auth_middleware(req, next, keys).await
            }
        }))
        // Audit logging middleware
        .layer(axum_middleware::from_fn(move |req, next| {
            let file = audit_file.clone();
            async move {
                crate::middleware::logging::audit_logging(req, next, file).await
            }
        }));

    // ─── Start server ────────────────────────────────────────────────
    info!(bind_addr = %bind_addr, "Starting Samaryn Gateway");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", bind_addr, e));

    info!("🚀 Samaryn Gateway listening on {}", bind_addr);

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e));

    info!("Samaryn Gateway shut down gracefully");
}

/// Fallback handler for unmatched routes.
async fn fallback_handler() -> (axum::http::StatusCode, Json<serde_json::Value>) {
    (
        axum::http::StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": {
                "message": "Not found. Available endpoints: POST /v1/chat/completions, GET /health",
                "type": "not_found_error",
                "code": "404"
            }
        })),
    )
}

/// Listen for shutdown signals (Ctrl+C).
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    info!("Shutdown signal received, starting graceful shutdown...");
}

/// Print the startup ASCII art banner.
fn print_banner() {
    let banner = r#"
  ____                                        
 / ___|  __ _ _ __ ___   __ _ _ __ _   _ _ __  
 \___ \ / _` | '_ ` _ \ / _` | '__| | | | '_ \ 
  ___) | (_| | | | | | | (_| | |  | |_| | | | |
 |____/ \__,_|_| |_| |_|\__,_|_|   \__, |_| |_|
                                     |___/       
    ╔══════════════════════════════════════╗
    ║     AI Security Gateway v0.1.0      ║
    ║   OpenAI-Compatible Reverse Proxy   ║
    ╚══════════════════════════════════════╝
"#;
    println!("{}", banner);
}
