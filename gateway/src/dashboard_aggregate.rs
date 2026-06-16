use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::Serialize;

use crate::models::dashboard::{DashboardEvent, DashboardVerdict};

#[derive(Debug, Clone, Serialize)]
pub struct DashboardSummaryResponse {
    pub window: String,
    pub requests_total: usize,
    pub success_rate: f64,
    pub blocked_total: usize,
    pub avg_latency_ms: u64,
    pub active_models: usize,
    pub gateway_health: String,
    pub service: String,
    pub version: String,
    pub ml_service: MlServiceSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct MlServiceSummary {
    pub configured: bool,
    pub fail_open: bool,
    pub health: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardTrafficResponse {
    pub window: String,
    pub bucket_minutes: u32,
    pub points: Vec<TrafficPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrafficPoint {
    pub timestamp: String,
    pub requests: usize,
    pub allowed: usize,
    pub blocked: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardModelsResponse {
    pub items: Vec<ModelUsageItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelUsageItem {
    pub model: String,
    pub requests: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardSecurityResponse {
    pub blocked_rule: usize,
    pub blocked_ml: usize,
    pub escalated: usize,
    pub pii_redacted: usize,
    pub top_threats: Vec<NamedCount>,
    pub top_rule_reasons: Vec<ReasonCount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NamedCount {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReasonCount {
    pub reason: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardEventsResponse {
    pub items: Vec<DashboardEventListItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardEventListItem {
    pub timestamp: String,
    pub request_id: String,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub verdict: DashboardVerdict,
    pub status: Option<u16>,
    pub latency_ms: Option<u64>,
    pub reason: Option<String>,
    pub threat_types: Vec<String>,
}

pub fn load_dashboard_events(path: Option<&str>) -> Vec<DashboardEvent> {
    let Some(path) = path else {
        return Vec::new();
    };

    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };

    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<DashboardEvent>(line).ok())
        .collect()
}

pub fn filter_recent_chat_events(events: &[DashboardEvent], hours: i64) -> Vec<DashboardEvent> {
    let cutoff = Utc::now() - Duration::hours(hours);

    let mut filtered: Vec<DashboardEvent> = events
        .iter()
        .filter(|event| event.path == "/v1/chat/completions")
        .filter(|event| parse_timestamp(&event.timestamp).is_some_and(|ts| ts >= cutoff))
        .cloned()
        .collect();

    filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    filtered
}

pub fn build_summary(
    events: &[DashboardEvent],
    gateway_health: &str,
    service: &str,
    version: &str,
    ml_health: &str,
    ml_configured: bool,
    ml_fail_open: bool,
) -> DashboardSummaryResponse {
    let requests_total = events.len();
    let allowed_like = events
        .iter()
        .filter(|event| {
            matches!(
                event.verdict,
                DashboardVerdict::Allowed | DashboardVerdict::PiiRedacted
            )
        })
        .count();
    let blocked_total = events
        .iter()
        .filter(|event| {
            matches!(
                event.verdict,
                DashboardVerdict::BlockedRule
                    | DashboardVerdict::BlockedMl
                    | DashboardVerdict::Escalated
            )
        })
        .count();
    let success_rate = if requests_total == 0 {
        0.0
    } else {
        ((allowed_like as f64 / requests_total as f64) * 1000.0).round() / 10.0
    };

    let latency_samples: Vec<u64> = events.iter().filter_map(|event| event.latency_ms).collect();
    let avg_latency_ms = if latency_samples.is_empty() {
        0
    } else {
        latency_samples.iter().sum::<u64>() / latency_samples.len() as u64
    };

    let active_models = events
        .iter()
        .filter_map(|event| event.model.as_ref())
        .collect::<HashSet<_>>()
        .len();

    DashboardSummaryResponse {
        window: "24h".to_string(),
        requests_total,
        success_rate,
        blocked_total,
        avg_latency_ms,
        active_models,
        gateway_health: gateway_health.to_string(),
        service: service.to_string(),
        version: version.to_string(),
        ml_service: MlServiceSummary {
            configured: ml_configured,
            fail_open: ml_fail_open,
            health: ml_health.to_string(),
        },
    }
}

pub fn build_traffic(events: &[DashboardEvent], bucket_minutes: u32) -> DashboardTrafficResponse {
    let mut buckets: BTreeMap<String, TrafficPoint> = BTreeMap::new();

    for event in events {
        let Some(timestamp) = parse_timestamp(&event.timestamp) else {
            continue;
        };

        let bucket = truncate_to_bucket(timestamp, bucket_minutes);
        let key = bucket.to_rfc3339();
        let entry = buckets.entry(key.clone()).or_insert(TrafficPoint {
            timestamp: key,
            requests: 0,
            allowed: 0,
            blocked: 0,
        });

        entry.requests += 1;
        match event.verdict {
            DashboardVerdict::Allowed | DashboardVerdict::PiiRedacted => entry.allowed += 1,
            DashboardVerdict::BlockedRule
            | DashboardVerdict::BlockedMl
            | DashboardVerdict::Escalated => entry.blocked += 1,
            DashboardVerdict::UpstreamError => {}
        }
    }

    DashboardTrafficResponse {
        window: "24h".to_string(),
        bucket_minutes,
        points: buckets.into_values().collect(),
    }
}

pub fn build_models(events: &[DashboardEvent]) -> DashboardModelsResponse {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for event in events {
        if let Some(model) = &event.model {
            *counts.entry(model.clone()).or_default() += 1;
        }
    }

    let mut items: Vec<ModelUsageItem> = counts
        .into_iter()
        .map(|(model, requests)| ModelUsageItem { model, requests })
        .collect();
    items.sort_by(|a, b| b.requests.cmp(&a.requests).then_with(|| a.model.cmp(&b.model)));

    DashboardModelsResponse { items }
}

pub fn build_security(events: &[DashboardEvent]) -> DashboardSecurityResponse {
    let mut blocked_rule = 0;
    let mut blocked_ml = 0;
    let mut escalated = 0;
    let mut pii_redacted = 0;
    let mut threat_counts: HashMap<String, usize> = HashMap::new();
    let mut rule_reason_counts: HashMap<String, usize> = HashMap::new();

    for event in events {
        match event.verdict {
            DashboardVerdict::BlockedRule => blocked_rule += 1,
            DashboardVerdict::BlockedMl => blocked_ml += 1,
            DashboardVerdict::Escalated => escalated += 1,
            DashboardVerdict::PiiRedacted => pii_redacted += 1,
            DashboardVerdict::Allowed | DashboardVerdict::UpstreamError => {}
        }

        for threat in &event.threat_types {
            *threat_counts.entry(threat.clone()).or_default() += 1;
        }

        if let Some(reason) = &event.rule_reason {
            *rule_reason_counts.entry(reason.clone()).or_default() += 1;
        }
    }

    let mut top_threats: Vec<NamedCount> = threat_counts
        .into_iter()
        .map(|(name, count)| NamedCount { name, count })
        .collect();
    top_threats.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
    top_threats.truncate(5);

    let mut top_rule_reasons: Vec<ReasonCount> = rule_reason_counts
        .into_iter()
        .map(|(reason, count)| ReasonCount { reason, count })
        .collect();
    top_rule_reasons.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.reason.cmp(&b.reason)));
    top_rule_reasons.truncate(5);

    DashboardSecurityResponse {
        blocked_rule,
        blocked_ml,
        escalated,
        pii_redacted,
        top_threats,
        top_rule_reasons,
    }
}

pub fn build_recent_events(events: &[DashboardEvent], limit: usize) -> DashboardEventsResponse {
    let items = events
        .iter()
        .take(limit)
        .map(|event| DashboardEventListItem {
            timestamp: event.timestamp.clone(),
            request_id: event.request_id.clone(),
            model: event.model.clone(),
            provider: event.provider.clone(),
            verdict: event.verdict.clone(),
            status: event.status,
            latency_ms: event.latency_ms,
            reason: event.rule_reason.clone().or(event.ml_label.clone()),
            threat_types: event.threat_types.clone(),
        })
        .collect();

    DashboardEventsResponse { items }
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn truncate_to_bucket(timestamp: DateTime<Utc>, bucket_minutes: u32) -> DateTime<Utc> {
    let minute = timestamp.minute();
    let rounded_minute = minute - (minute % bucket_minutes.max(1));
    timestamp
        .with_minute(rounded_minute)
        .and_then(|value| value.with_second(0))
        .and_then(|value| value.with_nanosecond(0))
        .unwrap_or(timestamp)
}
