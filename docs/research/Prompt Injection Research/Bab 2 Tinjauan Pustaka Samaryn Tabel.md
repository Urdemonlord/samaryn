---
tags:
  - bab2
  - draft
  - skripsi
  - samaryn
  - tabel
---

# Bab 2 Tinjauan Pustaka Samaryn Tabel

## Judul penelitian

[[Judul Penelitian Samaryn]]

## Tinjauan pustaka dalam bentuk tabel

| No | Topik | Sumber utama | Inti pembahasan | Relevansi ke penelitian |
|---|---|---|---|---|
| 1 | Fondasi model encoder | [[BERT Pre-training of Deep Bidirectional Transformers for Language Understanding]] | BERT memperkenalkan pre-trained bidirectional encoder yang dapat di-fine-tune untuk berbagai task klasifikasi teks. | Menjadi dasar teoritis penggunaan [[IndoBERT]] untuk deteksi prompt injection dan [[Out-of-Domain Detection]]. |
| 2 | Model bahasa Indonesia | [[IndoBERT]] | IndoBERT relevan untuk klasifikasi teks berbahasa Indonesia dan dapat dipakai sebagai detector pada lapisan awal sistem. | Menjadi engine utama deteksi pada penelitian ini. |
| 3 | Prompt injection sebagai ancaman | [[Prompt Injection]] dan [[Formalizing and Benchmarking Prompt Injection Attacks and Defenses]] | Prompt injection memanipulasi perilaku model melalui instruksi berbahaya, dan telah diformalkan sebagai masalah evaluasi sistematis. | Memberi definisi ancaman utama yang harus ditangani Samaryn. |
| 4 | Indirect prompt injection | [[Indirect Prompt Injection]] dan [[BIPIA]] | Serangan dapat disisipkan ke konten eksternal seperti email, web, tabel, atau konteks lain yang dibaca model. | Menjelaskan bahwa filtering input pengguna saja tidak cukup. |
| 5 | Prompt injection pada agent | [[InjecAgent]] | Tool-integrated agents rentan terhadap prompt injection yang bisa memicu aksi berbahaya. | Relevan untuk menempatkan Samaryn di depan sistem agentic dan tool-use. |
| 6 | Serangan realistis adaptif | [[LLMail-Inject]] | Dataset dari challenge publik menunjukkan attacker nyata dapat menyesuaikan strategi serangan terhadap email assistant. | Mendukung kebutuhan detector yang tidak hanya diuji pada benchmark statis. |
| 7 | Agentic attack surface | [[ReAct Synergizing Reasoning and Acting in Language Models]] dan [[Toolformer Language Models Can Teach Themselves to Use Tools]] | LLM modern tidak hanya menjawab, tetapi juga bernalar, memanggil tool, dan melakukan aksi. | Memperkuat alasan mengapa gateway keamanan dibutuhkan sebelum request mencapai model atau tool. |
| 8 | Routing dan pipeline keamanan | [[FrugalGPT How to Use Large Language Models While Reducing Cost and Improving Performance]] | Konsep cascade atau routing dapat digunakan untuk menyusun pemeriksaan bertahap. | Mendukung arsitektur rule engine, classifier [[IndoBERT]], lalu escalation layer. |
| 9 | Adaptive detector allocation | [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]] | Tiap detector memiliki blind spot, sehingga perlu alokasi detector yang adaptif sesuai karakteristik request. | Menjadi referensi terdekat untuk desain hybrid defense pada Samaryn. |
| 10 | Ancaman integrasi enterprise | [[AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations]] | Integrasi seperti Gmail, Jira, dan Salesforce dapat menjadi medium indirect prompt injection. | Relevan untuk positioning Samaryn sebagai AI security gateway di lingkungan enterprise. |
| 11 | Kontrol persisten pada agent | [[From Prompt Injection to Persistent Control Defending Agentic Harness Against Trojan Backdoors]] | Serangan prompt injection dapat berkembang menjadi kontrol persisten melalui file, tool output, atau state workspace. | Menunjukkan bahwa proteksi perlu mencakup audit dan pemeriksaan state, bukan hanya request tunggal. |
| 12 | Evaluasi detector security | [[Gate AI LLM Security Benchmark Evaluation Methodology and Results]] | Evaluasi detector harus memperhatikan threshold global, leakage, dan generalisasi lintas benchmark. | Menjadi dasar metodologi evaluasi detector pada penelitian ini. |
| 13 | Out-of-domain detection | [[Out-of-Domain Detection]] dan [[Controlling Out-of-Domain Gaps in LLMs for Genre Classification and Generated Text Detection]] | Domain shift dapat menurunkan performa model, sehingga perlu mekanisme untuk mengenali input di luar distribusi data normal. | Menjadi inti kebaruan penelitian, yaitu deteksi out-of-domain prompt injection. |
| 14 | Landscape autonomous agents | [[A Survey on Large Language Model based Autonomous Agents]] | Sistem agent berbasis LLM berkembang menuju otonomi, interaksi, dan integrasi eksternal yang lebih luas. | Memberi konteks teoritis mengapa topik keamanan agentic systems semakin penting. |
| 15 | Benchmark domain khusus | [[MPIB]] | Prompt injection pada domain medis menunjukkan bahwa dampak serangan perlu dinilai bukan hanya dari keberhasilan instruksi, tetapi juga dari harm yang ditimbulkan. | Menambah perspektif bahwa evaluasi keamanan idealnya mempertimbangkan dampak, bukan hanya klasifikasi benar atau salah. |

