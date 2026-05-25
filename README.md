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

<!-- GIF ALANI — Buraya demo GIF'inizi ekleyin -->
<!-- Örnek kullanım: -->
<!--
<img src="assets/demo.gif" alt="CVE-2025-21082 UAF Simülasyon Demosu" width="800"/>
-->

> 📌 **Demo GIF'i buraya ekleyin:** `assets/demo.gif` dosyasını oluşturup yukarıdaki yorum satırını aktif hale getirin.
> Önerilen boyut: 800×450px, maksimum 10MB

---

</div>

## 📖 Proje Hakkında

Bu repository, Xiaomi **HyperOS AVCodec** medya işleme framework'ünde tespit edilen kritik bir **Use-After-Free (UAF)** zafiyeti olan **CVE-2025-21082**'nin derinlemesine teknik analizini, saldırı mekanizmasının simülasyonunu ve çözüm önerilerini içermektedir.

Zafiyetin temel sebebi, AVCodec'in asenkron callback mekanizmasında codec context'inin, worker thread'ler hâlâ çalışırken serbest bırakılmasıdır. Bu durum klasik bir **Use-After-Free race condition**'a yol açmakta ve teorik olarak **Uzaktan Kod Çalıştırma (RCE)** imkânı sunmaktadır.

Proje; Rust ile yazılmış güvenli bir UAF simülasyonu, kapsamlı teknik dokümantasyon ve tarayıcı tabanlı interaktif bir görselleştirme içermektedir.

---

## 📂 Depo Yapısı

```
HyperOS-Directory-Traversal-Analysis/
│
├── 📁 docs/                        # Teknik dokümantasyon
│   ├── analysis.md                 # Zafiyet analizi ve CVSS skorlaması
│   ├── architecture.md             # HyperOS AVCodec mimari şeması
│   ├── mitigation.md               # Çözüm önerileri ve yamalar
│   └── README.md                   # Dokümantasyon rehberi
│
├── 📁 poc_python/                  # Python analiz araçları (korundu)
│   ├── exploit.py
│   └── requirements.txt
│
├── 📁 poc_rust/                    # Rust UAF simülasyonu (ana PoC)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs                 # Unsafe Rust ile UAF simülasyonu
│
├── 🌐 simulation.html              # İnteraktif web simülasyonu (5 sahne)
├── 📄 README.md                    # Bu dosya
├── 📋 plan.md                      # Proje zaman çizelgesi
├── ✅ TODO.md                      # Görev takip listesi
├── 📘 kullanmatalimatlari.md       # Adım adım kullanım rehberi
└── ⚖️  LICENSE                     # MIT Lisansı
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
[Ana Thread]  processFrameAsync() → Worker thread başlatılır
                    ↓
              release() çağrılır → Bellek SERBEST BIRAKILIR ⚠️
                    ↓
[Worker Thread]  Serbest bırakılan belleğe erişmeye devam eder → UAF 💥
```

---

## 🚀 Hızlı Başlangıç

### Gereksinimler

