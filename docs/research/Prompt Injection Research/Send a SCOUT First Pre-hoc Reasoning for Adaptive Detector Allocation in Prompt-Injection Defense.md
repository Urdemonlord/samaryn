---
tags:
  - paper
  - defense
  - detector-allocation
---

# Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense

## Metadata

- Judul: Send a SCOUT First: Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense
- Tahun: 2026
- Link: https://arxiv.org/abs/2605.30837
- Tipe: Prompt injection defense paper
- Domain: LLM security

## Ringkasan inti

- Problem: detector tunggal punya blind spot dan tidak optimal untuk semua request.
- Tujuan: memilih detector yang tepat secara adaptif untuk tiap request.
- Kontribusi utama: framing defense sebagai detector allocation, bukan one-size-fits-all detector.

## Metode

- Setting atau skenario: adaptive defense selection.
- Model atau sistem yang diuji: beberapa detector dengan alokasi adaptif.
- Jenis serangan: prompt injection.
- Jenis defense: adaptive detector allocation.

## Dataset atau benchmark

- Nama dataset atau benchmark: benchmark prompt injection.
- Ukuran: tidak dirinci di note ini.
- Sumber data: evaluasi prompt injection benchmark.
- Apakah publik: diasumsikan ya dari paper arXiv, perlu cek implementasi detail bila dipakai eksperimen.

## Evaluasi

- Metric utama: efektivitas detector allocation dibanding detector tetap.
- Baseline: single detector setup.
- Hasil penting: mendukung ide bahwa defense berlapis lebih realistis daripada satu filter tunggal.

## Kelebihan

- Paling dekat dengan arsitektur hybrid yang diinginkan.
- Sangat kuat untuk justifikasi desain sistem.

## Keterbatasan

- Detail benchmark dan implementasi perlu dicek lagi jika akan direplikasi.
- Masih bergantung pada kualitas detector dasar.

## Relevansi ke topik saya

- Menjadi referensi utama untuk desain [[Samaryn AI Security Gateway]] berbasis rule engine, ML detector, dan escalation.

## Koneksi ke note lain

- [[FrugalGPT How to Use Large Language Models While Reducing Cost and Improving Performance]]
- [[Research Gap Prompt Injection]]
- [[Samaryn Security Reading List]]

