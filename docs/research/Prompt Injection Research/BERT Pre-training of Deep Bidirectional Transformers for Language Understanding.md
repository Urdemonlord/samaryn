---
tags:
  - paper
  - bert
  - foundation
  - text-classification
---

# BERT Pre-training of Deep Bidirectional Transformers for Language Understanding

## Metadata

- Judul: BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding
- Tahun: 2018
- Link: https://arxiv.org/abs/1810.04805
- Tipe: Foundation model paper
- Domain: NLP, text classification

## Ringkasan inti

- Problem: banyak task NLP sebelumnya butuh task-specific architecture dan transfer learning yang kurang fleksibel.
- Tujuan: memperkenalkan pre-trained bidirectional encoder yang bisa di-fine-tune untuk banyak task.
- Kontribusi utama: fondasi teknis untuk classifier berbasis encoder, termasuk penggunaan [[IndoBERT]] untuk deteksi out-of-domain atau prompt berbahaya.

## Metode

- Setting atau skenario: pretraining language representation lalu fine-tuning ke task downstream.
- Model atau sistem yang diuji: BERT encoder architecture.
- Jenis serangan: tidak spesifik ke prompt injection.
- Jenis defense: tidak spesifik ke prompt injection.

## Dataset atau benchmark

- Nama dataset atau benchmark: multiple NLP tasks downstream.
- Ukuran: tidak menjadi fokus utama note ini.
- Sumber data: pretraining corpora dan benchmark NLP umum.
- Apakah publik: ya.

## Evaluasi

- Metric utama: akurasi dan task-specific NLP metrics.
- Baseline: model pretraining sebelumnya.
- Hasil penting: menunjukkan encoder pretraining efektif untuk klasifikasi teks dan transfer lintas task.

## Kelebihan

- Fondasi kuat untuk task klasifikasi.
- Sangat relevan untuk desain detector berbasis encoder.

## Keterbatasan

- Bukan paper keamanan.
- Tidak membahas prompt injection atau OOD secara langsung.

## Relevansi ke topik saya

- Menjadi basis teoritis untuk classifier [[IndoBERT]] di dalam [[Samaryn AI Security Gateway]].
- Mendukung pendekatan deteksi sebelum request diteruskan ke LLM.

## Koneksi ke note lain

- [[IndoBERT]]
- [[Out-of-Domain Detection]]
- [[Samaryn Security Reading List]]

