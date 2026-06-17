---
tags:
  - paper
  - tool-use
  - agent
---

# Toolformer Language Models Can Teach Themselves to Use Tools

## Metadata

- Judul: Toolformer: Language Models Can Teach Themselves to Use Tools
- Tahun: 2023
- Link: https://arxiv.org/abs/2302.04761
- Tipe: Tool-use capability paper
- Domain: LLM agents and tools

## Ringkasan inti

- Problem: language model murni terbatas bila harus mengakses tool atau API eksternal.
- Tujuan: menunjukkan model bisa belajar kapan dan bagaimana memakai tools.
- Kontribusi utama: menegaskan bahwa tool access memperbesar risiko ketika instruksi berbahaya lolos.

## Metode

- Setting atau skenario: self-supervised tool-use learning.
- Model atau sistem yang diuji: language model with tool invocation behavior.
- Jenis serangan: tidak fokus pada prompt injection secara langsung.
- Jenis defense: tidak fokus pada defense.

## Dataset atau benchmark

- Nama dataset atau benchmark: task dengan akses tool dan API.
- Ukuran: tidak dirinci di note ini.
- Sumber data: eksperimen tool-use.
- Apakah publik: ya.

## Evaluasi

- Metric utama: task performance with tools.
- Baseline: model tanpa tool-use.
- Hasil penting: memperlihatkan tool integration meningkatkan kemampuan model.

## Kelebihan

- Relevan untuk menjelaskan mengapa sistem agent butuh pre-LLM filtering.
- Mendukung desain gateway yang memeriksa request sebelum tool dipanggil.

## Keterbatasan

- Bukan benchmark security.
- Tidak menilai prompt injection defense.

## Relevansi ke topik saya

- Membantu memosisikan [[Samaryn AI Security Gateway]] sebagai kontrol sebelum tool/provider diakses.

## Koneksi ke note lain

- [[ReAct Synergizing Reasoning and Acting in Language Models]]
- [[Prompt Injection Agent Benchmark]]
- [[Samaryn Security Reading List]]

