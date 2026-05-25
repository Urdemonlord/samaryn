# Product Requirements Document

**Samaryn** — Open-source AI Security Gateway for Enterprise Applications

---

## Overview

| Field | Detail |
|---|---|
| **Project Name** | Samaryn |
| **Tagline** | Open-source AI Security Gateway for Enterprise Applications |
| **Version** | v0.1 (MVP) |
| **License** | Apache 2.0 / MIT |
| **Business Model** | Open Core |

### Summary

Samaryn adalah middleware open source yang bertindak sebagai security dan privacy layer di antara aplikasi dan Large Language Model (LLM) provider seperti OpenAI, Anthropic, dan local model seperti Ollama.

**Tujuan utama:**

- Melindungi data sensitif dari kebocoran ke provider AI
- Mencegah prompt injection attack
- Memberikan observability dan audit trail AI
- Menyediakan AI governance layer
- Mempermudah perusahaan menggunakan AI secara aman

Samaryn dirancang untuk bersifat **self-hosted**, **lightweight**, **OpenAI-compatible**, dan **developer-friendly**.

---

## Problem Statement

Perusahaan yang ingin mengadopsi AI menghadapi berbagai tantangan berikut:

| Masalah | Dampak |
|---|---|
| Data sensitif bocor ke provider AI | Risiko keamanan dan kepatuhan |
| Prompt injection attack | Manipulasi perilaku model |
| Tidak ada audit log AI | Tidak ada accountability |
| Sulit enforce security policy | Governance lemah |
| AI agent terlalu bebas mengakses tools | Eksposur sistem internal |
| Tidak ada observability request AI | Blind spot operasional |
| Compliance concern (healthcare, fintech) | Hambatan adopsi enterprise |

Saat ini, developer harus membangun seluruh security layer sendiri tanpa tooling standar.

---

## Solution

Samaryn berperan sebagai **security layer** di tengah alur komunikasi antara aplikasi dan LLM provider.

```
Application
     ↓
Samaryn Gateway
     ↓
LLM Provider
```

Samaryn menangani:

- PII masking otomatis
- Prompt firewall dan injection detection
- Output filtering
- Request & response logging
- Multi-provider routing
- Security policy enforcement
- AI observability

Tanpa mengharuskan developer mengubah arsitektur aplikasi yang sudah ada.

---

## Target Users

### Primary Users

| Segmen | Deskripsi |
|---|---|
| Enterprise internal AI team | Tim yang mengelola adopsi AI internal perusahaan |
| AI SaaS developers | Developer yang membangun produk berbasis AI |
| AI chatbot developers | Developer chatbot untuk pelanggan atau operasional |
| AI WhatsApp bot developers | Developer bot berbasis WhatsApp API |
| Healthcare AI systems | Sistem AI di lingkungan medis |
| Fintech AI systems | Sistem AI di layanan keuangan |

### Secondary Users

| Segmen | Deskripsi |
|---|---|
| OSS AI developers | Kontributor dan pengguna open source |
| Security researchers | Peneliti keamanan AI |
| AI infra engineers | Engineer yang membangun infrastruktur AI |
| DevOps engineers | Engineer yang mengelola deployment sistem AI |

---

## Core Features

### 1. AI Gateway

OpenAI-compatible API endpoint yang memungkinkan developer hanya perlu mengganti `baseURL`.

**Endpoint:**

```
POST /v1/chat/completions
```

**Sebelum:**

```javascript
const client = new OpenAI()
```

**Sesudah:**

```javascript
const client = new OpenAI({
  baseURL: "https://samaryn.local/v1"
})
```

---

### 2. PII Masking

Otomatis mendeteksi dan menyembunyikan data pribadi sensitif sebelum dikirim ke LLM provider.

**Data yang di-mask:**

- Email address
- Nomor telepon
- API key & token
- Nomor rekening bank
- Credentials

**Contoh:**

```
Input:  Nomor saya 08123456789
Output: Nomor saya [PHONE_1]
```

---

### 3. Prompt Firewall

Mendeteksi dan memblokir request berbahaya sebelum mencapai LLM.

**Deteksi:**

- Prompt injection
- Role override attempt
- Jailbreak attempt
- Hidden unicode attack
- Encoded attack
- Malicious tool request

