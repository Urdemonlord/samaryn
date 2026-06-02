# Samaryn

**The Cloudflare for LLM Security** — Lightweight, production-ready AI Security Gateway dengan diferensiasi utama pada **IndoBERT** untuk deteksi prompt injection dan PII masking yang akurat dalam bahasa Indonesia.

## Mengapa Samaryn?

Sebagian besar AI Security Gateway saat ini hanya mengandalkan rule-based atau model Inggris-sentris. Samaryn hadir dengan pendekatan berbeda:

- **IndoBERT-native** — Deteksi ancaman dalam bahasa Indonesia jauh lebih akurat dibanding model general
- **Security-first, bukan dashboard-first** — Fokus pada RBAC, API key management, quota, dan audit log sejak awal
- **Production-ready dari hari pertama** — Bukan research prototype; sudah berjalan di production dengan systemd + Nginx

## Nama "Samaryn"

Samaryn berasal dari gabungan **"Sama"** (bahasa Indonesia untuk "same/together") dan **"Ryn"** (dari "guardian"). Filosofinya: *menjaga agar setiap interaksi dengan LLM tetap sama amannya*, terutama untuk konteks bahasa Indonesia dan Asia Tenggara.

## Fitur

### Open Source Core (v0.1+)
- Proxy OpenAI-compatible (`/v1/chat/completions`)
- Prompt injection detection (rule + model)
- PII masking (email, phone, NIK, dll)
- Health check & structured logging
- Support multiple upstream provider (OpenRouter, Anthropic, OpenAI)

### Enterprise Features (v0.2+)
- Role-Based Access Control (RBAC)
- API Key management + rotation
- Per-user quota & rate limiting
- Audit logging lengkap
- Multi-tenant isolation

### Roadmap

| Versi | Fitur Utama                          | Status     |
|-------|--------------------------------------|------------|
| v0.1  | Core gateway + security filter       | ✅ Done    |
| v0.2  | RBAC, API Keys, Quota, Audit         | 🚧 In Progress |
| v0.3  | IndoBERT fine-tuned untuk prompt injection | 📅 Planned |
| v0.4  | WhatsApp Agent integration (RAG)     | 📅 Planned |
| v1.0  | Enterprise SaaS tier                 | 📅 Planned |

## Quick Start

```bash
git clone https://github.com/Urdemonlord/samaryn
cd samaryn
cargo build --release
./target/release/samaryn-gateway
```

Atau pakai systemd (rekomendasi production):

```bash
sudo cp samaryn.service /etc/systemd/system/
sudo systemctl enable --now samaryn
```

## Deployment

Samaryn saat ini berjalan di:
- **Production**: https://samaryn.meowlabs.id
- **Health check**: `GET /health`

## Lisensi

Open source di bawah MIT License. Enterprise features akan tersedia sebagai add-on berbayar di masa depan.

---

**Dibuat dengan fokus pada keamanan LLM yang benar-benar relevan untuk bahasa Indonesia.**