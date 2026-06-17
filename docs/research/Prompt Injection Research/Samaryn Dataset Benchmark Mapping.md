---
tags:
  - dataset
  - benchmark
  - mapping
  - samaryn
---

# Samaryn Dataset Benchmark Mapping

Note ini memetakan kandidat dataset dan benchmark untuk penelitian [[Samaryn AI Security Gateway]] dengan fokus detector [[IndoBERT]] dan [[Out-of-Domain Detection]].

## Tujuan mapping

- memilih sumber data latih
- memilih sumber data uji
- membedakan data in-domain, prompt injection, dan out-of-domain
- menjaga evaluasi tetap realistis

## Kandidat utama

| Sumber | Tipe | Cocok untuk | Kelebihan | Keterbatasan |
|---|---|---|---|---|
| [[BIPIA]] | benchmark | indirect prompt injection test set | terstruktur, multi-task | dominan berbahasa Inggris |
| [[InjecAgent]] | benchmark | tool-agent attack scenarios | relevan untuk agentic setting | fokus tool-use, bukan klasifikasi Indo |
| [[LLMail-Inject]] | dataset | adaptive attack samples | realistis dan attacker-driven | domain email assistant |
| [[MPIB]] | benchmark | domain-specific stress test | harm-aware metric | domain medis, bisa terlalu sempit untuk sistem umum |
| [[AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations]] | benchmark | enterprise integration threat model | kuat untuk positioning Samaryn | detail data perlu dicek jika ingin dipakai langsung |
| data internal atau sintetik domain target | dataset | in-domain benign and OOD samples | paling sesuai dengan use case | perlu proses kurasi dan labeling |

## Rekomendasi pembagian data

### Data latih

- prompt normal in-domain berbahasa Indonesia
- prompt injection hasil adaptasi atau translasi terkontrol dari benchmark publik
- sampel out-of-domain dari domain lain yang tidak relevan dengan sistem target

### Data validasi

- subset seimbang dari in-domain, prompt injection, dan OOD
- dipakai untuk threshold tuning dan error analysis

### Data uji

- benchmark publik yang tidak dipakai saat pelatihan
- data OOD dari sumber berbeda
- skenario campuran agar evaluasi lebih realistis

## Skema eksperimen yang disarankan

### Opsi A: sederhana dan feasible

- latih pada data internal plus sintetik
- uji pada subset adaptasi dari [[BIPIA]] dan [[LLMail-Inject]]
- tambahkan OOD set manual berbahasa Indonesia

Kelebihan:
- paling realistis untuk skripsi implementatif
- beban kurasi masih masuk akal

### Opsi B: lebih kuat secara evaluasi

- latih pada kombinasi data internal, sintetik, dan adaptasi benchmark
- uji silang pada [[BIPIA]], [[InjecAgent]], dan [[LLMail-Inject]]
- buat satu set OOD terpisah dari domain yang berbeda

Kelebihan:
- generalisasi lebih bisa diuji

Kekurangan:
- kurasi dan pemetaan label lebih berat

## Skema label yang konsisten

- `safe`
- `prompt_injection`
- `out_of_domain`

Jika ingin lebih rinci:

- `safe`
- `direct_injection`
- `indirect_injection`
- `out_of_domain`

## Risiko metodologis

- translasi otomatis dapat mengubah karakter prompt injection
- data sintetik bisa terlalu mudah dikenali model
- threshold yang dituning pada satu dataset bisa gagal di dataset lain
- distribusi OOD yang terlalu ekstrem bisa membuat hasil tidak representatif

## Rekomendasi praktis

- gunakan benchmark publik sebagai data uji utama, bukan satu-satunya data latih
- sediakan data in-domain Indonesia yang realistis
- pisahkan jelas antara injection detection dan OOD detection saat analisis hasil
- dokumentasikan asal dan transformasi tiap sampel

## Terkait

- [[Bab 3 Metodologi Samaryn Draft]]
- [[Samaryn Paper Review Matrix]]
- [[Prompt Injection Benchmark]]
