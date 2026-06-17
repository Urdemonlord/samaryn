# Samaryn Codebase Structure

Tanggal cek: 2026-06-04
Workspace kode: `C:\Users\click\samaryn`
Vault referensi: `D:\obsidian\meow\Prompt Injection Research`

## Ringkasan Singkat

Codebase `samaryn` saat ini adalah prototype gateway keamanan AI dengan dua service utama:

1. `gateway/` dalam Rust
2. `ml-service/` dalam Python FastAPI

Deployment repo tetap menyediakan `docker-compose.yml`, dengan `config/default.yaml` sebagai sumber konfigurasi utama. Namun status environment pengguna per 2026-06-10 adalah Docker sudah dihapus dari laptop ini, sehingga verifikasi lokal sebaiknya tidak diasumsikan selalu lewat container.

Secara implementasi, repo ini sudah memiliki alur dasar:

`client -> Rust gateway -> security scan / PII redaction -> upstream LLM provider`

Namun beberapa bagian yang disebut di `PRD.md` masih belum ada di codebase, misalnya dashboard, database, Redis, dan analytics. Catatan lama bahwa "belum ada model ML nyata seperti IndoBERT" sudah tidak berlaku lagi, karena repo sekarang sudah memuat bundle model ONNX untuk classifier [[IndoBERT]].

## Update Implementasi 2026-06-10

- `ml-service` sekarang memuat classifier [[IndoBERT]] ONNX dari bundle model lokal:
  - `model.onnx`
  - `config.json`
  - `tokenizer.json`
  - `tokenizer_config.json`
  - `training_metadata.json`
- `ml-service/app/services/indobert_runtime.py` menjadi inference layer aktual untuk klasifikasi `SAFE / PROMPT_INJECTION / OUT_OF_DOMAIN`.
- `ml-service/app/services/injection.py` sekarang merupakan detector hybrid:
  - hard rules untuk override, exfiltration, system prompt markers, encoded attack, dan intent ofensif
  - rules `OUT_OF_DOMAIN` untuk context drift dan request di luar domain bisnis
  - classifier [[IndoBERT]] ONNX sebagai fallback keputusan tiga kelas
- Gateway Rust sekarang punya route tambahan `GET /demo` untuk halaman demo interaktif.
- Repo sekarang memiliki test minimal di `ml-service/tests/` untuk memverifikasi rule penting seperti blokir `bobol web` dan eskalasi context drift.

## Struktur Folder Utama

```text
samaryn/
|- config/
|  `- default.yaml
|- gateway/
|  |- Cargo.toml
|  |- Dockerfile
|  `- src/
|     |- main.rs
|     |- config.rs
|     |- error.rs
|     |- state.rs
|     |- middleware/
|     |- models/
|     |- proxy/
|     |- routes/
|     `- security/
|- ml-service/
|  |- Dockerfile
|  |- requirements.txt
|  `- app/
|     |- main.py
|     |- core/
|     |- routers/
|     |- schemas/
|     `- services/
|- docker-compose.yml
|- README.md
`- PRD.md
```

## Peran Tiap Bagian

### 1. Root repository

- `README.md`
  Menjelaskan positioning produk: OpenAI-compatible AI security gateway.
- `PRD.md`
  Menjelaskan visi produk, target user, roadmap, dan fitur yang diinginkan.
- `docker-compose.yml`
  Menjalankan dua container: `gateway` dan `ml-service`.
- `.env.example`
  Menyediakan placeholder API key provider.

### 2. `config/`

- `default.yaml`
  Menyimpan:
  - server host/port
  - daftar provider LLM
  - URL ML service
  - flag security
  - lokasi audit log

Provider yang dideklarasikan:
- OpenAI
- Anthropic
- Gemini
- OpenRouter
- Ollama

### 3. `gateway/` (Rust)

Ini adalah service inti yang menerima request OpenAI-compatible dari client.

#### File utama

