# Samaryn Operator Dashboard v1 Implementation Plan

> **For Hermes:** implement in order: event schema -> event capture -> aggregator API -> dashboard UI -> verification.

**Goal:** Replace the current dashboard status page with a real operator dashboard for Samaryn Gateway.

**Architecture:** Keep v1 VPS-first and minimal. Use gateway-side event logging as the source of truth, normalize those events into a structured JSONL stream, then add read-only aggregator endpoints that drive the dashboard UI. Avoid fake metrics. If data is missing, show operator-style empty states instead of marketing copy.

**Tech Stack:** Rust, Axum, existing gateway middleware/handlers, server-rendered HTML + client-side fetch, JSONL audit/event files.

---

## Current codebase facts

### Existing runtime surfaces
- `gateway/src/routes/health.rs`
  - returns `status`, `service`, `version`
- `gateway/src/middleware/logging.rs`
  - already writes JSONL audit lines with:
    - `timestamp`
    - `request_id`
    - `method`
    - `path`
    - `status`
    - `latency_ms`
- `gateway/src/routes/chat.rs`
  - already computes enough decision points for dashboarding:
    - request_id
    - model
    - allow/block
    - blocked by rules
    - PII redaction count
    - ML scanner flags
    - provider routing
- `gateway/audit.jsonl`
  - current sample lines show request-level audit JSONL exists already
- `gateway/src/routes/dashboard.rs`
  - currently only serves static HTML
- `gateway/src/assets/dashboard.html`
  - currently still behaves as a dressed-up status page, not an operator console

### Current gap
What is missing is **not frontend styling only**. The actual missing layer is:
1. structured security/event capture
2. aggregation API
3. dashboard UI bound to real data

---

## Product target for `/dashboard`

The page must become an **operator + security runtime console**.

### Required sections
1. KPI strip
   - Requests 24h
   - Success rate
   - Blocked requests
   - Avg latency
   - Active models
   - Gateway health
2. Traffic section
   - requests over time
   - success vs blocked over time
3. Model usage section
   - request count by model
4. Security section
   - blocked by rules
   - prompt injection hits
   - PII redactions
   - escalations / ML blocks
5. Recent events section
   - recent request/security events table
6. Runtime health section
   - gateway status
   - version
   - ML scanner status
   - fail-open mode

---

## Phase 1: Event schema

### Objective
Define one normalized event schema that the dashboard can rely on, without tying UI directly to ad-hoc logs.

### Files
- Create: `gateway/src/models/dashboard.rs`
- Modify: `gateway/src/models/mod.rs`
- Modify: `gateway/src/error.rs` only if needed for serialization helpers

### New types

#### `DashboardEvent`
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardEvent {
    pub timestamp: String,
    pub request_id: String,
    pub path: String,
    pub method: String,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub status: Option<u16>,
    pub latency_ms: Option<u128>,
    pub verdict: DashboardVerdict,
    pub pii_entities_redacted: u32,
    pub rule_reason: Option<String>,
    pub ml_label: Option<String>,
    pub ml_action: Option<String>,
    pub threat_types: Vec<String>,
}
```

#### `DashboardVerdict`
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DashboardVerdict {
    Allowed,
    PiiRedacted,
    BlockedRule,
    BlockedMl,
    Escalated,
    UpstreamError,
}
```

### Rules
- One event = one processed chat-completion request outcome.
- `GET /health` and `GET /dashboard` stay in audit logs but do **not** pollute security event metrics.
- Dashboard aggregation should mostly read `/v1/chat/completions` events.

### Output file format
Store as JSONL, one serialized `DashboardEvent` per line.

### Proposed file path for events
- `gateway/dashboard-events.jsonl`

### Why this phase exists
Without this schema, the UI either:
- fakes metrics, or
- scrapes incomplete logs, or
- hardcodes product claims

All are wrong.

---

## Phase 2: Event capture in gateway runtime

### Objective
Emit real `DashboardEvent` lines from the chat request pipeline.

### Files
- Modify: `gateway/src/state.rs`
- Modify: `gateway/src/config.rs`
- Modify: `gateway/src/main.rs`
- Modify: `gateway/src/routes/chat.rs`
- Create: `gateway/src/dashboard_events.rs`

### Config addition
Add a dedicated dashboard event file path in logging config.

```rust
pub struct LoggingConfig {
    pub format: String,
    pub level: String,
    pub audit_file: Option<String>,
    pub dashboard_event_file: Option<String>,
}
```

Default recommendation:
- `audit_file`: keep existing optional generic audit log
- `dashboard_event_file`: optional but strongly recommended for dashboarding

### AppState addition
```rust
pub struct AppState {
    pub config: AppConfig,
    pub http_client: reqwest::Client,
    pub pii_detector: PiiDetector,
    pub security_scanner: SecurityScanner,
    pub provider_router: ProviderRouter,
    pub rules_engine: RulesEngine,
    pub dashboard_event_file: Option<String>,
}
```

