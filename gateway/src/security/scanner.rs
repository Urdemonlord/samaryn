//! ML service HTTP client for advanced security scanning.
//!
//! Communicates with an external ML service (e.g., the Samaryn Python scanner)
//! to perform prompt injection detection and advanced PII analysis.

use crate::error::GatewayError;
use crate::models::security::{ScanRequest, ScanResponse};
use std::time::Duration;
use tracing::{info, warn};

/// Client for the ML security scanning service.
#[derive(Debug, Clone)]
pub struct SecurityScanner {
    /// HTTP client for making requests.
    client: reqwest::Client,

    /// Base URL of the ML scanning service.
    ml_service_url: String,

    /// Request timeout.
    timeout: Duration,

    /// If true, allow requests through when the ML service is unavailable.
    fail_open: bool,
}

impl SecurityScanner {
    /// Create a new security scanner client.
    pub fn new(ml_service_url: String, timeout_secs: u64, fail_open: bool) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create HTTP client for security scanner");

        Self {
            client,
            ml_service_url,
            timeout: Duration::from_secs(timeout_secs),
            fail_open,
        }
    }

    /// Scan text content for security threats.
    ///
    /// Sends the text to the ML service for analysis. If the service is
    /// unavailable and `fail_open` is true, returns a safe response.
    /// If `fail_open` is false, returns an error.
    pub async fn scan(
        &self,
        text: &str,
        scan_types: Vec<String>,
    ) -> Result<ScanResponse, GatewayError> {
        let request = ScanRequest {
            text: text.to_string(),
            scan_types,
        };

        let url = format!("{}/api/v1/scan", self.ml_service_url);

        info!(
            url = %url,
            text_length = text.len(),
            "Sending scan request to ML service"
        );

        match self
            .client
            .post(&url)
            .timeout(self.timeout)
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<ScanResponse>().await {
                        Ok(scan_response) => Ok(scan_response),
                        Err(e) => {
                            warn!("Failed to parse ML service response: {}", e);
                            if self.fail_open {
                                warn!("Fail-open: treating unparseable response as safe");
                                Ok(self.safe_response())
                            } else {
                                Err(GatewayError::MlService(format!(
                                    "Failed to parse scan response: {}",
                                    e
                                )))
                            }
                        }
                    }
                } else {
                    let status = response.status();
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "unknown".to_string());
                    warn!(
                        status = %status,
                        body = %body,
                        "ML service returned error"
                    );

                    if self.fail_open {
                        warn!("Fail-open: treating ML service error as safe");
                        Ok(self.safe_response())
                    } else {
                        Err(GatewayError::MlService(format!(
                            "ML service returned {}: {}",
                            status, body
                        )))
                    }
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    warn!("ML service request timed out: {}", e);
                } else if e.is_connect() {
                    warn!("Failed to connect to ML service: {}", e);
                } else {
                    warn!("ML service request failed: {}", e);
                }

                if self.fail_open {
                    warn!("Fail-open: treating ML service failure as safe");
                    Ok(self.safe_response())
                } else {
                    Err(GatewayError::MlService(format!(
                        "ML service unavailable: {}",
                        e
                    )))
                }
            }
        }
    }

    /// Returns a safe (all-clear) scan response for fail-open scenarios.
    fn safe_response(&self) -> ScanResponse {
        ScanResponse {
            is_safe: true,
            injection: None,
            pii: None,
            classification: None,
        }
    }
}
