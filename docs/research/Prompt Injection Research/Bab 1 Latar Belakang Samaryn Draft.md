---
tags:
  - bab1
  - draft
  - skripsi
  - samaryn
---

# Bab 1 Latar Belakang Samaryn Draft

## Judul penelitian

[[Judul Penelitian Samaryn]]

## Latar belakang

Perkembangan large language models telah mempercepat adopsi sistem cerdas pada berbagai bidang, mulai dari chatbot, asisten produktivitas, hingga agen yang dapat menggunakan tool eksternal secara mandiri. Kemampuan ini menjadikan LLM semakin bermanfaat dalam membantu pengguna menyelesaikan tugas kompleks, tetapi pada saat yang sama juga meningkatkan risiko keamanan. Ketika model tidak lagi hanya menjawab pertanyaan, melainkan juga dapat memanggil API, membaca dokumen, mengambil keputusan, atau mengeksekusi tindakan lanjutan, maka kesalahan interpretasi input dapat berdampak lebih serius daripada sekadar keluaran teks yang keliru.

Salah satu ancaman yang paling menonjol dalam konteks tersebut adalah [[Prompt Injection]]. Serangan ini bertujuan menyisipkan instruksi berbahaya atau manipulatif agar model menyimpang dari tujuan awal sistem. Dalam sistem modern, serangan tidak selalu berasal langsung dari input pengguna, tetapi juga dapat muncul melalui email, halaman web, dokumen, tabel, atau konteks retrieval yang dibaca model. Bentuk serangan seperti ini dikenal sebagai [[Indirect Prompt Injection]] dan menjadi semakin penting ketika LLM diintegrasikan dengan tool eksternal atau enterprise systems.

Berbagai penelitian telah menunjukkan bahwa risiko prompt injection bukan sekadar persoalan teoretis. [[BIPIA]] menunjukkan kerentanan LLM terhadap indirect prompt injection pada berbagai task berbasis konten eksternal. [[InjecAgent]] memperluas temuan tersebut ke skenario tool-integrated agents, sedangkan [[LLMail-Inject]] menampilkan bagaimana serangan adaptif dari attacker nyata dapat memanipulasi email assistant. Di domain enterprise, [[AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations]] menegaskan bahwa integrasi seperti email, CRM, dan ticketing system dapat menjadi jalur serangan yang praktis. Sementara itu, [[From Prompt Injection to Persistent Control Defending Agentic Harness Against Trojan Backdoors]] menunjukkan bahwa serangan bahkan dapat berkembang menjadi kontrol persisten melalui file, tool output, atau state workspace.

Permasalahan ini menunjukkan bahwa sistem berbasis LLM membutuhkan lapisan keamanan sebelum request diteruskan ke model utama maupun tool eksternal. Dalam konteks tersebut, penelitian ini memosisikan [[Samaryn AI Security Gateway]] sebagai lapisan proteksi awal yang bertugas melakukan filtering, klasifikasi, deteksi anomali, dan eskalasi terhadap request yang berisiko. Pendekatan ini relevan dengan gagasan pipeline berlapis yang menggabungkan rule engine, classifier, dan mekanisme eskalasi adaptif. Secara konseptual, arsitektur seperti ini didukung oleh [[FrugalGPT How to Use Large Language Models While Reducing Cost and Improving Performance]] dan diperkuat secara keamanan oleh [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]].

Di sisi lain, tantangan penting dalam pengembangan detector keamanan adalah perubahan distribusi data. Input berbahaya tidak selalu memiliki pola yang sama dengan data pelatihan, sehingga model deteksi dapat mengalami penurunan performa ketika menghadapi request yang berada di luar domain yang sudah dipelajari. Masalah ini berkaitan erat dengan [[Out-of-Domain Detection]], yaitu kemampuan sistem mengenali input yang menyimpang dari distribusi normal. Dalam penelitian ini, pendekatan tersebut dipadukan dengan [[IndoBERT]] sebagai classifier berbasis encoder untuk bahasa Indonesia. Landasan penggunaan encoder model didukung oleh [[BERT Pre-training of Deep Bidirectional Transformers for Language Understanding]], sedangkan relevansi domain shift diperkuat oleh [[Controlling Out-of-Domain Gaps in LLMs for Genre Classification and Generated Text Detection]].

Dengan demikian, penelitian ini diarahkan untuk mengembangkan engine deteksi out-of-domain prompt injection berbasis [[IndoBERT]] yang diintegrasikan ke dalam [[Samaryn AI Security Gateway]]. Fokus penelitian bukan hanya pada pembuatan classifier, tetapi juga pada bagaimana classifier tersebut dapat berfungsi sebagai lapisan awal dalam pipeline keamanan yang lebih realistis untuk deployment. Melalui pendekatan tersebut, penelitian diharapkan dapat memberikan kontribusi baik pada sisi akademik, melalui penguatan metode deteksi prompt injection berbahasa Indonesia, maupun pada sisi praktis, melalui rancangan gateway keamanan yang relevan untuk sistem agentic modern.

## Identifikasi masalah

- Sistem LLM modern memiliki attack surface yang lebih luas karena terhubung dengan tool dan integrasi eksternal.
- [[Prompt Injection]] dan [[Indirect Prompt Injection]] dapat memanipulasi perilaku model sebelum maupun saat tool digunakan.
- Detector tunggal cenderung memiliki blind spot dan belum tentu robust terhadap variasi input berbahaya.
- Performa model deteksi dapat turun ketika menghadapi input di luar domain pelatihan.
- Masih terbatas pendekatan yang menggabungkan OOD detection berbahasa Indonesia dengan security gateway untuk agentic systems.

## Rumusan masalah

1. Bagaimana merancang engine deteksi out-of-domain prompt injection berbasis [[IndoBERT]] untuk bahasa Indonesia?
2. Bagaimana mengintegrasikan engine tersebut ke dalam [[Samaryn AI Security Gateway]] sebagai lapisan proteksi awal?
3. Bagaimana mengevaluasi performa engine deteksi agar tetap relevan terhadap perubahan distribusi input dan skenario prompt injection?

## Tujuan penelitian

1. Mengembangkan model deteksi out-of-domain prompt injection berbasis [[IndoBERT]].
2. Merancang integrasi model ke dalam arsitektur [[Samaryn AI Security Gateway]].
3. Mengevaluasi efektivitas model dengan skema yang mempertimbangkan robustness dan generalisasi.

## Manfaat penelitian

- Memberi dasar teknis untuk keamanan sistem LLM berbahasa Indonesia.
- Menyediakan rancangan gateway keamanan yang dapat dipakai pada sistem agentic atau tool-integrated.
- Menambah referensi akademik pada topik deteksi prompt injection dan OOD detection.

## Terkait

- [[Bab 2 Tinjauan Pustaka Samaryn Tabel]]
- [[Samaryn Formal Related Work Draft]]
- [[Bab 3 Metodologi Samaryn Draft]]
- [[Samaryn Dataset Benchmark Mapping]]
