---
tags:
  - paper
  - react
  - agent
---

# ReAct Synergizing Reasoning and Acting in Language Models

## Metadata

- Judul: ReAct: Synergizing Reasoning and Acting in Language Models
- Tahun: 2022
- Link: https://arxiv.org/abs/2210.03629
- Tipe: Agent reasoning framework
- Domain: LLM agents

## Ringkasan inti

- Problem: pure chain-of-thought atau pure action pipelines tidak cukup untuk banyak task interaktif.
- Tujuan: menggabungkan reasoning dan acting secara interleaved.
- Kontribusi utama: memperjelas bahwa LLM modern bukan hanya generator teks, tetapi decision-maker yang bisa memicu aksi.

## Metode

- Setting atau skenario: reasoning plus action loop.
- Model atau sistem yang diuji: language model agent with reasoning and action traces.
- Jenis serangan: tidak fokus pada serangan, tetapi memperbesar attack surface ketika dipakai di sistem nyata.
- Jenis defense: tidak fokus pada defense.

## Dataset atau benchmark

- Nama dataset atau benchmark: berbagai task agentic dan reasoning.
- Ukuran: tidak dirinci di note ini.
- Sumber data: benchmark task interaktif.
- Apakah publik: ya.

## Evaluasi

- Metric utama: task success dan reasoning quality.
- Baseline: prompting alternatives.
- Hasil penting: menunjukkan sinergi reasoning dan action dapat meningkatkan performa agent.

## Kelebihan

- Sangat berguna untuk menjelaskan attack surface agentic systems.
- Cocok untuk narasi mengapa gateway keamanan diperlukan.

## Keterbatasan

- Bukan paper prompt injection.
- Tidak mengevaluasi mitigasi keamanan.

## Relevansi ke topik saya

- Memperkuat alasan keberadaan [[Samaryn AI Security Gateway]] di depan model yang dapat bertindak.
- Menjelaskan konteks mengapa [[Prompt Injection]] pada agent lebih berbahaya daripada chat biasa.

## Koneksi ke note lain

- [[Toolformer Language Models Can Teach Themselves to Use Tools]]
- [[Prompt Injection Agent Benchmark]]
- [[Samaryn Security Reading List]]