### New helper module
Create `gateway/src/dashboard_events.rs` with helpers:
- `append_dashboard_event(path: &str, event: &DashboardEvent)`
- best-effort append, no panic on write failure
- reuse `OpenOptions::new().create(true).append(true)` pattern already used in middleware logging

### Capture points inside `chat.rs`
Add event writes at each terminal decision point:

#### Allowed request
After upstream response returns successfully:
- verdict = `allowed` or `pii_redacted`
- include:
  - model
  - provider
  - request_id
  - latency if available
  - redaction count

#### Rule block
Inside:
```rust
RuleResult::Blocked(reason)
```
Write event:
- verdict = `blocked_rule`
- rule_reason = reason
- threat_types = `["prompt_injection"]`

#### ML escalation
Inside:
```rust
classification.action == "escalate"
```
Write event:
- verdict = `escalated`
- ml_label
- ml_action

#### ML block
Inside:
```rust
classification.action == "block"
```
Write event:
- verdict = `blocked_ml`
- threats
- ml_label
- ml_action

#### Upstream error
If upstream provider call fails:
- verdict = `upstream_error`
- include model/provider/status if available

### Important implementation detail
Right now latency is only measured in middleware. For v1 there are 2 acceptable options:

#### Option A: duplicate timing in `chat.rs`
Start an `Instant` near the top of `chat_completions()` and use elapsed time for event records.

#### Option B: keep event latency optional
Use `latency_ms: None` for first pass and rely on audit log for summary stats.

### Recommendation
Use **Option A**. It makes event records self-contained and avoids joining two files for the main dashboard queries.

---

## Phase 3: Aggregator API

### Objective
Expose read-only JSON endpoints that turn raw dashboard events into operator-friendly summaries.

### Files
- Create: `gateway/src/routes/dashboard_api.rs`
- Modify: `gateway/src/routes/mod.rs`
- Modify: `gateway/src/main.rs`
- Create: `gateway/src/dashboard_aggregate.rs`

### New API routes

#### `GET /api/dashboard/summary`
Returns:
```json
{
  "window": "24h",
  "requests_total": 123,
  "success_rate": 97.6,
  "blocked_total": 3,
  "avg_latency_ms": 412,
  "active_models": 4,
  "gateway_health": "healthy",
  "service": "samaryn-gateway",
  "version": "0.1.0",
  "ml_service": {
    "configured": true,
    "fail_open": true
  }
}
```

#### `GET /api/dashboard/traffic`
Returns bucketed recent timeline:
```json
{
  "window": "24h",
  "bucket_minutes": 60,
  "points": [
    {
      "timestamp": "2026-06-16T13:00:00Z",
      "requests": 12,
      "allowed": 11,
      "blocked": 1
    }
  ]
}
```

#### `GET /api/dashboard/models`
Returns:
```json
{
  "items": [
    {"model": "openrouter/free", "requests": 77},
    {"model": "gemini-2.5-flash", "requests": 31}
  ]
}
```

#### `GET /api/dashboard/security`
Returns:
```json
{
  "blocked_rule": 2,
  "blocked_ml": 1,
  "escalated": 1,
  "pii_redacted": 7,
  "top_threats": [
    {"name": "prompt_injection", "count": 3},
    {"name": "out_of_domain", "count": 1}
  ],
  "top_rule_reasons": [
    {"reason": "ignore previous instructions", "count": 2}
  ]
}
```

#### `GET /api/dashboard/events?limit=20`
Returns recent events:
```json
{
  "items": [
    {
      "timestamp": "2026-06-16T13:05:39Z",
      "request_id": "...",
      "model": "openrouter/free",
      "verdict": "allowed",
      "status": 200,
      "latency_ms": 532,
      "reason": null,
      "threat_types": []
    }
  ]
}
```

### Aggregation rules
- default window: last 24h
- ignore non-chat events for traffic/model/security panels
- `success_rate = allowed_like / total_chat_requests * 100`
- `allowed_like` includes:
  - `allowed`
  - `pii_redacted`
- `blocked_total` includes:
  - `blocked_rule`
  - `blocked_ml`
  - `escalated` if product wants escalation counted as blocked

### Product decision for escalation
Recommended for v1:
- count `escalated` as **blocked from auto-pass**, but display separately in security summary

### Parsing strategy
Create parser/reader in `dashboard_aggregate.rs`:
- load JSONL lines
- deserialize `DashboardEvent`
- filter by time window
- sort descending by timestamp for recent events
- aggregate maps for model counts, threat counts, verdict counts

### Keep v1 simple
- no DB yet
- no pagination beyond `limit`
- no heavy filtering yet

---

## Phase 4: Dashboard UI rewrite

### Objective
Replace landing-like HTML with a metric-first operator dashboard UI backed by the new API.

### Files
- Modify: `gateway/src/assets/dashboard.html`
- Keep: `gateway/src/routes/dashboard.rs` as static asset server