- `src/main.rs`
  Entry point server Axum. Di sini dibangun:
  - tracing/logging
  - shared `AppState`
  - route `/v1/chat/completions`
  - route `/health`
  - middleware auth dan audit logging
  - timeout, CORS, request-id

- `src/config.rs`
  Loader konfigurasi YAML + override environment variable `SAMARYN__...`.

- `src/state.rs`
  Menyatukan dependency runtime seperti config, HTTP client, PII detector, rules engine, scanner, dan provider router.

- `src/error.rs`
  Definisi error gateway.

#### Folder internal

- `routes/`
  - `chat.rs`: handler utama proxy chat completion
  - `demo.rs`: halaman demo interaktif gateway
  - `health.rs`: health endpoint

- `proxy/`
  - `provider.rs`: memilih provider berdasarkan nama model
  - `streaming.rs`: forward response streaming dan non-streaming

- `security/`
  - `pii.rs`: deteksi/redaksi PII berbasis regex di sisi Rust
  - `rules.rs`: rules engine untuk blokir pola prompt tertentu
  - `scanner.rs`: client HTTP ke ML service

- `middleware/`
  - `auth.rs`: API key auth gateway client
  - `logging.rs`: audit logging request/response

- `models/`
  - `openai.rs`: schema request/response yang meniru OpenAI API
  - `security.rs`: schema request/response untuk komunikasi ke ML service

### 4. `ml-service/` (Python FastAPI)

Ini adalah service pendukung untuk scanning teks.

#### File utama

- `app/main.py`
  Entry point FastAPI. Mendaftarkan router health dan scan, serta logging dasar.

- `app/core/config.py`
  Konfigurasi runtime Python service.

#### Folder internal

- `routers/`
  - `health.py`: endpoint health check
  - `scan.py`: endpoint scan teks

- `schemas/`
  - `scan.py`: model request/response scan

- `services/`
  - `indobert_runtime.py`: loader dan inference runtime untuk model ONNX [[IndoBERT]]
  - `injection.py`: detektor hybrid berbasis hard rules + classifier [[IndoBERT]]
  - `pii.py`: detektor PII berbasis regex dan anonymizer placeholder

- `tests/`
  - `test_injection_detector.py`: unit test dasar untuk prompt ofensif dan context drift

## Alur Request Aktual di Code

Implementasi aktual yang terlihat dari `gateway/src/routes/chat.rs`:

1. Gateway menerima `POST /v1/chat/completions`
2. Semua konten pesan diekstrak menjadi satu string
3. Rules engine lokal mengecek prompt injection sederhana
4. Jika `pii_masking` aktif, konten pesan di-redact langsung di gateway
5. Gateway memanggil ML service untuk scan tambahan
6. Gateway menentukan provider upstream dari field `model`
7. Request diteruskan ke provider
8. Response dikembalikan ke client, termasuk mode streaming

Artinya, tanggung jawab security saat ini terbagi dua:

- Rust gateway: filtering cepat dan redaction inline
- Python ML service: scan tambahan berbasis rules dan classifier [[IndoBERT]] ONNX

## Dependensi Nyata yang Dipakai

### Gateway

Dari `gateway/Cargo.toml`, dependensi penting:

- `axum`
- `tokio`
- `tower` dan `tower-http`
- `reqwest`
- `serde`, `serde_json`, `serde_yaml`
- `tracing`
- `regex`
- `uuid`

### ML service

Dari `ml-service/requirements.txt`, dependensi penting sekarang mencakup:

- `fastapi`
- `uvicorn`
- `pydantic`
- `pydantic-settings`
- `numpy`
- `onnxruntime`
- `transformers`

Catatan: runtime yang dipakai sekarang adalah ONNX, bukan PyTorch training runtime penuh.

## Kesesuaian PRD vs Implementasi

### Yang sudah tampak di code