- **Rust** 1.70 veya üzeri → [rustup.rs](https://rustup.rs/)
- **Python** 3.8 veya üzeri
- Modern bir web tarayıcısı (Chrome, Firefox, Edge)

### 1. Rust PoC — UAF Simülasyonu

```bash
# Projeyi klonlayın
git clone https://github.com/kullanici/HyperOS-Directory-Traversal-Analysis.git
cd HyperOS-Directory-Traversal-Analysis/poc_rust

# Release modunda derleyin
cargo build --release
```

#### 🔴 Zafiyetli Senaryo (Race Condition & UAF Gösterimi)

```bash
# Windows
.\target\release\cve_2025_21082_uaf_poc.exe --mode vulnerable --verbose

# Linux / macOS
./target/release/cve_2025_21082_uaf_poc --mode vulnerable --verbose
```

**Beklenen Çıktı:**
```
🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC
Mode: vulnerable

⚠️  Running vulnerable scenario...
🧵 Starting worker thread...
🗑️  Main thread releasing codec context (UAF trigger)...
🔄 Worker thread accessing codec context...
🚨 UAF detected! Magic number corrupted: 0xFEEDFACE
💥 UAF vulnerability triggered on frame 0!

🚨 Vulnerable scenario completed - UAF demonstrated!
⚠️  In a real exploit, this could lead to RCE
```

#### 🟢 Yamalanmış Senaryo (Güvenli Senkronizasyon)

```bash
# Windows
.\target\release\cve_2025_21082_uaf_poc.exe --mode patched --verbose

# Linux / macOS
./target/release/cve_2025_21082_uaf_poc --mode patched --verbose
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

```bash
# Proje kök dizininde basit bir HTTP sunucusu başlatın
python -m http.server 8000

# Tarayıcınızda açın
# http://localhost:8000/simulation.html
```

Simülasyon **5 sahne** içermektedir:
1. 🧪 **Lab Ortamı** — AVCodec pipeline tanıtımı
2. 🏗️ **Context Oluşturma** — Heap bellek düzeni
3. ⚡ **Race Condition** — Zaman çizelgesi ve zafiyet penceresi
4. 💥 **UAF Sömürüsü** — Heap reclamation ve dangling pointer
5. 🛡️ **Patch Karşılaştırması** — Zafiyetli vs. yamalı kod

### 3. Python Analiz Araçları

```bash
cd poc_python
pip install -r requirements.txt
python exploit.py -u http://hedef:5000 -f etc/shadow
```

---

## 📚 Teknik Dokümantasyon

| Doküman | İçerik |
| :--- | :--- |
| 📊 [Zafiyet Analizi](docs/analysis.md) | CVE-2025-21082 teknik analizi, CVSS skorlaması, saldırı senaryoları, HyperOS vs AOSP karşılaştırması |
| 🏗️ [Mimari Şeması](docs/architecture.md) | AVCodec async pipeline, bileşen diyagramları, race condition akış şeması |
| 🛡️ [Çözüm Önerileri](docs/mitigation.md) | Zafiyetli/yamalı C++ kod örnekleri, RAII pattern, MTE, CFI, ASan entegrasyonu |
| 📘 [Kullanım Talimatları](kullanmatalimatlari.md) | Adım adım kurulum ve çalıştırma rehberi |

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
    // Magic number bozulduysa UAF gerçekleşmiş demektir
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
| :--- | :--- | :---: |
| **Thread Senkronizasyonu** | `join()` ile thread tamamlanana kadar bekleme | ⭐⭐⭐⭐⭐ |
| **Referans Sayımı** | `shared_ptr` ile otomatik yaşam süresi yönetimi | ⭐⭐⭐⭐⭐ |
| **RAII Pattern** | Destructor ile otomatik kaynak temizliği | ⭐⭐⭐⭐ |
| **AddressSanitizer** | Derleme zamanı UAF tespiti | ⭐⭐⭐⭐ |
| **MTE (ARM64)** | Donanım seviyesi bellek etiketleme | ⭐⭐⭐⭐⭐ |
| **CFI** | Kontrol akışı bütünlüğü koruması | ⭐⭐⭐ |

---

## ⚠️ Yasal Uyarı

> **Bu proje yalnızca eğitim ve akademik araştırma amacıyla geliştirilmiştir.**
>
> Rust simülasyonu, zafiyetin mekanizmasını göstermek için tasarlanmış olup gerçek bir exploit **değildir**. Buradaki bilgilerin ve araçların yetkisiz sistemler üzerinde kullanılması yasal sorumluluk doğurabilir. Geliştirici hiçbir sorumluluk kabul etmez.

---

## 📄 Lisans

Bu proje **MIT Lisansı** ile lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakınız.

---

<div align="center">

*Siber güvenlik araştırması — Eğitim amaçlı hazırlanmıştır*

</div>