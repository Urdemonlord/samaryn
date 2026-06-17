---
tags:
  - related-work
  - prompt-injection
---

# Related Work Prompt Injection

## Draft narasi

Prompt injection merupakan salah satu isu keamanan utama pada aplikasi berbasis large language model karena model dapat diarahkan untuk mengikuti instruksi berbahaya yang disisipkan oleh pihak penyerang. Dalam perkembangannya, fokus penelitian bergeser dari prompt injection pada input langsung menuju [[Indirect Prompt Injection]], yaitu serangan yang menyisipkan instruksi ke dalam konten eksternal seperti email, halaman web, tabel, atau konteks retrieval.

Salah satu benchmark awal yang banyak dirujuk adalah [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]], yang memformalkan prompt injection sebagai masalah evaluasi sistematis dan menyediakan platform untuk membandingkan attack maupun defense pada berbagai model dan task. Karya ini penting sebagai fondasi metodologis karena memperjelas ruang desain serangan dan pertahanan, meskipun fokus utamanya masih pada kerangka evaluasi umum.

Untuk skenario indirect prompt injection yang lebih spesifik, [[BIPIA]] memperkenalkan benchmark terstruktur untuk menguji ketahanan LLM terhadap instruksi berbahaya yang tertanam pada konten eksternal. Benchmark ini mencakup beberapa task seperti email QA, web QA, table QA, summarization, dan code QA, sehingga memberikan cakupan evaluasi yang cukup luas pada aplikasi LLM berbasis konten.

Seiring munculnya sistem agentic, kebutuhan evaluasi bergeser ke agent yang tidak hanya menghasilkan teks tetapi juga menggunakan tools dan melakukan aksi. Dalam konteks ini, [[InjecAgent]] menawarkan benchmark yang dirancang khusus untuk tool-integrated LLM agents. Benchmark ini menunjukkan bahwa prompt injection tidak lagi hanya memengaruhi kualitas jawaban, tetapi juga dapat memicu tindakan berbahaya seperti data exfiltration atau aksi yang merugikan pengguna. Perspektif ini dapat dipadukan dengan [[ReAct Synergizing Reasoning and Acting in Language Models]] dan [[Toolformer Language Models Can Teach Themselves to Use Tools]] untuk menjelaskan mengapa modern LLM systems membutuhkan lapisan keamanan sebelum aksi benar-benar dijalankan.

Di sisi lain, [[LLMail-Inject]] menghadirkan perspektif yang lebih realistis melalui dataset hasil challenge publik, di mana partisipan secara adaptif berusaha menyisipkan instruksi berbahaya ke dalam email untuk memicu tool calls yang tidak sah. Dibanding benchmark statis, kontribusi utama karya ini terletak pada kemampuannya merepresentasikan dinamika attacker nyata dan mengevaluasi defense terhadap serangan yang terus beradaptasi.

Untuk domain berisiko tinggi, [[MPIB]] memperluas evaluasi prompt injection ke ranah medis. Benchmark ini penting karena tidak hanya mengukur attack success rate, tetapi juga dampak klinis melalui metrik Clinical Harm Event Rate. Dengan demikian, penelitian ini menegaskan bahwa keberhasilan serangan tidak selalu identik dengan tingkat bahaya yang ditimbulkan, terutama pada domain safety-critical.

Secara keseluruhan, literatur yang ada telah menyediakan fondasi kuat dalam bentuk framework, benchmark, dan dataset untuk evaluasi prompt injection. Namun, masih terdapat kebutuhan akan benchmark yang lebih realistis, lintas domain, adaptif, serta mampu merepresentasikan agent workflows modern dan mengukur outcome-level harm secara lebih langsung. Kebutuhan inilah yang membuka ruang bagi penelitian lanjutan pada topik [[Prompt Injection Benchmark]] dan [[Prompt Injection Dataset]].

## Versi poin cepat

- [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] memberi fondasi metodologis.
- [[BIPIA]] memberi benchmark umum untuk indirect prompt injection.
- [[InjecAgent]] memindahkan evaluasi ke tool-using agents.
- [[LLMail-Inject]] memberi dataset realistis dari adaptive attack challenge.
- [[MPIB]] menambahkan perspektif domain medis dan harm-aware evaluation.

## Terkait

- [[Literature Review Prompt Injection Benchmark]]
- [[Research Gap Prompt Injection]]
- [[Prompt Injection Reading Notes]]
