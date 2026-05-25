//! Configuration loading for the Samaryn Gateway.
//!
//! Supports loading from a YAML config file with environment variable overrides.
//! Config is searched for at `config/default.yaml` relative to the binary,
//! then `../config/default.yaml`, then falls back to sensible defaults.

use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use tracing::{info, warn};

/// Top-level application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Server binding configuration.
    #[serde(default)]
    pub server: ServerConfig,

    /// LLM provider configurations.
    #[serde(default)]
    pub providers: Vec<ProviderConfig>,

    /// ML scanning service configuration.
    #[serde(default)]
    pub ml_service: MlServiceConfig,

    /// Security policy configuration.
    #[serde(default)]
    pub security: SecurityConfig,

    /// Logging configuration.
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Optional API keys for authenticating gateway clients.
    #[serde(default)]
    pub auth_keys: Vec<String>,
}

/// Server binding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to (default: "0.0.0.0").
    #[serde(default = "default_host")]
    pub host: String,

    /// Port to bind to (default: 8080).
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

/// Configuration for an upstream LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Human-readable name for this provider (e.g., "openai", "anthropic").
    pub name: String,

    /// Base URL for the provider's API (e.g., "https://api.openai.com/v1").
    pub base_url: String,

    /// API key for authentication. Supports env var syntax: `${ENV_VAR_NAME}`.
    #[serde(default)]
    pub api_key: String,

    /// List of model names/prefixes this provider handles.
    #[serde(default)]
    pub models: Vec<String>,
}

/// Configuration for the ML scanning service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlServiceConfig {
    /// URL of the ML scanning service.
    #[serde(default = "default_ml_url")]
    pub url: String,

    /// Timeout in seconds for ML service requests.
    #[serde(default = "default_ml_timeout")]
    pub timeout_secs: u64,

    /// If true, allow requests through when the ML service is unavailable.
    /// If false (fail-closed), block requests when the ML service is down.
    #[serde(default = "default_fail_open")]
    pub fail_open: bool,
}

impl Default for MlServiceConfig {
    fn default() -> Self {
        Self {
            url: default_ml_url(),
            timeout_secs: default_ml_timeout(),
            fail_open: default_fail_open(),
        }
    }
}

fn default_ml_url() -> String {
    "http://localhost:8000".to_string()
}

fn default_ml_timeout() -> u64 {
    5
}

fn default_fail_open() -> bool {
    true
}

/// Security policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable PII masking in requests.
    #[serde(default = "default_true")]
    pub pii_masking: bool,

    /// Enable prompt injection detection.
    #[serde(default = "default_true")]
    pub injection_detection: bool,

    /// Action to take when a security violation is detected: "block" or "warn".
    #[serde(default = "default_action")]
    pub action: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            pii_masking: true,
            injection_detection: true,
            action: default_action(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_action() -> String {
    "block".to_string()
}

/// Logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log output format: "json" or "text".
    #[serde(default = "default_log_format")]
    pub format: String,

    /// Log level filter (e.g., "info", "debug", "warn").
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Optional path to an audit log file (JSONL format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_file: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            format: default_log_format(),
            level: default_log_level(),
            audit_file: None,
        }
    }
}

fn default_log_format() -> String {
    "json".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            providers: vec![ProviderConfig {
                name: "openai".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: "${OPENAI_API_KEY}".to_string(),
                models: vec![
                    "gpt-4o".to_string(),
                    "gpt-4o-mini".to_string(),
                    "gpt-4".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
            }],
            ml_service: MlServiceConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
            auth_keys: vec![],
        }
    }
}

/// Load configuration from YAML file with environment variable overrides.
///
/// Search order:
/// 1. `config/default.yaml` relative to the binary
/// 2. `../config/default.yaml` relative to the binary
/// 3. `config/default.yaml` relative to the current working directory
/// 4. Fall back to defaults
pub fn load_config() -> AppConfig {
    let config_paths = get_config_search_paths();

    let mut config = None;
    for path in &config_paths {
        if path.exists() {
            info!("Loading config from: {}", path.display());
            match std::fs::read_to_string(path) {
                Ok(contents) => match serde_yaml::from_str::<AppConfig>(&contents) {
                    Ok(cfg) => {
                        config = Some(cfg);
                        break;
                    }
                    Err(e) => {
                        warn!("Failed to parse config file {}: {}", path.display(), e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read config file {}: {}", path.display(), e);
                }
            }
        }
    }

    let mut config = config.unwrap_or_else(|| {
        info!("No config file found, using defaults");
        AppConfig::default()
    });

    // Apply environment variable overrides
    apply_env_overrides(&mut config);

    // Resolve API key env var references (e.g., "${OPENAI_API_KEY}")
    for provider in &mut config.providers {
        provider.api_key = resolve_env_var(&provider.api_key);
    }

    config
}

/// Build the list of paths to search for config files.
fn get_config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Relative to the binary location
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("config").join("default.yaml"));
            paths.push(exe_dir.join("..").join("config").join("default.yaml"));
        }
    }

    // Relative to the current working directory
    paths.push(PathBuf::from("config").join("default.yaml"));

    paths
}

