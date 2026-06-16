//! Provider routing for upstream LLM services.
//!
//! Routes requests to the correct LLM provider based on the model name
//! specified in the request.

use crate::config::ProviderConfig;
use crate::error::GatewayError;
use tracing::debug;

/// Routes requests to the appropriate upstream LLM provider.
#[derive(Debug, Clone)]
pub struct ProviderRouter {
    /// List of configured providers.
    providers: Vec<ProviderConfig>,
}

impl ProviderRouter {
    /// Create a new provider router with the given provider configurations.
    pub fn new(providers: Vec<ProviderConfig>) -> Self {
        Self { providers }
    }

    /// Resolve which provider should handle a request for the given model.
    ///
    /// Returns the matching provider configuration.
    ///
    /// Matching logic:
    /// 1. Check each provider's `models` list for an exact match.
    /// 2. Check each provider's `models` list for a prefix match (e.g., "gpt-" matches "gpt-4o").
    /// 3. If the model matched one or more providers but all matched providers are
    ///    unavailable, return an explicit error instead of silently rerouting.
    /// 4. If no match found, default to the first configured provider.
    pub fn resolve(&self, model: &str) -> Result<&ProviderConfig, GatewayError> {
        if self.providers.is_empty() {
            return Err(GatewayError::Config(
                "No providers configured".to_string(),
            ));
        }

        let mut unavailable_matches: Vec<String> = Vec::new();

        // First pass: exact match
        for provider in &self.providers {
            for supported_model in &provider.models {
                if supported_model == model {
                    if !provider_is_available(provider) {
                        unavailable_matches.push(provider.name.clone());
                        debug!(
                            model = %model,
                            provider = %provider.name,
                            "Exact model match skipped because provider is unavailable"
                        );
                        continue;
                    }
                    debug!(
                        model = %model,
                        provider = %provider.name,
                        "Exact model match found"
                    );
                    return Ok(provider);
                }
            }
        }

        // Second pass: prefix match (e.g., model "gpt-4o-2024-05-13" matches pattern "gpt-4o")
        for provider in &self.providers {
            for supported_model in &provider.models {
                if model.starts_with(supported_model.as_str()) {
                    if !provider_is_available(provider) {
                        if !unavailable_matches.contains(&provider.name) {
                            unavailable_matches.push(provider.name.clone());
                        }
                        debug!(
                            model = %model,
                            provider = %provider.name,
                            pattern = %supported_model,
                            "Prefix model match skipped because provider is unavailable"
                        );
                        continue;
                    }
                    debug!(
                        model = %model,
                        provider = %provider.name,
                        pattern = %supported_model,
                        "Prefix model match found"
                    );
                    return Ok(provider);
                }
            }
        }

        if !unavailable_matches.is_empty() {
            let providers = unavailable_matches.join(", ");
            return Err(GatewayError::ProviderUnavailable(format!(
                "No active credentials for providers matching model '{}': {}",
                model, providers
            )));
        }

        // Default to the first available provider (for custom/local models)
        if let Some(default_provider) = self
            .providers
            .iter()
            .find(|provider| provider_is_available(provider))
        {
            debug!(
                model = %model,
                provider = %default_provider.name,
                "No model match found, using first available provider"
            );
            return Ok(default_provider);
        }

        let default_provider = &self.providers[0];
        debug!(
            model = %model,
            provider = %default_provider.name,
            "No available providers found, falling back to first configured provider"
        );
        Ok(default_provider)
    }
}

fn provider_is_available(provider: &ProviderConfig) -> bool {
    if !provider.api_key.trim().is_empty() {
        return true;
    }

    provider.base_url.starts_with("http://localhost")
        || provider.base_url.starts_with("http://127.0.0.1")
        || provider.base_url.starts_with("http://ollama")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_providers() -> Vec<ProviderConfig> {
        vec![
            ProviderConfig {
                name: "openai".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: "sk-test-openai".to_string(),
                models: vec![
                    "gpt-4o".to_string(),
                    "gpt-4o-mini".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
            },
            ProviderConfig {
                name: "anthropic".to_string(),
                base_url: "https://api.anthropic.com/v1".to_string(),
                api_key: "sk-test-anthropic".to_string(),
                models: vec![
                    "claude-3".to_string(),
                    "claude-2".to_string(),
                ],
            },
        ]
    }

    #[test]
    fn test_exact_match() {
        let router = ProviderRouter::new(test_providers());
        let provider = router.resolve("gpt-4o").unwrap();
        assert_eq!(provider.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn test_prefix_match() {
        let router = ProviderRouter::new(test_providers());
        let provider = router.resolve("claude-3-sonnet").unwrap();
        assert_eq!(provider.base_url, "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_default_fallback() {
        let router = ProviderRouter::new(test_providers());
        let provider = router.resolve("some-unknown-model").unwrap();
        assert_eq!(provider.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn test_empty_providers() {
        let router = ProviderRouter::new(vec![]);
        assert!(router.resolve("gpt-4o").is_err());
    }

    #[test]
    fn test_matching_provider_without_credentials_returns_error() {
        let router = ProviderRouter::new(vec![
            ProviderConfig {
                name: "meowlabs".to_string(),
                base_url: "https://api.meowlabs.store/v1".to_string(),
                api_key: "sk-test-meowlabs".to_string(),
                models: vec!["kr/auto".to_string()],
            },
            ProviderConfig {
                name: "anthropic".to_string(),
                base_url: "https://api.anthropic.com".to_string(),
                api_key: String::new(),
                models: vec!["claude-sonnet-4-20250514".to_string()],
            },
            ProviderConfig {
                name: "openrouter".to_string(),
                base_url: "https://openrouter.ai/api/v1".to_string(),
                api_key: String::new(),
                models: vec!["claude-".to_string()],
            },
        ]);

        let err = router.resolve("claude-sonnet-4-20250514").unwrap_err();
        match err {
            GatewayError::ProviderUnavailable(message) => {
                assert!(message.contains("claude-sonnet-4-20250514"));
                assert!(message.contains("anthropic"));
                assert!(message.contains("openrouter"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn test_unknown_model_still_falls_back_to_first_available_provider() {
        let router = ProviderRouter::new(vec![
            ProviderConfig {
                name: "meowlabs".to_string(),
                base_url: "https://api.meowlabs.store/v1".to_string(),
                api_key: "sk-test-meowlabs".to_string(),
                models: vec!["kr/auto".to_string()],
            },
            ProviderConfig {
                name: "openrouter".to_string(),
                base_url: "https://openrouter.ai/api/v1".to_string(),
                api_key: String::new(),
                models: vec!["claude-".to_string()],
            },
        ]);

        let provider = router.resolve("some-unknown-model").unwrap();
        assert_eq!(provider.name, "meowlabs");
    }
}
