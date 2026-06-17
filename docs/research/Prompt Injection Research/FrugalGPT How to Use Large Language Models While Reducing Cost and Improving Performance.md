---
tags:
  - paper
  - routing
  - cascade
---

# FrugalGPT How to Use Large Language Models While Reducing Cost and Improving Performance

## Metadata

- Judul: FrugalGPT: How to Use Large Language Models While Reducing Cost and Improving Performance
- Tahun: 2023
- Link: https://arxiv.org/abs/2305.05176
- Tipe: Routing or cascade paper
- Domain: LLM systems optimization

## Ringkasan inti

- Problem: pemakaian LLM besar mahal dan tidak selalu efisien.
- Tujuan: merancang pipeline routing atau cascade yang menekan biaya sambil menjaga performa.
- Kontribusi utama: memberi pola desain yang bisa diadaptasi untuk security triage.

## Metode

- Setting atau skenario: multi-stage routing atau cascade.
- Model atau sistem yang diuji: kombinasi beberapa model dengan strategi pemilihan.
- Jenis serangan: tidak fokus pada serangan.
- Jenis defense: bukan defense prompt injection, tetapi relevan untuk desain defense pipeline.

## Dataset atau benchmark

- Nama dataset atau benchmark: task LLM umum.
- Ukuran: tidak dirinci di note ini.
- Sumber data: benchmark aplikasi LLM.
- Apakah publik: ya.

## Evaluasi

- Metric utama: cost-performance tradeoff.
- Baseline: single-model strategy.
- Hasil penting: menunjukkan routing dapat meningkatkan efisiensi.

## Kelebihan

- Langsung berguna untuk arsitektur sistem.
- Cocok untuk pola rule engine, classifier, lalu escalation.

## Keterbatasan

- Tidak spesifik ke keamanan.
- Tidak mengevaluasi adversarial input.

## Relevansi ke topik saya

- Dasar desain cascade pada [[Samaryn AI Security Gateway]] untuk request filtering, classifier [[IndoBERT]], dan escalation.

## Koneksi ke note lain

- [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]]
- [[Samaryn Security Reading List]]
- [[Out-of-Domain Detection]]

