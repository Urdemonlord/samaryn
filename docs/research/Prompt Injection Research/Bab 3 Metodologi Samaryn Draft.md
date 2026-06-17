---
tags:
  - bab3
  - draft
  - metodologi
  - samaryn
---

# Bab 3 Metodologi Samaryn Draft

## Judul penelitian

[[Judul Penelitian Samaryn]]

## Pendekatan penelitian

Penelitian ini menggunakan pendekatan eksperimen untuk mengembangkan dan mengevaluasi engine deteksi out-of-domain prompt injection berbasis [[IndoBERT]] yang diintegrasikan ke dalam [[Samaryn AI Security Gateway]]. Fokus utamanya adalah klasifikasi request berisiko dan identifikasi input yang berada di luar domain normal sistem, sehingga request tersebut dapat ditahan, ditandai, atau diekskalasi sebelum diteruskan ke model utama atau tool eksternal.

## Arsitektur sistem usulan

Arsitektur sistem usulan terdiri atas tiga lapisan utama:

1. `Rule Engine`
   Lapisan awal untuk mendeteksi pola eksplisit seperti kata kunci manipulatif, instruksi override, pola exfiltration, atau indicator prompt injection sederhana.

2. `IndoBERT Detector`
   Lapisan klasifikasi berbasis [[IndoBERT]] untuk menilai apakah request termasuk normal, suspicious, atau out-of-domain. Lapisan ini berfungsi sebagai engine utama penelitian.

3. `Escalation Layer`
   Lapisan keputusan lanjutan untuk request ambigu. Pada implementasi awal, lapisan ini dapat berupa manual review, rule tambahan, atau model evaluasi lain. Secara konseptual, lapisan ini dekat dengan pendekatan pada [[Send a SCOUT First Pre-hoc Reasoning for Adaptive Detector Allocation in Prompt-Injection Defense]].

## Alur sistem

1. Pengguna atau sumber eksternal mengirim request.
2. Request diperiksa oleh rule engine.
3. Jika tidak diblok oleh rule eksplisit, request diteruskan ke detector [[IndoBERT]].
4. Detector memberi label, misalnya `safe`, `suspicious`, atau `ood`.
5. Request dengan label aman diteruskan ke sistem target.
6. Request dengan label berisiko ditahan atau diekskalasi.

## Desain dataset

Desain dataset dapat dibangun dari tiga kelompok data:

1. `In-domain benign prompts`
   Prompt normal yang sesuai dengan domain sistem target.

2. `Prompt injection samples`
   Sampel prompt berbahaya, baik direct maupun indirect, yang diadaptasi dari benchmark seperti [[BIPIA]], [[InjecAgent]], [[LLMail-Inject]], dan sumber lain yang relevan.

3. `Out-of-domain samples`
   Input yang tidak berbahaya secara eksplisit tetapi berada di luar distribusi domain target, misalnya permintaan yang tidak sesuai konteks aplikasi, pola bahasa yang sangat berbeda, atau instruksi yang tidak relevan terhadap fungsi sistem.

## Strategi pelabelan

Skema label awal yang sederhana:

- `safe`
- `prompt_injection`
- `out_of_domain`

Jika diperlukan, label dapat diperluas menjadi:

- `safe`
- `direct_injection`
- `indirect_injection`
- `ood`
- `ambiguous`

## Tahapan eksperimen

1. Pengumpulan dan normalisasi data.
2. Penyusunan skema label.
3. Fine-tuning [[IndoBERT]] pada data pelatihan.
4. Validasi hyperparameter dan threshold.
5. Pengujian pada data uji in-domain dan out-of-domain.
6. Integrasi hasil klasifikasi ke dalam alur keputusan [[Samaryn AI Security Gateway]].

## Skenario evaluasi

### 1. Evaluasi klasifikasi dasar

Mengukur kemampuan model membedakan request aman, prompt injection, dan out-of-domain.

Metric yang dapat digunakan:

- accuracy
- precision
- recall
- F1-score

### 2. Evaluasi OOD robustness

Mengukur apakah model tetap mampu mengenali input di luar distribusi data latih.

Metric yang dapat digunakan:

- macro F1
- false positive rate
- false negative rate
- AUROC untuk skenario biner jika dipakai

### 3. Evaluasi deployment-oriented

Mengukur operating point detector dalam skenario lebih realistis, mengikuti semangat [[Gate AI LLM Security Benchmark Evaluation Methodology and Results]].

Aspek yang diamati:

- threshold global
- tradeoff antara blocking dan miss rate
- generalisasi lintas sumber data

## Baseline pembanding

Baseline yang dapat digunakan:

- rule-based detector saja
- model klasik seperti TF-IDF + SVM atau Logistic Regression
- [[IndoBERT]] tanpa skema OOD-aware thresholding

## Variabel penelitian

### Variabel independen

- jenis data latih
- skema label
- threshold klasifikasi
- konfigurasi fine-tuning [[IndoBERT]]

### Variabel dependen

- performa klasifikasi
- performa OOD detection
- efektivitas filtering di alur gateway

## Output penelitian

- model deteksi berbasis [[IndoBERT]]
- rancangan alur [[Samaryn AI Security Gateway]]
- hasil evaluasi robustness dan generalisasi

## Catatan implementasi

Jika penelitian harus tetap terukur dan sederhana, ruang lingkup implementasi dapat dibatasi pada detector tekstual level request, tanpa harus membangun seluruh agent runtime. Dengan begitu, kontribusi utama tetap jelas, yaitu deteksi out-of-domain prompt injection untuk lapisan awal gateway.

## Terkait

- [[Bab 2 Tinjauan Pustaka Samaryn Tabel]]
- [[Bab 1 Latar Belakang Samaryn Draft]]
- [[Bab 4 Hasil Penelitian dan Pembahasan Samaryn Draft]]
- [[Samaryn Dataset Benchmark Mapping]]
- [[Variabel dan Instrumen Evaluasi Samaryn]]
- [[Gate AI LLM Security Benchmark Evaluation Methodology and Results]]
