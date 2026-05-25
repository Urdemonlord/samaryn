//! Shared application state.
//!
//! All shared state is held in `AppState` and wrapped in `Arc` for
//! thread-safe sharing across handlers.

use crate::config::AppConfig;
use crate::proxy::provider::ProviderRouter;
use crate::security::pii::PiiDetector;
use crate::security::rules::RulesEngine;
use crate::security::scanner::SecurityScanner;

/// Shared application state, accessible from all request handlers.
pub struct AppState {
    /// Application configuration.
    pub config: AppConfig,

    /// HTTP client for upstream requests (with connection pooling).
    pub http_client: reqwest::Client,

    /// PII detection and redaction engine.
    pub pii_detector: PiiDetector,

    /// ML service client for advanced security scanning.
    pub security_scanner: SecurityScanner,

    /// Routes models to the correct upstream provider.
    pub provider_router: ProviderRouter,

    /// YAML-based security rules engine.
    pub rules_engine: RulesEngine,
}
