use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardEvent {
    pub timestamp: String,
    pub request_id: String,
    pub path: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    pub verdict: DashboardVerdict,
    #[serde(default)]
    pub pii_entities_redacted: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_action: Option<String>,
    #[serde(default)]
    pub threat_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DashboardVerdict {
    Allowed,
    PiiRedacted,
    BlockedRule,
    BlockedMl,
    Escalated,
    UpstreamError,
}