- OpenAI-style gateway endpoint
- PII masking / anonymization berbasis regex
- Prompt injection detection berbasis rules/regex
- Detector hybrid berbasis rules + [[IndoBERT]] ONNX
- Multi-provider config
- Dockerized local deployment
- Health endpoint
- Streaming proxy support
- Demo page di `GET /demo`

### Yang belum tampak di code

- Dashboard web
- Analytics stack
- Redis
- PostgreSQL
- ClickHouse
- Qdrant
- Next.js app
- Rate limiting
- RBAC
- Enterprise policy engine
- Dashboard web penuh

## Observasi Teknis Penting

### 1. Codebase masih MVP yang cukup ramping

Walaupun `PRD.md` menyebut visi yang luas, implementasi saat ini fokus pada core gateway dan scanner sederhana. Ini bagus untuk fondasi, tapi penting membedakan antara:

- fitur yang benar-benar sudah ada
- fitur yang baru ada di dokumen produk

### 2. ML service sekarang hybrid, bukan rule-based murni

`ml-service/app/services/injection.py` masih memakai regex/pattern matching untuk serangan eksplisit, tetapi sekarang juga memanggil runtime ONNX [[IndoBERT]] untuk klasifikasi tiga kelas. Jadi label "ML service" sudah lebih tepat dibanding catatan awal, walaupun tetap bukan platform ML yang besar.

### 3. Ada jejak artefak runtime Python di repo

Folder `__pycache__` ada di `ml-service/app/core`, `schemas`, dan `services`. Ini menandakan repo pernah dijalankan lokal dan belum dibersihkan dari artefak bytecode Python.

### 4. Test suite masih kecil, tetapi sudah mulai ada

Saat ini sudah ada `ml-service/tests/` untuk unit test dasar detector. Namun coverage masih kecil dan gateway Rust belum terlihat memiliki test suite yang setara.

### 5. Ada potensi mismatch endpoint internal

Di `ml-service/app/routers/scan.py`, endpoint scan terdaftar dengan prefix `/api/v1`, sehingga route efektifnya tampak sebagai:

`POST /api/v1/scan`

Tetapi di `gateway/src/security/scanner.rs`, URL yang dibentuk adalah:

`{ml_service_url}/scan`

Dengan default config `http://ml-service:8000`, gateway tampaknya akan memanggil:

`http://ml-service:8000/scan`

Ini terlihat tidak konsisten dengan route FastAPI yang ada, dan perlu dicek karena bisa membuat scanning gagal di runtime.

### 6. Ada potensi mismatch path upstream provider OpenAI-compatible

Di `gateway/src/routes/chat.rs`, URL upstream dibentuk sebagai:

`{base_url}/chat/completions`

Sementara di `config/default.yaml`, `openai.base_url` diisi:

`https://api.openai.com`

Jika dibiarkan seperti ini, hasilnya menjadi:

`https://api.openai.com/chat/completions`

Padahal endpoint OpenAI-style biasanya berada di jalur `/v1/...`. Ini juga perlu diverifikasi saat masuk tahap debugging integrasi provider.

## Kesimpulan Praktis

Kalau codebase ini dibaca sebagai bahan skripsi/proyek riset, posisi terbaiknya saat ini adalah:

- sebuah MVP AI security gateway
- arsitektur microservice sederhana
- fokus utama pada proxying, PII redaction, dan prompt injection detection hybrid berbasis rules + [[IndoBERT]] ONNX

Belum tepat jika dipresentasikan sebagai platform AI security enterprise yang lengkap, kecuali dibedakan jelas antara:

- implementasi saat ini
- roadmap pengembangan berikutnya

## Saran Catatan Lanjutan

Untuk langkah dokumentasi berikutnya, note turunan yang masuk akal:

1. `Samaryn Request Flow Detail`
2. `Samaryn PRD vs Implementasi`
3. `Samaryn Technical Debt dan Bug Kandidat`
4. `Samaryn Evaluasi untuk Penelitian Prompt Injection`
