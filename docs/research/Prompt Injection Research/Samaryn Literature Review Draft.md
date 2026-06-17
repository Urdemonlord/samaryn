---
tags:
  - draft
  - literature-review
  - samaryn
---

# Samaryn Literature Review Draft

Draft ini diambil dari teks yang kamu tambahkan dan diposisikan untuk skripsi atau proposal dengan fokus [[Samaryn AI Security Gateway]] dan engine deteksi berbasis [[IndoBERT]].

## Tinjauan pustaka

Perkembangan Large Language Models (LLMs) telah mendorong lahirnya berbagai aplikasi berbasis agen cerdas, termasuk chatbot, asisten kerja, dan agen percakapan yang terhubung dengan tool eksternal. Namun, peningkatan kemampuan ini juga memperluas permukaan serangan keamanan. Salah satu ancaman yang paling menonjol adalah [[Prompt Injection]], yaitu upaya menyisipkan instruksi berbahaya atau manipulatif ke dalam input maupun konteks eksternal agar model menyimpang dari tujuan sistem. Pada sistem agentic, ancaman ini menjadi lebih serius karena model tidak hanya menghasilkan teks, tetapi juga dapat mengambil tindakan, memanggil tool, membaca file, atau mengakses sistem pihak ketiga.

[[ReAct Synergizing Reasoning and Acting in Language Models]] menunjukkan bahwa model bahasa dapat menggabungkan proses penalaran dan tindakan secara interleaved untuk menyelesaikan tugas secara lebih efektif. Pendekatan ini meningkatkan utilitas sistem, tetapi sekaligus membuka jalur baru bagi serangan, karena keluaran model tidak lagi berhenti pada jawaban, melainkan dapat memicu aksi lanjutan. Temuan ini diperkuat oleh [[Toolformer Language Models Can Teach Themselves to Use Tools]], yang menunjukkan bahwa language model dapat belajar menggunakan API eksternal, memilih kapan tool dipanggil, dan bagaimana hasil tool dipakai kembali dalam inferensi berikutnya. Dengan demikian, integrasi tool memperbesar kebutuhan akan lapisan kontrol keamanan sebelum permintaan diteruskan ke model maupun ke tool.

Dalam konteks ancaman nyata, prompt injection tidak selalu datang langsung dari input pengguna. [[AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations]] menyoroti ancaman indirect prompt injection pada agent yang membaca data dari integrasi enterprise seperti Gmail, Jira, atau Salesforce. Pada skenario tersebut, konten dari pihak ketiga yang tampak biasa dapat membawa instruksi tersembunyi yang memengaruhi keputusan model. Temuan ini sangat relevan bagi sistem keamanan seperti [[Samaryn AI Security Gateway]], karena menunjukkan bahwa validasi input pengguna saja tidak cukup; sistem juga harus memeriksa konten dari integrasi dan jalur tool-use.

## Catatan

Teks sumber yang ditempel berhenti di bagian ini, jadi draft ini saat ini masih parsial dan bisa dilanjutkan nanti.

## Terkait

- [[Samaryn Security Reading List]]
- [[Related Work Prompt Injection]]
- [[Research Gap Prompt Injection]]