### UI structure

#### Section A: KPI strip
6 compact cards:
- Requests 24h
- Success Rate
- Blocked
- Avg Latency
- Active Models
- Gateway Health

#### Section B: Main charts row
- left: traffic chart
- right: model usage chart/list

For v1, avoid pulling a heavyweight JS chart library. Use either:
- simple SVG bars/lines generated in JS
- or CSS bar lists if you want very low risk

Recommendation:
- traffic = lightweight inline SVG polyline or vertical bar chart
- model usage = ranked horizontal bars

#### Section C: Security summary
- blocked by rules
- blocked by ML
- PII redactions
- escalations
- top threat list

#### Section D: Recent events table
Columns:
- time
- request id
- model
- verdict
- status
- latency
- reason

#### Section E: Runtime panel
- health
- version
- scanner URL/config state
- fail-open mode

### Copy rules
- no hero section
- no product manifesto
- no roadmap storytelling at top
- no defensive phrases like “ini bukan dashboard penuh”
- labels must read like observability product UI

### Empty state rules
If there are zero events:
- show cards with `0`
- show events table empty state like:
  - `No gateway request events recorded yet.`
  - `Send a request to /v1/chat/completions to populate metrics.`

That is acceptable.
Marketing-style apology text is not.

---

## Phase 5: Verification and rollout

### Objective
Prove the dashboard is real, data-backed, and operator-shaped.

### Files / surfaces to verify
- `GET /health`
- `POST /v1/chat/completions`
- `GET /api/dashboard/summary`
- `GET /api/dashboard/traffic`
- `GET /api/dashboard/models`
- `GET /api/dashboard/security`
- `GET /api/dashboard/events`
- `GET /dashboard`

### Verification flow

#### 1. Enable dashboard event file
Confirm config includes dashboard event output path.

#### 2. Generate live traffic
Send multiple test requests through `/v1/chat/completions`:
- one clean request
- one request containing PII
- one prompt-injection-style request
- one request that triggers escalation if possible

#### 3. Inspect event file directly
Verify JSONL lines are written with the expected verdicts.

#### 4. Verify API endpoints
Confirm aggregates reflect those events.

#### 5. Verify UI surface
Open `/dashboard` and confirm:
- KPI cards render real numbers
- traffic panel changes after test requests
- security panel reflects block/redaction events
- recent events list shows real request IDs/models/verdicts

### Minimum acceptance criteria
- dashboard no longer reads like landing page copy
- dashboard displays at least 4 real metrics from runtime data
- recent events section is populated by real request events
- blocked and redacted requests show up distinctly
- `/api/dashboard/*` endpoints return valid JSON
- page still loads if event file is empty

---

## Concrete file-by-file implementation map

### Create
- `gateway/src/models/dashboard.rs`
- `gateway/src/dashboard_events.rs`
- `gateway/src/dashboard_aggregate.rs`
- `gateway/src/routes/dashboard_api.rs`
- `docs/plans/2026-06-16-operator-dashboard-v1.md`

### Modify
- `gateway/src/models/mod.rs`
- `gateway/src/config.rs`
- `gateway/src/state.rs`
- `gateway/src/main.rs`
- `gateway/src/routes/mod.rs`
- `gateway/src/routes/chat.rs`
- `gateway/src/assets/dashboard.html`

### Optional later
- `gateway/config/default.yaml`
  - if repo stores logging defaults there

---

## Recommended build order

### Commit 1
`feat(gateway): add dashboard event schema and writer`
- add event model
- add config/state plumbing
- add event file append helper

### Commit 2
`feat(gateway): emit operator dashboard events from chat pipeline`
- write events for allow/block/redact/escalate/error paths

### Commit 3
`feat(gateway): add dashboard aggregation api`
- add aggregator reader and `/api/dashboard/*`

### Commit 4
`feat(dashboard): replace status page with operator console ui`
- rewrite HTML/CSS/JS to fetch new APIs

### Commit 5
`test(dashboard): verify live metrics and event rendering`
- run requests
- verify event file
- verify endpoints
- verify UI

---

## Risks and guardrails

### Risk 1: partial data joins
If summary uses audit log but security panels use event log, numbers can drift.

**Guardrail:** prefer `DashboardEvent` as the primary source for chat metrics.

### Risk 2: no persisted event file configured
Dashboard will stay empty.

**Guardrail:** expose obvious empty state and document required config.

### Risk 3: UI over-design
Could regress into another marketing page.

**Guardrail:** metric-first layout only. No hero.

### Risk 4: parsing entire file forever
Large JSONL files can become slow.

**Guardrail:** acceptable for v1. Document Postgres/SQLite migration path for v2.

---

## Final recommendation

For Samaryn right now, the correct move is:
1. build event schema
2. emit real security/operator events
3. add read-only aggregation endpoints
4. rewrite dashboard around those endpoints
5. verify with real requests

Anything else will either produce fake dashboard numbers or another landing page in disguise.
