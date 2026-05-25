//! PII (Personally Identifiable Information) detection and masking.
//!
//! Uses compiled regex patterns to detect and redact sensitive information
//! such as email addresses, phone numbers, API keys, JWTs, credit card
//! numbers, and bank account numbers.

use crate::models::security::PiiEntity;
use regex::{Regex, RegexSet};
use std::sync::OnceLock;

/// Global PII detector instance, initialized once.
static GLOBAL_PII_DETECTOR: OnceLock<PiiDetector> = OnceLock::new();

/// Get the global PII detector instance.
pub fn global_pii_detector() -> &'static PiiDetector {
    GLOBAL_PII_DETECTOR.get_or_init(PiiDetector::new)
}

/// PII entity type names, indexed to match the regex patterns.
const PII_TYPES: &[&str] = &[
    "EMAIL",
    "PHONE",
    "API_KEY",
    "JWT",
    "CREDIT_CARD",
    "BANK_ACCOUNT",
];

/// Regex patterns for each PII type.
const PII_PATTERNS: &[&str] = &[
    // EMAIL: standard email pattern
    r"[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}",
    // PHONE: US formats (xxx-xxx-xxxx, (xxx) xxx-xxxx, +1xxxxxxxxxx)
    // and Indonesian format (08xxxxxxxxxx)
    r"(?:\+?1[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}|08\d{8,12}",
    // API_KEY: OpenAI sk-* keys and AWS AKIA* keys
    r"(?:sk-[a-zA-Z0-9]{20,}|AKIA[A-Z0-9]{16,})",
    // JWT: three base64url-encoded segments separated by dots, starting with eyJ
    r"eyJ[a-zA-Z0-9_-]+\.eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+",
    // CREDIT_CARD: 13-19 digit numbers with optional separators
    r"\b(?:\d{4}[-\s]?){3,4}\d{1,4}\b",
    // BANK_ACCOUNT: 8-17 digit numbers (standalone)
    r"\b\d{8,17}\b",
];

/// Detects and redacts PII from text using compiled regex patterns.
pub struct PiiDetector {
    /// Fast check: does the text contain any PII?
    regex_set: RegexSet,

    /// Individual compiled patterns for extraction and position tracking.
    patterns: Vec<(String, Regex)>,
}

impl PiiDetector {
    /// Create a new PII detector with all compiled patterns.
    pub fn new() -> Self {
        let regex_set = RegexSet::new(PII_PATTERNS)
            .expect("Failed to compile PII regex set");

        let patterns: Vec<(String, Regex)> = PII_TYPES
            .iter()
            .zip(PII_PATTERNS.iter())
            .map(|(name, pattern)| {
                let re = Regex::new(pattern)
                    .unwrap_or_else(|e| panic!("Failed to compile pattern for {}: {}", name, e));
                (name.to_string(), re)
            })
            .collect();

        Self { regex_set, patterns }
    }

    /// Fast check: does the text contain any PII?
    ///
    /// Uses RegexSet for efficient multi-pattern matching without
    /// extracting positions or values.
    pub fn contains_pii(&self, text: &str) -> bool {
        self.regex_set.is_matching(text)
    }

    /// Detect all PII entities in the text with their types, values, and positions.
    ///
    /// Returns entities sorted by their start position. Overlapping matches
    /// from different patterns are all included (the caller can decide how
    /// to handle overlaps).
    pub fn detect(&self, text: &str) -> Vec<PiiEntity> {
        // First check which pattern groups matched (fast path)
        let matches: Vec<usize> = self.regex_set.matches(text).into_iter().collect();

        if matches.is_empty() {
            return Vec::new();
        }

        let mut entities = Vec::new();

        // Only iterate over the patterns that actually matched
        for &idx in &matches {
            let (ref entity_type, ref regex) = self.patterns[idx];
            for mat in regex.find_iter(text) {
                entities.push(PiiEntity {
                    entity_type: entity_type.clone(),
                    value: mat.as_str().to_string(),
                    start: mat.start(),
                    end: mat.end(),
                });
            }
        }

        // Sort by position for consistent output
        entities.sort_by_key(|e| e.start);
        entities
    }

    /// Redact all PII in the text, replacing each occurrence with a
    /// numbered placeholder like `[EMAIL_1]`, `[PHONE_1]`, etc.
    ///
    /// Returns the redacted text and the list of detected entities.
    pub fn redact(&self, text: &str) -> (String, Vec<PiiEntity>) {
        let entities = self.detect(text);

        if entities.is_empty() {
            return (text.to_string(), entities);
        }

        // Track counts per entity type for numbering (e.g., [EMAIL_1], [EMAIL_2])
        let mut type_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        // Build replacements sorted by position (already sorted by detect())
        let mut replacements: Vec<(usize, usize, String)> = Vec::new();
        for entity in &entities {
            let count = type_counts
                .entry(entity.entity_type.clone())
                .or_insert(0);
            *count += 1;
            let placeholder = format!("[{}_{}]", entity.entity_type, count);
            replacements.push((entity.start, entity.end, placeholder));
        }

        // Apply replacements from end to start to preserve positions
        let mut result = text.to_string();
        for (start, end, placeholder) in replacements.into_iter().rev() {
            // Ensure bounds are valid
            if start <= result.len() && end <= result.len() && start <= end {
                result.replace_range(start..end, &placeholder);
            }
        }

        (result, entities)
    }
}

impl Default for PiiDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_detection() {
        let detector = PiiDetector::new();
        let text = "Contact me at user@example.com for details.";
        assert!(detector.contains_pii(text));

        let entities = detector.detect(text);
        assert!(!entities.is_empty());
        assert_eq!(entities[0].entity_type, "EMAIL");
        assert_eq!(entities[0].value, "user@example.com");
    }

    #[test]
    fn test_phone_detection() {
        let detector = PiiDetector::new();
        let text = "Call me at 555-123-4567 or 08123456789.";
        let entities = detector.detect(text);

        let phone_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == "PHONE")
            .collect();
        assert!(phone_entities.len() >= 1);
    }

    #[test]
    fn test_api_key_detection() {
        let detector = PiiDetector::new();
        let text = "My API key is sk-abcdefghijklmnopqrstuvwxyz123456";
        assert!(detector.contains_pii(text));

        let entities = detector.detect(text);
        let api_keys: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == "API_KEY")
            .collect();
        assert!(!api_keys.is_empty());
    }

    #[test]
    fn test_redaction() {
        let detector = PiiDetector::new();
        let text = "Email: user@example.com and also admin@test.org";
        let (redacted, entities) = detector.redact(text);

        assert!(!redacted.contains("user@example.com"));
        assert!(!redacted.contains("admin@test.org"));
        assert!(redacted.contains("[EMAIL_1]"));
        assert!(redacted.contains("[EMAIL_2]"));
        assert_eq!(
            entities
                .iter()
                .filter(|e| e.entity_type == "EMAIL")
                .count(),
            2
        );
    }

    #[test]
    fn test_no_pii() {
        let detector = PiiDetector::new();
        let text = "Hello, how are you today?";
        assert!(!detector.contains_pii(text));
        assert!(detector.detect(text).is_empty());

        let (redacted, entities) = detector.redact(text);
        assert_eq!(redacted, text);
        assert!(entities.is_empty());
    }
}
