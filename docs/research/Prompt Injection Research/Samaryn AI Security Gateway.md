---
tags:
  - samaryn
  - security-gateway
  - prompt-injection
---

# Samaryn AI Security Gateway

Samaryn diposisikan sebagai lapisan keamanan di depan LLM, agent, dan tool integrations untuk mendeteksi, menyaring, atau mengeskalasi request berisiko sebelum menyentuh model atau tool.

## Update Implementasi 2026-06-10

- Implementasi aktual sudah bergerak ke arsitektur hybrid: rule engine cepat ditambah classifier [[IndoBERT]] ONNX di `ml-service`.
- Jalur keputusan runtime yang dipakai sekarang adalah:
  - `SAFE -> allow`
  - `PROMPT_INJECTION -> block`
  - `OUT_OF_DOMAIN -> escalate`
- Gateway sudah memiliki halaman demo interaktif di route `GET /demo` untuk mencoba tiga skenario utama secara langsung melalui endpoint gateway.
- Temuan penting terbaru: model belum cukup andal untuk menangkap semua intent berbahaya secara mandiri, sehingga hard rules tetap diperlukan untuk kata/intent ofensif seperti `bobol web`, `hack`, `retas`, `bypass`, dan sejenisnya.
- Status environment pengguna per 2026-06-10: Docker di laptop ini sudah dihapus, sehingga workflow lokal sebaiknya tidak lagi diasumsikan bergantung pada `docker compose`.

## Fungsi inti

- mitigasi [[Prompt Injection]]
- pemeriksaan [[Indirect Prompt Injection]] dari konten eksternal
- routing ke detector yang tepat
- audit log dan origin tracing
- escalation untuk kasus ambigu

## Arsitektur yang didukung note ini

- rule engine
- classifier berbasis [[IndoBERT]]
- LLM judge atau escalation layer

## Implikasi Riset

- Posisi Samaryn semakin kuat sebagai gateway keamanan berlapis, bukan sekadar classifier tunggal.
- Hasil terbaru menguatkan bahwa kualitas rules dan batas kelas label tetap krusial walaupun classifier [[IndoBERT]] sudah diintegrasikan.
- Untuk konteks skripsi, ini mendukung narasi bahwa kontribusi penelitian tidak berhenti pada fine-tuning model, tetapi juga pada desain pipeline pertahanan yang realistis.

## Relevansi riset

Samaryn cocok dibingkai sebagai AI security gateway untuk agentic systems, enterprise integrations, dan workflow berbasis tool-use.

## Terkait

- [[Out-of-Domain Detection]]
- [[Samaryn Security Reading List]]
- [[Samaryn Literature Review Draft]]
- [[Samaryn Formal Related Work Draft]]
- [[Bab 2 Tinjauan Pustaka Samaryn Tabel]]
- [[Bab 1 Latar Belakang Samaryn Draft]]
- [[Bab 3 Metodologi Samaryn Draft]]
- [[Samaryn Dataset Benchmark Mapping]]
- [[Judul Penelitian Samaryn]]
- [[Research Gap Prompt Injection]]
