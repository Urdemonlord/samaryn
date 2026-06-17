---
tags:
  - draft
  - related-work
  - samaryn
  - skripsi
---

# Samaryn Formal Related Work Draft

## Tinjauan pustaka formal

Perkembangan large language models telah mendorong transformasi berbagai sistem cerdas menjadi lebih otonom, interaktif, dan terhubung dengan tool eksternal. Dalam konteks ini, model bahasa tidak lagi hanya berfungsi sebagai generator teks, tetapi juga sebagai komponen pengambil keputusan yang dapat memicu aksi lanjutan. Implikasi dari perubahan ini adalah meningkatnya attack surface pada sistem berbasis LLM, terutama ketika model diberi akses ke API, file, basis pengetahuan, maupun layanan pihak ketiga. Salah satu ancaman yang paling relevan pada konteks tersebut adalah [[Prompt Injection]], yaitu upaya menyisipkan instruksi manipulatif agar perilaku sistem menyimpang dari tujuan yang telah ditetapkan.

Kebutuhan akan lapisan keamanan sebelum model melakukan aksi dapat dijelaskan dari evolusi kapabilitas agentic systems itu sendiri. [[ReAct Synergizing Reasoning and Acting in Language Models]] menunjukkan bahwa model bahasa dapat menggabungkan proses penalaran dan tindakan secara interleaved untuk menyelesaikan tugas secara lebih efektif. Sementara itu, [[Toolformer Language Models Can Teach Themselves to Use Tools]] memperlihatkan bahwa model dapat belajar memilih kapan tool dipanggil dan bagaimana hasilnya digunakan kembali dalam inferensi. Kedua studi ini memperkuat argumen bahwa semakin besar kemampuan model untuk bertindak, semakin besar pula konsekuensi ketika input berbahaya berhasil memengaruhi proses pengambilan keputusan.

Pada sisi pertahanan sistem, desain lapisan kontrol tidak cukup dipahami hanya sebagai pemblokiran statis terhadap kata kunci berbahaya. [[FrugalGPT How to Use Large Language Models While Reducing Cost and Improving Performance]] memang tidak ditulis sebagai paper keamanan, tetapi menawarkan ide arsitektural penting berupa cascade dan routing. Gagasan ini relevan untuk membangun pipeline pemeriksaan bertahap, misalnya melalui rule engine sebagai penyaring awal, classifier berbasis [[IndoBERT]] untuk penilaian lanjutan, dan LLM judge atau escalation layer untuk kasus ambigu. Pendekatan serupa diperkuat secara lebih spesifik oleh [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]], yang memandang pertahanan prompt injection sebagai masalah detector allocation. Perspektif tersebut penting karena menunjukkan bahwa tidak ada satu detector tunggal yang optimal untuk semua request, sehingga strategi pertahanan perlu adaptif terhadap karakteristik input.

Ancaman prompt injection juga menjadi lebih kompleks ketika sistem LLM beroperasi sebagai agent yang berinteraksi dengan integrasi eksternal. [[AgentRedBench Dynamic Redteaming and Integration-Aware Defense for LLM Agents over SaaS Integrations]] menyoroti bahwa konten dari layanan enterprise seperti email, ticketing, atau CRM dapat menjadi medium untuk indirect prompt injection. Temuan ini memperlihatkan bahwa validasi input pengguna saja tidak memadai, karena sumber ancaman dapat muncul dari jalur integrasi yang tampak sah. Di samping itu, [[From Prompt Injection to Persistent Control Defending Agentic Harness Against Trojan Backdoors]] menunjukkan bahwa dampak serangan tidak selalu berhenti pada satu eksekusi prompt, tetapi dapat berkembang menjadi kontrol persisten melalui file, tool output, atau state workspace. Artinya, arsitektur keamanan perlu memperhatikan bukan hanya request-level filtering, tetapi juga audit log, origin tracing, dan pemeriksaan state jangka panjang.

Dari sisi metodologi evaluasi, kualitas detector keamanan tidak cukup diukur dengan akurasi pada satu dataset yang dituning secara terpisah. [[Gate AI LLM Security Benchmark Evaluation Methodology and Results]] menekankan pentingnya threshold global, pencegahan leakage, dan pengujian generalisasi lintas benchmark. Pendekatan ini relevan bagi penelitian yang ingin mengembangkan engine deteksi secara realistis, karena deployment nyata menuntut operating point yang stabil terhadap distribusi input yang berubah. Hal ini sejalan dengan kebutuhan [[Out-of-Domain Detection]], yaitu kemampuan sistem untuk mengenali request yang berada di luar distribusi data yang dipelajari. Dalam konteks tersebut, [[BERT Pre-training of Deep Bidirectional Transformers for Language Understanding]] menyediakan fondasi model encoder untuk klasifikasi teks, sedangkan [[Controlling Out-of-Domain Gaps in LLMs for Genre Classification and Generated Text Detection]] menguatkan bahwa domain shift dapat menurunkan performa model secara signifikan apabila tidak ditangani secara eksplisit.

Secara keseluruhan, literatur tersebut mendukung posisi [[Samaryn AI Security Gateway]] sebagai lapisan keamanan sebelum request mencapai model utama atau tool eksternal. Samaryn dapat dipahami sebagai sistem yang memadukan filtering awal, klasifikasi berbasis encoder, detector adaptif, dan mekanisme eskalasi untuk menghadapi prompt injection, indirect prompt injection, serta request out-of-domain pada sistem agentic modern. Dengan demikian, kontribusi penelitian dapat diarahkan tidak hanya pada pembuatan detector, tetapi juga pada perancangan pipeline pertahanan yang realistis, terukur, dan relevan dengan kebutuhan deployment.

## Posisi skripsi

Skripsi dapat diposisikan sebagai pengembangan engine deteksi out-of-domain prompt injection berbasis [[IndoBERT]] yang berfungsi sebagai salah satu lapisan inti dalam [[Samaryn AI Security Gateway]]. Nilai kebaruannya dapat ditegaskan pada kombinasi antara pendekatan klasifikasi teks berbahasa Indonesia, skenario prompt injection atau anomali domain, serta integrasinya ke dalam security gateway yang bersifat berlapis.

## Terkait

- [[Samaryn Security Reading List]]
- [[Samaryn Paper Review Matrix]]
- [[Research Gap Prompt Injection]]

