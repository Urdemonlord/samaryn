---
tags:
  - paper
  - agent
  - persistent-attack
---

# From Prompt Injection to Persistent Control Defending Agentic Harness Against Trojan Backdoors

## Metadata

- Judul: From Prompt Injection to Persistent Control: Defending Agentic Harness Against Trojan Backdoors
- Tahun: 2026
- Link: https://arxiv.org/abs/2605.31042
- Tipe: Security paper
- Domain: Agentic systems

## Ringkasan inti

- Problem: prompt injection tidak selalu one-shot dan dapat berkembang menjadi kontrol persisten.
- Tujuan: menunjukkan dan mempertahankan sistem dari serangan multi-step yang menetap.
- Kontribusi utama: menggeser perhatian dari input filtering ke state, file output, dan workspace persistence.

## Metode

- Setting atau skenario: agentic harness dengan file atau tool output.
- Model atau sistem yang diuji: agent framework dengan persistent state.
- Jenis serangan: prompt injection leading to persistent control or trojan-like backdoors.
- Jenis defense: harness defense and state-aware protection.

## Dataset atau benchmark

- Nama dataset atau benchmark: skenario persistent control.
- Ukuran: tidak dirinci di note ini.
- Sumber data: serangan multi-step dalam agentic environment.
- Apakah publik: perlu cek artifact jika akan dipakai eksperimen.

## Evaluasi

- Metric utama: keberhasilan mempertahankan agent dari persistent takeover.
- Baseline: defense baseline.
- Hasil penting: menunjukkan risiko tidak berhenti di satu prompt.

## Kelebihan

- Memperluas definisi serangan prompt injection ke level sistem.
- Relevan untuk desain audit log dan origin tracing.

## Keterbatasan

- Bukan paper OOD detection.
- Detail benchmark kuantitatif perlu dicek lagi.

## Relevansi ke topik saya

- Menguatkan kebutuhan scanning state dan audit pada [[Samaryn AI Security Gateway]].

## Koneksi ke note lain

- [[Samaryn Security Reading List]]
- [[Research Gap Prompt Injection]]
- [[Prompt Injection Agent Benchmark]]