/// Apply environment variable overrides using the `SAMARYN__` prefix.
///
/// Supported overrides:
/// - `SAMARYN__SERVER__HOST` → server.host
/// - `SAMARYN__SERVER__PORT` → server.port
/// - `SAMARYN__ML_SERVICE__URL` → ml_service.url
/// - `SAMARYN__ML_SERVICE__TIMEOUT_SECS` → ml_service.timeout_secs
/// - `SAMARYN__ML_SERVICE__FAIL_OPEN` → ml_service.fail_open
/// - `SAMARYN__SECURITY__PII_MASKING` → security.pii_masking
/// - `SAMARYN__SECURITY__INJECTION_DETECTION` → security.injection_detection
/// - `SAMARYN__SECURITY__ACTION` → security.action
/// - `SAMARYN__LOGGING__LEVEL` → logging.level
/// - `SAMARYN__LOGGING__FORMAT` → logging.format
fn apply_env_overrides(config: &mut AppConfig) {
    if let Ok(val) = env::var("SAMARYN__SERVER__HOST") {
        config.server.host = val;
    }
    if let Ok(val) = env::var("SAMARYN__SERVER__PORT") {
        if let Ok(port) = val.parse::<u16>() {
            config.server.port = port;
        }
    }
    if let Ok(val) = env::var("SAMARYN__ML_SERVICE__URL") {
        config.ml_service.url = val;
    }
    if let Ok(val) = env::var("SAMARYN__ML_SERVICE__TIMEOUT_SECS") {
        if let Ok(timeout) = val.parse::<u64>() {
            config.ml_service.timeout_secs = timeout;
        }
    }
    if let Ok(val) = env::var("SAMARYN__ML_SERVICE__FAIL_OPEN") {
        if let Ok(b) = val.parse::<bool>() {
            config.ml_service.fail_open = b;
        }
    }
    if let Ok(val) = env::var("SAMARYN__SECURITY__PII_MASKING") {
        if let Ok(b) = val.parse::<bool>() {
            config.security.pii_masking = b;
        }
    }
    if let Ok(val) = env::var("SAMARYN__SECURITY__INJECTION_DETECTION") {
        if let Ok(b) = val.parse::<bool>() {
            config.security.injection_detection = b;
        }
    }
    if let Ok(val) = env::var("SAMARYN__SECURITY__ACTION") {
        config.security.action = val;
    }
    if let Ok(val) = env::var("SAMARYN__LOGGING__LEVEL") {
        config.logging.level = val;
    }
    if let Ok(val) = env::var("SAMARYN__LOGGING__FORMAT") {
        config.logging.format = val;
    }
}

/// Resolve environment variable references in a string.
///
/// If the string matches `${VAR_NAME}`, the value of `VAR_NAME` is returned.
/// Otherwise, the original string is returned unchanged.
fn resolve_env_var(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with("${") && trimmed.ends_with('}') {
        let var_name = &trimmed[2..trimmed.len() - 1];
        match env::var(var_name) {
            Ok(val) => val,
            Err(_) => {
                warn!(
                    "Environment variable '{}' not set, using empty string",
                    var_name
                );
                String::new()
            }
        }
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert!(config.security.pii_masking);
        assert!(config.security.injection_detection);
    }

    #[test]
    fn test_resolve_env_var_passthrough() {
        assert_eq!(resolve_env_var("plain-value"), "plain-value");
    }

    #[test]
    fn test_resolve_env_var_with_env() {
        env::set_var("TEST_SAMARYN_KEY", "test-api-key-123");
        assert_eq!(
            resolve_env_var("${TEST_SAMARYN_KEY}"),
            "test-api-key-123"
        );
        env::remove_var("TEST_SAMARYN_KEY");
    }
}