## Tabel tinjauan studi atau state-of-the-art

| No | Penelitian | Masalah | Metode atau pendekatan | Hasil utama | Perbedaan dengan penelitian ini |
|---|---|---|---|---|---|
| 1 | [[BIPIA]] | LLM rentan terhadap indirect prompt injection pada berbagai task berbasis konten eksternal. | Benchmark indirect prompt injection dan evaluasi defense. | Menyediakan benchmark foundational untuk menguji kerentanan dan mitigasi. | Penelitian ini tidak membangun benchmark umum, tetapi detector gateway berbasis [[IndoBERT]] untuk konteks Indonesia dan OOD. |
| 2 | [[InjecAgent]] | Agent dengan tool-use dapat dipaksa melakukan aksi berbahaya melalui prompt injection. | Benchmark tool-integrated agent attacks. | Menunjukkan prompt injection dapat memicu tindakan, bukan hanya salah jawaban. | Penelitian ini fokus pada lapisan proteksi sebelum request mencapai agent atau tool. |
| 3 | [[LLMail-Inject]] | Defense terhadap email-based prompt injection sering lemah terhadap attacker adaptif. | Dataset challenge realistis dari serangan adaptif. | Menunjukkan pentingnya evaluasi pada serangan nyata yang terus berubah. | Penelitian ini memanfaatkan ide realisme serangan untuk mengevaluasi detector OOD. |
| 4 | [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]] | Detector tunggal memiliki blind spot pada request berbeda. | Adaptive detector allocation. | Defense lebih efektif bila detector dialokasikan secara adaptif. | Penelitian ini mengambil inspirasi arsitektur hybrid, tetapi menempatkan [[IndoBERT]] sebagai engine inti. |
| 5 | [[Controlling Out-of-Domain Gaps in LLMs for Genre Classification and Generated Text Detection]] | Domain shift menurunkan performa model klasifikasi atau deteksi. | Pengendalian OOD gap. | Menunjukkan pentingnya robustness terhadap distribusi baru. | Penelitian ini menerapkan gagasan OOD ke prompt injection berbahasa Indonesia dalam skenario gateway. |

## Sintesis pustaka

| Aspek | Temuan utama | Implikasi untuk penelitian |
|---|---|---|
| Model dasar | Encoder seperti BERT efektif untuk klasifikasi teks. | [[IndoBERT]] layak dipakai sebagai basis detector. |
| Ancaman | Prompt injection dan indirect prompt injection nyata pada LLM dan agents. | Sistem butuh proteksi sebelum request mencapai model utama. |
| Agentic systems | Tool-use memperbesar konsekuensi serangan. | Samaryn perlu diposisikan sebagai gateway, bukan sekadar classifier mandiri. |
| Defense design | Detector tunggal punya blind spot. | Arsitektur berlapis lebih kuat daripada satu filter statis. |
| Evaluasi | Threshold dan generalisasi sangat penting. | Pengujian harus realistis dan lintas distribusi. |
| OOD | Domain shift menurunkan performa detector. | Fokus penelitian dapat diarahkan pada deteksi out-of-domain prompt injection. |

## Posisi penelitian dalam tabel

| Komponen | Posisi penelitian ini |
|---|---|
| Objek | request tekstual menuju sistem AI atau LLM |
| Masalah utama | deteksi prompt injection dan input out-of-domain |
| Pendekatan | classifier [[IndoBERT]] di dalam [[Samaryn AI Security Gateway]] |
| Arsitektur | rule engine, detector, escalation |
| Kontribusi | deteksi out-of-domain prompt injection berbahasa Indonesia |
| Nilai tambah | relevan untuk deployment security gateway pada agentic systems |

## Kerangka pemikiran dalam tabel

| Tahap logika | Isi |
|---|---|
| Masalah awal | Sistem AI modern terhubung dengan tool dan integrasi eksternal, sehingga rentan terhadap [[Prompt Injection]] dan [[Indirect Prompt Injection]]. |
| Kesenjangan | Banyak benchmark membuktikan ancaman, tetapi masih terbatas solusi gateway berbahasa Indonesia yang fokus pada deteksi out-of-domain prompt injection. |
| Dasar teori | Encoder model seperti [[BERT Pre-training of Deep Bidirectional Transformers for Language Understanding]] mendukung klasifikasi teks; [[Controlling Out-of-Domain Gaps in LLMs for Genre Classification and Generated Text Detection]] mendukung pentingnya OOD handling. |
| Pendekatan | Menggunakan [[IndoBERT]] sebagai detector utama di dalam [[Samaryn AI Security Gateway]]. |
| Mekanisme sistem | Request diperiksa oleh rule engine, lalu classifier, lalu escalation untuk kasus ambigu. |
| Hasil yang diharapkan | Sistem dapat memfilter request aman, mendeteksi prompt injection, dan mengenali input di luar domain normal. |
| Kontribusi | Memberi rancangan security gateway dan detector OOD prompt injection yang relevan untuk deployment agentic systems. |

## Terkait

- [[Judul Penelitian Samaryn]]
- [[Bab 1 Latar Belakang Samaryn Draft]]
- [[Bab 3 Metodologi Samaryn Draft]]
- [[Bab 4 Hasil Penelitian dan Pembahasan Samaryn Draft]]
- [[Samaryn Paper Review Matrix]]
