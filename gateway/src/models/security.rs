//! Security-related types for communication with the ML scanning service.

use serde::{Deserialize, Serialize};

/// Request to the ML scanning service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    /// The text content to scan.
    pub text: String,

    /// Types of scans to perform (e.g., "injection", "pii").
    pub scan_types: Vec<String>,
}

/// Response from the ML scanning service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponse {
    /// Whether the content is considered safe.
    pub is_safe: bool,

    /// Prompt injection detection results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injection: Option<InjectionResult>,

    /// PII detection results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<PiiResult>,
}

/// Results from prompt injection detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionResult {
    /// Whether a prompt injection was detected.
    pub detected: bool,

    /// List of detected threat descriptions.
    pub threats: Vec<String>,

    /// Severity level (e.g., "low", "medium", "high", "critical").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

/// Results from PII detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiResult {
    /// Whether PII was detected.
    pub detected: bool,

    /// List of detected PII entities.
    pub entities: Vec<PiiEntity>,

    /// The text with PII replaced by placeholders.
    pub masked_text: String,
}

/// A single detected PII entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiEntity {
    /// The type of PII (e.g., "EMAIL", "PHONE", "CREDIT_CARD").
    pub entity_type: String,

    /// The actual PII value found.
    pub value: String,

    /// Start position (byte offset) in the original text.
    pub start: usize,

    /// End position (byte offset) in the original text.
    pub end: usize,
}
