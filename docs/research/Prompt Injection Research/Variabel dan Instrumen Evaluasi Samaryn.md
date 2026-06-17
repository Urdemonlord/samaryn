---
tags:
  - evaluasi
  - variabel
  - instrumen
  - samaryn
---

# Variabel dan Instrumen Evaluasi Samaryn

## Variabel penelitian

### Variabel independen

| Variabel | Keterangan |
|---|---|
| jenis dataset | in-domain, prompt injection, out-of-domain |
| skema label | biner atau multikelas |
| threshold detector | batas keputusan allow, block, atau escalate |
| konfigurasi fine-tuning | epoch, learning rate, batch size |
| rule engine patterns | daftar pola eksplisit yang dipakai pada filtering awal |

### Variabel dependen

| Variabel | Keterangan |
|---|---|
| accuracy | ketepatan prediksi keseluruhan |
| precision | proporsi prediksi positif yang benar |
| recall | proporsi data positif yang berhasil dideteksi |
| F1-score | keseimbangan precision dan recall |
| false positive rate | tingkat salah blok pada data aman |
| false negative rate | tingkat lolosnya data berbahaya |
| AUROC | pemisahan kelas pada skenario biner jika digunakan |

## Instrumen evaluasi

| Instrumen | Fungsi |
|---|---|
| confusion matrix | melihat distribusi prediksi antar kelas |
| classification report | precision, recall, F1-score per kelas |
| dataset uji campuran | menilai performa deployment-oriented |
| threshold analysis | menentukan operating point yang realistis |
| error analysis table | menganalisis false positive dan false negative |

## Skenario uji

| Skenario | Tujuan |
|---|---|
| safe in-domain | memastikan request normal tidak terlalu sering diblok |
| prompt injection | memastikan serangan dapat dikenali |
| out-of-domain | memastikan input anomali dapat ditandai |
| mixed traffic | menguji performa pada distribusi yang lebih realistis |

## Kriteria keberhasilan

| Aspek | Indikator |
|---|---|
| deteksi prompt injection | recall tinggi pada data berbahaya |
| kestabilan OOD | performa tidak turun drastis pada domain baru |
| kelayakan deployment | false positive tetap terkontrol |
| kontribusi sistem | detector bisa diintegrasikan ke [[Samaryn AI Security Gateway]] |

## Terkait

- [[Bab 3 Metodologi Samaryn Draft]]
- [[Bab 4 Hasil Penelitian dan Pembahasan Samaryn Draft]]
- [[Gate AI LLM Security Benchmark Evaluation Methodology and Results]]

