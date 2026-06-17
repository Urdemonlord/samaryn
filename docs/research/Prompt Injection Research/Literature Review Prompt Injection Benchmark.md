---
tags:
  - literature-review
  - prompt-injection
  - benchmark
  - dataset
---

# Literature Review Prompt Injection Benchmark

Note ini merangkum paper paling relevan untuk topik [[Prompt Injection Dataset]] dan [[Prompt Injection Benchmark]].

## Tabel ringkas

| Paper | Tipe | Skenario | Ukuran | Fokus utama | Metric atau evaluasi penting | Kekuatan | Keterbatasan |
|---|---|---|---|---|---|---|---|
| [[LLMail-Inject]] | Dataset | Email assistant | 208,095 submissions, 839 participants | Serangan realistis dan adaptif | Evaluasi lintas defense, arsitektur, retrieval | Paling dekat ke dinamika attacker nyata | Domain masih sempit ke email assistant |
| [[BIPIA]] | Benchmark | External-content LLM tasks | 626,250 train prompts, 86,250 test prompts | Indirect prompt injection umum | ASR dan kualitas output | Structured benchmark besar lintas task | Lebih sintetis daripada challenge nyata |
| [[InjecAgent]] | Benchmark | Agent + tools | 1,054 test cases | Tool-integrated LLM agents | Keberhasilan serangan pada tool use | Sangat relevan untuk agentic workflows | Lebih kecil dari benchmark skala besar |
| [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] | Framework + benchmark platform | LLM-integrated applications | 5 attacks, 10 defenses, 10 LLM, 7 tasks | Landasan metodologi evaluasi | Perbandingan sistematis attack vs defense | Kuat sebagai basis konseptual | Bukan dataset nyata yang besar |
| [[MPIB]] | Dataset + benchmark | Medis dan clinical RAG | 9,697 curated instances | Clinical safety under prompt injection | CHER dan ASR | Tambah dimensi harm, bukan cuma compliance | Khusus domain medis |

## Sintesis

Jika targetmu adalah dataset nyata, [[LLMail-Inject]] paling menonjol karena berasal dari challenge publik dan memotret serangan adaptif.

Jika targetmu adalah benchmark umum untuk [[Indirect Prompt Injection]], [[BIPIA]] paling foundational karena cakupan task-nya luas dan strukturnya rapi untuk evaluasi berulang.

Jika targetmu adalah sistem agentic, [[InjecAgent]] paling relevan karena benchmark-nya dibangun khusus untuk tool-calling agents.

Jika kamu butuh landasan konseptual dan evaluasi sistematis attack-defense, [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] adalah basis metodologinya.

Jika konteks risetmu domain-spesifik dan safety-critical, [[MPIB]] paling tepat karena memasukkan harm-aware metric.

## Rekomendasi pemakaian

- gunakan [[LLMail-Inject]] untuk argumen tentang real-world adaptive attacks
- gunakan [[BIPIA]] untuk baseline benchmark umum
- gunakan [[InjecAgent]] bila topikmu agent, tools, atau autonomous actions
- gunakan [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] untuk framework teori dan perbandingan defense
- gunakan [[MPIB]] bila fokus pada healthcare, RAG klinis, atau safety evaluation

## Koneksi konsep

- [[Prompt Injection]]
- [[Indirect Prompt Injection]]
- [[Prompt Injection Dataset]]
- [[Prompt Injection Benchmark]]
- [[Prompt Injection Agent Benchmark]]
- [[Medical Prompt Injection]]