---

### 4. Output Filtering

Mencegah kebocoran informasi sensitif pada response LLM.

**Melindungi dari:**

- Secret leakage
- Credential exposure
- Unsafe output
- Sensitive data leak

---

### 5. Audit Logging

Menyimpan jejak lengkap setiap interaksi AI untuk keperluan audit, debugging, dan compliance.

**Tracked fields:**

- Request & response content
- Token usage
- Provider yang digunakan
- Latency
- Flagged attacks

---

### 6. Multi-Provider Routing

Mendukung routing request ke berbagai LLM provider.

| Provider | Status |
|---|---|
| OpenAI | ✅ Supported |
| Anthropic | ✅ Supported |
| Gemini | ✅ Supported |
| Ollama | ✅ Supported |
| OpenRouter | ✅ Supported |

---

### 7. Streaming Interceptor

Filtering realtime untuk streaming response dari LLM.

```
LLM Stream
    ↓
Samaryn Interceptor
    ↓
Filtered Stream
```

---

### 8. Security Rules Engine

Konfigurasi berbasis YAML untuk mendefinisikan kebijakan keamanan secara deklaratif.

**Contoh konfigurasi:**

```yaml
security:
  pii_masking: true
  injection_detection: true

rules:
  - block:
      contains:
        - "ignore previous instructions"

  - redact:
      type: api_key
```

---

## Architecture

### High-Level Architecture

```
Client
  ↓
Samaryn Gateway (Rust)
  ↓
ML Security Service (Python)
  ↓
LLM Providers
```

### Rust Gateway

Bertanggung jawab atas:

- Reverse proxy
- Streaming handler
- Request pipeline
- Logging
- Routing
- Rate limiting
- Middleware execution

**Stack:** Axum · Tokio · Hyper

### Python ML Service

Bertanggung jawab atas:

- IndoBERT classification
- Prompt injection detection
- NLP analysis
- Embeddings
- Advanced security scanning

**Stack:** FastAPI · PyTorch · Transformers

---

## Tech Stack

| Layer | Technology |
|---|---|
| Gateway | Rust + Axum |
| ML Service | Python + FastAPI |
| Cache | Redis |
| Database | PostgreSQL |
| Analytics | ClickHouse |
| Vector DB | Qdrant |
| Dashboard | Next.js |
| Deployment | Docker |

---

## Deployment

Samaryn dirancang untuk self-hosted di berbagai environment.

```bash
docker compose up
```

**Supported environments:**

- VPS
- Kubernetes
- On-premise server
- Docker (local/cloud)

---

## MVP Scope (v0.1)

### In Scope

| Feature | Keterangan |
|---|---|
| OpenAI-compatible proxy | Drop-in replacement dengan ubah baseURL |
| PII masking | Masking email, phone, API key, token |
| Basic prompt injection detection | Rule-based detection |
| Audit logging | Log request, response, latency, token |
| Docker deployment | Single command `docker compose up` |

### Out of Scope (v0.1)

- Dashboard kompleks
- Billing & payment
- Multi-tenancy
- RBAC (Role-Based Access Control)

---

## Roadmap

### v0.2

- Web dashboard
- Analytics & usage metrics
- Rule editor UI
- Provider fallback

### v0.3

- Realtime stream filtering
- AI agent sandbox
- Tool permission system

### v1.0

- Enterprise policy engine
- Distributed deployment
- Compliance mode (HIPAA, SOC2)
- Advanced observability

---

## Open Source Strategy

### License

Apache 2.0 atau MIT (TBD)

### Business Model — Open Core

| Tier | Features |
|---|---|
| **Free (Open Source)** | Gateway, PII masking, Prompt firewall, Multi-provider routing, Basic logging |
| **Paid (Future)** | Enterprise dashboard, Team management, Cloud hosting, Advanced analytics, Compliance packs |

---

## Key Differentiators

| Dibanding | Keunggulan Samaryn |
|---|---|
| AI wrappers | Fokus pada security dan governance, bukan sekadar abstraksi |
| Observability tools | Aktif memfilter dan melindungi request, bukan hanya memonitor |
| General API gateways | AI-native dan memahami konteks prompt |

---

## Vision

> Menjadi **standard open-source security layer** untuk aplikasi AI modern.
