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

## How It Works

```
Your Application
       ↓
┌─────────────────────────────┐
│    Samaryn Gateway (Rust)   │
│  ┌───────────────────────┐  │
│  │ 1. Security Rules     │  │  ← Block known attack patterns
│  │ 2. PII Detection      │  │  ← Regex-based, sub-μs latency
│  │ 3. ML Security Scan   │──│──→ Python ML Service
│  │ 4. PII Masking        │  │  ← Replace PII with placeholders
│  │ 5. Forward to LLM     │──│──→ OpenAI / Anthropic / etc.
│  │ 6. Audit Log          │  │  ← Structured JSONL
│  └───────────────────────┘  │
└─────────────────────────────┘
       ↓
LLM Provider Response
```

---

## Configuration

Edit `config/default.yaml`:

```yaml
security:
  pii_masking: true            # Enable PII detection & masking
  injection_detection: true     # Enable prompt injection detection
  action: "block"              # block | redact | warn

providers:
  - name: openai
    base_url: "https://api.openai.com/v1"
    api_key: "${OPENAI_API_KEY}"
    models: ["gpt-4o", "gpt-4o-mini"]
```

See [config/default.yaml](config/default.yaml) for all options.

---

## Architecture

| Component | Stack | Purpose |
|---|---|---|
| **Gateway** | Rust + Axum + Tokio | Reverse proxy, streaming, PII regex, routing, logging |
| **ML Service** | Python + FastAPI | Hybrid prompt scan, IndoBERT ONNX classification, advanced PII analysis |

Both services run as Docker containers orchestrated by Docker Compose.

---

## Supported Providers

| Provider | Status |
|---|---|
| OpenAI | ✅ Supported |
| Anthropic | ✅ Supported |
| Google Gemini | ✅ Supported |
| Ollama (local) | ✅ Supported |
| OpenRouter | ✅ Supported |

---

## Development

### Run gateway locally (requires Rust toolchain)

```bash
cd gateway
cargo run
```

### Run ML service locally (requires Python 3.12+)

```bash
cd ml-service
pip install -r requirements.txt
set ML_SERVICE_MODEL_DIR=C:\path\to\samaryn\ml-service\models\indobert-agentwa
uvicorn app.main:app --reload --port 8000
```

The local model bundle is expected under `ml-service/models/indobert-agentwa/` and includes:
- `model.onnx`
- `config.json`
- `tokenizer.json`
- `tokenizer_config.json`
- `training_metadata.json`

For CI/CD and VPS deploys, `model.onnx` is **not committed to git**. GitHub Actions downloads it from a GitHub Release asset before building the ML image. By default the workflow expects release assets named:
- `model.onnx`
- `model.onnx.sha256`

Optional GitHub Actions repository variables:
- `SAMARYN_MODEL_RELEASE_TAG` — pin the model to a specific release tag; if unset, the latest release is used
- `SAMARYN_MODEL_ASSET_NAME` — override the ONNX asset filename (default `model.onnx`)
- `SAMARYN_MODEL_CHECKSUM_ASSET_NAME` — override the checksum asset filename (default `model.onnx.sha256`)

---

## Contributing

We welcome contributions! Here's how to get started:

### Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/samaryn.git
   cd samaryn
   ```
3. **Create a branch** for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Development Setup

**Gateway (Rust):**
```bash
cd gateway
cargo build
cargo run
```

**ML Service (Python):**
```bash
cd ml-service
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
uvicorn app.main:app --reload --port 8000
```

### Making Changes

1. Write your code following the existing style
2. Add tests if applicable
3. Update [CHANGELOG.md](CHANGELOG.md) with your changes
4. Commit with clear messages:
   ```bash
   git commit -m "feat: add X feature"
   git commit -m "fix: resolve Y issue"
   ```

### Submit a Pull Request

1. **Push** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
2. **Create a Pull Request** on GitHub
3. **Describe** your changes and reference any related issues
4. Wait for review and address feedback

### Code Guidelines

- **Rust:** Follow [rustfmt](https://github.com/rust-lang/rustfmt) conventions
- **Python:** Follow [PEP 8](https://pep8.org/) and use `black` for formatting
- Keep functions small and focused
- Add docstrings/comments for complex logic
- Write tests for new features

### Reporting Issues

Found a bug? Please open an [Issue](https://github.com/Urdemonlord/samaryn/issues) with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Docker version, etc.)

---

## Roadmap

### ✅ Released (v0.1.0)

- [x] OpenAI-compatible proxy gateway
- [x] PII masking (email, phone, API keys, JWT, credit cards)
- [x] Prompt injection detection (rule-based)
- [x] Audit logging (JSONL)
- [x] Multi-provider routing
- [x] Docker Compose deployment

### 🚀 Upcoming (v0.2.0)

- [ ] Web dashboard for monitoring & logs
- [ ] Analytics & usage metrics per model/user
- [ ] Real-time stream filtering
- [ ] Custom security rule builder UI

### 🎯 Future Releases (v0.3.0+)

- [x] ML-based injection detection (IndoBERT ONNX in ML service)
- [ ] Enterprise policy engine
- [ ] Role-based access control (RBAC)
- [ ] Advanced threat detection
- [ ] Kubernetes support
- [ ] Rate limiting & quota management

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<p align="center">
  Built with ❤️ for AI security
</p>
