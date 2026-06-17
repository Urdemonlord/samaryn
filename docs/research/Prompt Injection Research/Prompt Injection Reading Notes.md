---
tags:
  - reading-notes
  - prompt-injection
---

# Prompt Injection Reading Notes

## Urutan baca yang efisien

1. [[LLMail-Inject]]
2. [[BIPIA]]
3. [[InjecAgent]]
4. [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]]
5. [[MPIB]]

## Kenapa urutan ini

[[LLMail-Inject]] memberi gambaran paling konkret tentang dataset serangan nyata.

[[BIPIA]] memberi benchmark umum yang lebih sistematis dan luas.

[[InjecAgent]] mempersempit fokus ke tool-using agents yang sangat relevan untuk sistem modern.

[[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] membantu menyusun terminologi, framing, dan metodologi evaluasi.

[[MPIB]] cocok dibaca setelah fondasi umum kuat, karena ia memperkenalkan domain-specific safety view.

## Pertanyaan saat membaca

- Apakah paper ini menyediakan dataset publik, benchmark, atau hanya framework?
- Apakah setting-nya chat biasa, external content, RAG, atau agent + tools?
- Apakah evaluasinya mengukur ASR saja, atau juga dampak seperti harm?
- Seberapa realistis attacker model yang dipakai?
- Apakah benchmark-nya bisa direproduksi untuk penelitian lanjutan?

## Terkait

- [[Literature Review Prompt Injection Benchmark]]
- [[Prompt Injection Benchmark Graph]]
