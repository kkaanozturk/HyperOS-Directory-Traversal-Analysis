
<div align="center">

# 🔬 CVE-2025-21082: HyperOS AVCodec Use-After-Free

[![Lisans: MIT](https://img.shields.io/badge/Lisans-MIT-green.svg?style=for-the-badge)](LICENSE)
[![Python](https://img.shields.io/badge/Python-3.8%2B-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://python.org)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-CE422B?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![CVE](https://img.shields.io/badge/CVE-2025--21082-red?style=for-the-badge&logo=security&logoColor=white)](https://cve.mitre.org)
[![CVSS](https://img.shields.io/badge/CVSS-8.1%20High-orange?style=for-the-badge)](https://nvd.nist.gov)

**Xiaomi HyperOS AVCodec Medya Framework'ündeki Use-After-Free Zafiyetinin Kapsamlı Analizi ve Rust ile Simülasyonu**

*Üniversite Final Ödevi — Siber Güvenlik Araştırma Projesi*

---

🌐 **[Canlı Simülasyonu Tarayıcıda Deneyimleyin](https://kkaanozturk.github.io/HyperOS-Directory-Traversal-Analysis/simulation.html)**

---

<video src="demo/project-demo.webm" width="800" controls></video>

---

</div>

## 📖 Proje Hakkında

Bu repository, Xiaomi **HyperOS AVCodec** medya işleme framework'ünde tespit edilen kritik bir **Use-After-Free (UAF)** zafiyeti olan **CVE-2025-21082**'nin derinlemesine teknik analizini, saldırı mekanizmasının simülasyonunu ve çözüm önerilerini içmektedir.

Zafiyetin temel sebebi, AVCodec'in asenkron callback mekanizmasında codec context'inin, worker thread'ler hâlâ çalışırken serbest bırakılmasıdır. Bu durum klasik bir **Use-After-Free race condition**'a yol açmakta ve teorik olarak **Uzaktan Kod Çalıştırma (RCE)** imkânı sunmaktadır.

Proje; Rust ile yazılmış güvenli bir UAF simülasyonu, kapsamlı teknik dokümantasyon, GitHub Actions CI boru hattı otomasyonu ve tarayıcı tabanlı interaktif bir görselleştirme içermektedir.

---

## 📂 Depo Yapısı


```

HyperOS-Directory-Traversal-Analysis/
│
├── 📁 .github/                     # GitHub topluluk ve CI/CD iş akışları
│   ├── 📁 ISSUE_TEMPLATE/          # Hata ve özellik talep şablonları
│   └── 📁 workflows/
│       └── 📄 rust.yml             # Otomatik test ve derleme boru hattı (CI)
│
├── 📁 assets/                      # Görsel analiz dokümanları
│   └── 🌐 infographic.html         # Görsel zafiyet şeması
│
├── 📁 docs/                        # Teknik raporlar ve süreç takibi
│   ├── 📄 zafiyet-analizi.md       # Zafiyet analizi ve CVSS skorlaması
│   ├── 📄 mimari-analiz.md         # HyperOS AVCodec mimari şeması
│   ├── 📄 cozum-onerileri.md       # Çözüm önerileri ve C++ yamaları
│   ├── 📄 plan.md                  # Proje zaman çizelgesi (Süreç Notu)
│   ├── 📄 SORULAR.md               # Süreç notları ve teknik sorular
│   ├── 📄 simple.md                # Taslak çalışma notları
│   └── 📄 README.md                # Dokümantasyon rehber indeksi
│
├── 📁 poc_python/                  # Python analiz araçları
│   ├── 📄 exploit.py               # CVE-2025-2844 Directory Traversal (Referans PoC)
│   └── 📄 requirements.txt         # Gerekli kütüphaneler
│
├── 📁 poc_rust/                    # Rust UAF simülasyonu (Ana PoC)
│   ├── 📁 src/
│   │   └── 📄 main.rs              # Unsafe Rust ile UAF simülasyon mantığı
│   ├── 📄 Cargo.toml
│   └── 📄 Cargo.lock
│
├── 🌐 simulation.html              # İnteraktif web simülasyon paneli (GitHub Pages)
├── 📄 README.md                    # Bu dosya (Ana Vitrin)
├── 📄 SECURITY.md                  # Güvenlik Politikası ve Sorumlu Açıklama Kuralları
├── 📄 CODE_OF_CONDUCT.md           # Evrensel Topluluk Davranış Kuralları
├── 📄 CONTRIBUTING.md              # Açık Kaynak Katkıda Bulunma Rehberi
├── 📋 TODO.md                      # Aktif görev takip listesi
└── ⚖️ LICENSE                       # MIT Lisans Dosyası

```

---

## 🧠 Zafiyet Özeti

| Özellik | Detay |
| :--- | :--- |
| **CVE Numarası** | CVE-2025-21082 |
| **Zafiyet Türü** | Use-After-Free (CWE-416) |
| **Etkilenen Bileşen** | Xiaomi HyperOS AVCodec Framework |
| **CVSS v3.1 Skoru** | **8.1 (High)** |
| **Saldırı Vektörü** | Ağ (Network) |
| **Etki** | Uzaktan Kod Çalıştırma (RCE) potansiyeli |
| **Keşif Tarihi** | 10 Şubat 2025 |
| **Yama Tarihi** | 20 Şubat 2025 |

### Zafiyetin Özü


```

[Ana Thread]   processFrameAsync() → Worker thread başlatılır
↓
release() çağrılır → Bellek SERBEST BIRAKILIR ⚠️
↓
[Worker Thread]  Serbest bırakılan belleğe erişmeye devam eder → UAF 💥

```

---

## 🎬 Demo

Projenin derlenmesini, çalıştırılmasını ve Use-After-Free simülasyon çıktısını gösteren ekran kaydını aşağıdan izleyebilirsiniz:

<div align="center">
  <img src="https://github.com/user-attachments/assets/9553a67a-05e5-4d37-b218-1a5ab239c3f7" width="800">
</div>

---

## 🚀 Hızlı Başlangıç

### Gereksinimler

- **Rust** 1.70 veya üzeri → [rustup.rs](https://rustup.rs/)
- **Python** 3.8 veya üzeri
- Modern bir web tarayıcısı (Chrome, Firefox, Edge)

### 1. Rust PoC — UAF Simülasyonu

```bash
# Projeyi klonlayın
git clone [https://github.com/kkaanozturk/HyperOS-Directory-Traversal-Analysis.git](https://github.com/kkaanozturk/HyperOS-Directory-Traversal-Analysis.git)
cd HyperOS-Directory-Traversal-Analysis/poc_rust

# Release modunda derleyin
cargo build --release

```

#### 🔴 Zafiyetli Senaryo (Race Condition & UAF Gösterimi)

```bash
# Windows
.\target\release\poc_rust.exe --mode vulnerable --verbose

# Linux / macOS
./target/release/poc_rust --mode vulnerable --verbose

```

**Beklenen Çıktı:**

```
🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC
Mode: vulnerable

⚠️  Running vulnerable scenario...
🧵 Starting worker thread...
🗑️  Main thread releasing codec context (UAF trigger)...
🔄 Worker thread accessing codec context...
🚨 UAF tespit edildi! Magic: 0xDEADBEEF
💥 UAF vulnerability triggered on frame 0!

🚨 Vulnerable scenario completed - UAF demonstrated!
⚠️  In a real exploit, this could lead to RCE

```

#### 🟢 Yamalanmış Senaryo (Güvenli Senkronizasyon)

```bash
# Windows
.\target\release\poc_rust.exe --mode patched --verbose

# Linux / macOS
./target/release/poc_rust --mode patched --verbose

```

**Beklenen Çıktı:**

```
🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC
Mode: patched

✅ Running patched scenario...
🧵 Starting worker thread...
⏳ Waiting for worker thread to complete (patch applied)...
✅ Frame 0 processed successfully
✅ Frame 1 processed successfully
✅ Worker thread completed safely
🗑️  Safely releasing codec context...

✅ Patched scenario completed - No UAF occurred!
🛡️  Proper synchronization prevents the vulnerability

```

### 2. İnteraktif Web Simülasyonu

Yerel sunucu kurmakla uğraşmak istemiyorsanız doğrudan internet üzerinden **[GitHub Pages Canlı Önizleme](https://www.google.com/url?sa=E&source=gmail&q=https://kkaanozturk.github.io/HyperOS-Directory-Traversal-Analysis/simulation.html)** linkine tıklayarak simülasyonu tarayıcınızda deneyimleyebilirsiniz.

Yerel olarak çalıştırmak isterseniz:

```bash
# Proje kök dizininde basit bir HTTP sunucusu başlatın
python -m http.server 8000

# Tarayıcınızda açın: http://localhost:8000/simulation.html

```

Simülasyon **5 sahne** içermektedir:

1. 🧪 **Lab Ortamı** — AVCodec pipeline tanıtımı
2. 🏗️ **Context Oluşturma** — Heap bellek düzeni
3. ⚡ **Race Condition** — Zaman çizelgesi ve zafiyet penceresi
4. 💥 **UAF Sömürüsü** — Heap reclamation ve dangling pointer
5. 🛡️ **Patch Karşılaştırması** — Zafiyetli vs. yamalı kod

### 3. Python Analiz Araçları (Eski PoC - Referans Amaçlı)

> **Önemli Not**: Bu klasör, projenin orijinal CVE-2025-2844 (Directory Traversal) versiyonundan teknik tarihçe ve hibrit araştırma referansı olması adına korunmuştur. Mevcut ana çalışma olan CVE-2025-21082 (UAF) analizi için yukarıdaki Rust PoC'yi kullanın.

```bash
cd poc_python
python -m pip install -r requirements.txt
python exploit.py -u http://hedef:5000 -f etc/shadow

```

---

## 📚 Teknik Dokümantasyon

| Doküman | İçerik |
| --- | --- |
| 📊 [Zafiyet Analizi](https://www.google.com/search?q=docs/zafiyet-analizi.md) | CVE-2025-21082 teknik analizi, CVSS skorlaması, saldırı senaryoları, HyperOS vs AOSP karşılaştırması |
| 🏗️ [Mimari Şeması](https://www.google.com/search?q=docs/mimari-analiz.md) | AVCodec async pipeline, bileşen diyagramları, race condition akış şeması |
| 🛡️ [Çözüm Önerileri](https://www.google.com/search?q=docs/cozum-onerileri.md) | Zafiyetli/yamalı C++ kod örnekleri, RAII pattern, MTE, CFI, ASan entegrasyonu |
| 📋 [Süreç Takip Belgeleri](https://www.google.com/search?q=docs/README.md) | `plan.md`, `SORULAR.md` ve `simple.md` gibi geliştirme aşaması notlarının indeksi |

---

## 🔬 Teknik Detaylar

### Zafiyetli Kod Paterni (C++)

```cpp
class AVCodecContext {
    void processFrameAsync() {
        worker_thread_ = std::thread([this]() {
            buffer_->processFrame(); // Worker thread çalışıyor
        });
        // ❌ join() yok — race condition!
    }

    void release() {
        delete buffer_; // ❌ Worker hâlâ çalışırken bellek serbest bırakılıyor
        buffer_ = nullptr;
    }
};

```

### Yamalı Kod Paterni (C++)

```cpp
class AVCodecContext {
    void release() {
        shutdown_requested_ = true;
        worker_cv_.notify_all();

        if (worker_thread_.joinable()) {
            worker_thread_.join(); // ✅ Thread tamamlanana kadar bekle
        }

        delete buffer_; // ✅ Güvenli temizlik
    }
};

```

### Rust Simülasyonu — UAF Tespiti

```rust
unsafe fn process_frame(&mut self) -> bool {
    // Magic number bozulduysa veya değiştiyse UAF gerçekleşmiş demektir
    if self.magic != 0xDEADBEEF {
        println!("🚨 UAF tespit edildi! Magic: 0x{:08X}", self.magic);
        return false;
    }
    self.frame_counter += 1;
    true
}

```

---

## 🛡️ Savunma Stratejileri

| Yöntem | Açıklama | Etkinlik |
| --- | --- | --- |
| **Thread Senkronizasyonu** | `join()` ile thread tamamlanana kadar bekleme | ⭐⭐⭐⭐⭐ |
| **Referans Sayımı** | `shared_ptr` ile otomatik yaşam süresi yönetimi | ⭐⭐⭐⭐⭐ |
| **RAII Pattern** | Destructor ile otomatik kaynak temizliği | ⭐⭐⭐⭐ |
| **AddressSanitizer** | Derleme zamanı UAF tespiti | ⭐⭐⭐⭐ |
| **MTE (ARM64)** | Donanım seviyesi bellek etiketleme | ⭐⭐⭐⭐⭐ |
| **CFI** | Kontrol akışı bütünlüğü koruması | ⭐⭐⭐ |

---

## ⚠️ Yasal Uyarı

> **Bu proje yalnızca eğitim ve akademik araştırma amacıyla geliştirilmiştir.**
> Rust simülasyonu, zafiyetin mekanizmasını göstermek için tasarlanmış olup gerçek bir exploit **değildir**. Buradaki bilgilerin ve araçların yetkisiz sistemler üzerinde kullanılması yasal sorumluluk doğurabilir. Geliştirici hiçbir sorumluluk kabul etmez.

---

## 📄 Lisans

Bu proje **MIT Lisansı** ile lisanslanmıştır. Detaylar için [LICENSE](https://www.google.com/search?q=LICENSE) dosyasına bakınız.

---

*Siber güvenlik araştırması — Eğitim amaçlı hazırlanmıştır*

