---
tags:
  - paper
  - benchmark
  - enterprise
  - agent
---

# AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations

## Metadata

- Judul: AgentRedBench: Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations
- Tahun: 2026
- Link: https://arxiv.org/abs/2606.02240
- Tipe: Benchmark and defense paper
- Domain: LLM agents over enterprise SaaS integrations

## Ringkasan inti

- Problem: agent yang membaca integrasi SaaS membawa risiko prompt injection dari pihak ketiga.
- Tujuan: mengevaluasi dan mempertahankan agent pada skenario integrasi enterprise.
- Kontribusi utama: menempatkan indirect prompt injection sebagai ancaman nyata di jalur Gmail, Salesforce, Jira, dan integrasi sejenis.

## Metode

- Setting atau skenario: dynamic redteaming untuk integration-aware agents.
- Model atau sistem yang diuji: agent yang berinteraksi dengan SaaS integrations.
- Jenis serangan: indirect prompt injection.
- Jenis defense: integration-aware defense.

## Dataset atau benchmark

- Nama dataset atau benchmark: AgentRedBench.
- Ukuran: tidak dirinci di note ini.
- Sumber data: skenario agent dengan integrasi enterprise.
- Apakah publik: perlu cek artifact publik jika ingin direplikasi.

## Evaluasi

- Metric utama: robustness under integration-aware attacks.
- Baseline: agent dan defense baseline.
- Hasil penting: memperlihatkan bahwa integrasi pihak ketiga adalah jalur serangan praktis.

## Kelebihan

- Sangat relevan untuk use case enterprise.
- Membantu positioning produk gateway.

## Keterbatasan

- Fokus domain enterprise integration.
- Detail ukuran benchmark perlu dicek lagi bila dipakai sebagai dataset utama.

## Relevansi ke topik saya

- Sangat cocok untuk narasi [[Samaryn AI Security Gateway]] sebagai kontrol pada jalur integrasi.

## Koneksi ke note lain

- [[Indirect Prompt Injection]]
- [[Samaryn Security Reading List]]
- [[Samaryn Literature Review Draft]]

