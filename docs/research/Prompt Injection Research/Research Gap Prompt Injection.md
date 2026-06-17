---
tags:
  - research-gap
  - prompt-injection
---

# Research Gap Prompt Injection

Note ini merangkum celah riset yang terlihat dari [[LLMail-Inject]], [[BIPIA]], [[InjecAgent]], [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]], dan [[MPIB]].

## Gap utama

### 1. Kesenjangan antara benchmark terstruktur dan serangan dunia nyata

[[BIPIA]] kuat untuk evaluasi terkontrol, tetapi banyak skenario masih dibangun secara sistematis dan tidak sepenuhnya menangkap adaptasi attacker manusia.

Sebaliknya, [[LLMail-Inject]] menangkap perilaku attacker nyata dan adaptif, tetapi domainnya masih sempit pada email assistant.

Gap:
- belum ada benchmark umum skala besar yang sekaligus realistis, adaptif, dan lintas domain

### 2. Agent benchmark masih sempit dibanding ekosistem agent modern

[[InjecAgent]] sudah relevan untuk tool-using agents, tetapi ekosistem agent saat ini melibatkan memory, browser automation, multi-agent workflows, long-horizon planning, dan tool chains yang lebih kompleks.

Gap:
- masih kurang benchmark untuk multi-step agent workflows
- masih kurang benchmark untuk memory persistence dan cross-tool contamination
- masih kurang benchmark untuk browser agents dan autonomous web actions

### 3. Evaluasi sering berat di ASR, ringan di harm

Banyak benchmark menilai keberhasilan serangan melalui attack success rate atau kepatuhan model terhadap instruksi berbahaya.

[[MPIB]] menonjol karena menambahkan CHER untuk mengukur dampak klinis.

Gap:
- masih sedikit benchmark yang mengukur harm nyata, bukan hanya compliance
- perlu metric yang menangkap financial harm, privacy harm, operational harm, dan safety harm

### 4. Domain coverage masih belum merata

[[MPIB]] memberi contoh benchmark spesifik domain medis, tetapi domain lain yang sensitif juga penting.

Gap:
- benchmark prompt injection untuk legal, finance, education, enterprise productivity, dan cybersecurity masih terbatas

### 5. Defense evaluation terhadap adaptive attackers masih kurang matang

[[LLMail-Inject]] menunjukkan pentingnya adaptive attack data. Banyak defense terlihat baik pada benchmark statis, tetapi belum tentu tahan terhadap attacker yang belajar dan menyesuaikan strategi.

Gap:
- perlu protokol evaluasi defense dengan adaptive adversary
- perlu benchmark yang mendukung red-teaming berulang

### 6. RAG evaluation belum cukup luas

[[MPIB]] sudah menyentuh direct dan RAG-mediated injection, tetapi RAG modern memakai retrieval pipeline yang jauh lebih kompleks.

Gap:
- masih kurang benchmark yang menguji retrieval poisoning, ranking manipulation, context packing, dan chunk-level instruction placement

## Rumusan gap yang bisa dipakai

### Versi singkat

Penelitian terdahulu telah menyediakan dataset, benchmark, dan framework untuk evaluasi prompt injection, namun masih terdapat kesenjangan pada benchmark yang realistis, adaptif, lintas domain, dan mampu mengukur dampak nyata serangan pada agentic dan RAG-based systems.

### Versi lebih akademik

Meskipun studi terdahulu seperti [[BIPIA]], [[InjecAgent]], [[LLMail-Inject]], [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]], dan [[MPIB]] telah memperkuat fondasi evaluasi prompt injection, literatur masih belum menyediakan kerangka evaluasi yang secara simultan memenuhi empat kebutuhan utama, yaitu realisme serangan adaptif, cakupan lintas domain, representasi agentic workflows modern, dan pengukuran outcome-level harm di luar attack success rate.

## Ide arah penelitian

- membangun benchmark prompt injection untuk browser-use agents
- membangun benchmark lintas domain dengan attacker manusia atau adaptive generation
- menambahkan harm-aware metric di luar ASR
- membandingkan defense statis vs defense terhadap adaptive attacks
- mengevaluasi prompt injection pada RAG pipeline end-to-end

## Terkait

- [[Literature Review Prompt Injection Benchmark]]
- [[Related Work Prompt Injection]]
- [[Prompt Injection Benchmark]]
- [[Prompt Injection Dataset]]

