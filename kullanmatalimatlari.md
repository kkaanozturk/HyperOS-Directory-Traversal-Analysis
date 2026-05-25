# CVE-2025-21082 Kullanım Talimatları

## 🎯 Proje Hakkında

Bu proje, Xiaomi HyperOS AVCodec framework'ünde tespit edilen **Use-After-Free (UAF)** zafiyeti olan **CVE-2025-21082**'nin detaylı analizini ve güvenli simülasyonunu içermektedir.

## 📋 Sistem Gereksinimleri

### Minimum Gereksinimler
- **Rust**: 1.70 veya üzeri
- **Python**: 3.8 veya üzeri  
- **İşletim Sistemi**: Windows 10/11, Linux, macOS
- **RAM**: 4GB (8GB önerilir)
- **Disk Alanı**: 500MB

### Önerilen Geliştirme Ortamı
- Visual Studio Code + Rust Analyzer
- Git versiyon kontrolü
- Modern web tarayıcısı (Chrome, Firefox, Edge)

## 🚀 Kurulum Adımları

### 1. Depoyu Klonlama
```bash
git clone https://github.com/username/HyperOS-Directory-Traversal-Analysis.git
cd HyperOS-Directory-Traversal-Analysis
```

### 2. Rust Ortamının Hazırlanması
```bash
# Rust kurulumu (eğer yüklü değilse)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Proje bağımlılıklarını kontrol etme
cd poc_rust
cargo check
```

### 3. Python Ortamının Hazırlanması (Opsiyonel)
```bash
cd poc_python
pip install -r requirements.txt
```

## 🔬 Rust PoC Kullanımı

### Temel Kullanım

#### Zafiyetli Senaryo Çalıştırma
```bash
cd poc_rust
cargo build --release

# Windows için:
.\target\release\cve_2025_21082_uaf_poc.exe --mode vulnerable

# Linux/macOS için:
./target/release/cve_2025_21082_uaf_poc --mode vulnerable
```

#### Yamalanmış Senaryo Çalıştırma
```bash
# Windows için:
.\target\release\cve_2025_21082_uaf_poc.exe --mode patched

# Linux/macOS için:
./target/release/cve_2025_21082_uaf_poc --mode patched
```

### Gelişmiş Kullanım

#### Detaylı Bellek Adresi Logları
```bash
# Verbose mod ile çalıştırma
./target/release/cve_2025_21082_uaf_poc --mode vulnerable --verbose
```

#### Debug Modunda Derleme
```bash
# Debug bilgileri ile derleme
cargo build

# Debug binary çalıştırma
./target/debug/cve_2025_21082_uaf_poc --mode vulnerable --verbose
```

## 📊 Çıktı Analizi

### Zafiyetli Senaryo Çıktısı
```
🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC
Mode: vulnerable
Verbose: true

⚠️ Running vulnerable scenario...
CodecContext allocated at: 0x7f8b4c000b20
🧵 Starting worker thread...
🗑️ Main thread releasing codec context (UAF trigger)...
Memory reclaimed with corrupted data at: 0x7f8b4c000b20
🔄 Worker thread accessing codec context...
🚨 UAF detected! Magic number corrupted: 0xFEEDFACE
💥 UAF vulnerability triggered on frame 0!

🚨 Vulnerable scenario completed - UAF demonstrated!
⚠️ In a real exploit, this could lead to RCE
```

### Yamalanmış Senaryo Çıktısı
```
🔬 CVE-2025-21082: HyperOS AVCodec UAF PoC
Mode: patched
Verbose: true

✅ Running patched scenario...
CodecContext allocated at: 0x7f8b4c000b20
🧵 Starting worker thread...
⏳ Waiting for worker thread to complete (patch applied)...
🔄 Worker thread processing frames safely...
✅ Frame 0 processed successfully
✅ Frame 1 processed successfully
✅ Frame 2 processed successfully
✅ Frame 3 processed successfully
✅ Frame 4 processed successfully
✅ Worker thread completed safely
🗑️ Safely releasing codec context...
CodecContext safely freed

✅ Patched scenario completed - No UAF occurred!
🛡️ Proper synchronization prevents the vulnerability
```

## 🌐 İnteraktif Simülasyon

### Web Simülasyonu Çalıştırma
```bash
# Basit HTTP sunucusu başlatma
python -m http.server 8000

# Tarayıcıda açma
# http://localhost:8000/simulation.html
```

### Simülasyon Özellikleri
- **5 Sahne Animasyonu**: Lab ortamı → Context oluşturma → Race condition → UAF sömürüsü → Patch karşılaştırması
- **İnteraktif Kontroller**: Oynat/Duraklat, adım adım ilerleme
- **Bellek Görselleştirmesi**: Heap layout ve corruption gösterimi
- **Zaman Çizelgesi**: Vulnerability timeline analizi

## 📚 Dokümantasyon İncelemesi

### Temel Dokümantasyon
```bash
# Zafiyet analizi
cat docs/analysis.md

# Sistem mimarisi
cat docs/architecture.md

# Çözüm önerileri
cat docs/mitigation.md
```

### Markdown Görüntüleme
- **VS Code**: Markdown Preview Extension
- **Typora**: Profesyonel Markdown editörü
- **GitHub**: Online görüntüleme

## 🧪 Test ve Doğrulama

### Otomatik Testler
```bash
cd poc_rust

# Unit testleri çalıştırma
cargo test

# Bellek güvenliği testleri
cargo test --features memory-safety

# Performans testleri
cargo bench
```

### Manuel Doğrulama
```bash
# Farklı parametrelerle test
./target/release/cve_2025_21082_uaf_poc --mode vulnerable --verbose > vulnerable_output.log
./target/release/cve_2025_21082_uaf_poc --mode patched --verbose > patched_output.log

# Çıktıları karşılaştırma
diff vulnerable_output.log patched_output.log
```

## 🔧 Sorun Giderme

### Yaygın Sorunlar ve Çözümleri

#### Rust Derleme Hataları
```bash
# Rust toolchain güncelleme
rustup update

# Bağımlılık önbelleğini temizleme
cargo clean
cargo build
```

#### Bellek Erişim Hataları
```bash
# AddressSanitizer ile çalıştırma
export RUSTFLAGS="-Z sanitizer=address"
cargo run --target x86_64-unknown-linux-gnu
```

#### Platform Uyumluluk Sorunları
```bash
# Hedef platform belirtme
cargo build --target x86_64-pc-windows-msvc  # Windows
cargo build --target x86_64-unknown-linux-gnu  # Linux
cargo build --target x86_64-apple-darwin  # macOS
```

### Debug Modunda Çalıştırma
```bash
# GDB ile debug (Linux/macOS)
gdb ./target/debug/cve_2025_21082_uaf_poc
(gdb) run --mode vulnerable --verbose

# LLDB ile debug (macOS)
lldb ./target/debug/cve_2025_21082_uaf_poc
(lldb) run --mode vulnerable --verbose
```

## 📈 Performans Optimizasyonu

### Release Modunda Derleme
```bash
# Optimizasyonlu derleme
cargo build --release

# Boyut optimizasyonu
cargo build --release --config profile.release.opt-level='"z"'
```

### Bellek Kullanımı İzleme
```bash
# Valgrind ile bellek analizi (Linux)
valgrind --tool=memcheck ./target/release/cve_2025_21082_uaf_poc --mode vulnerable

# Heaptrack ile heap analizi
heaptrack ./target/release/cve_2025_21082_uaf_poc --mode vulnerable
```

## 🎓 Eğitim Amaçlı Kullanım

### Akademik Çalışmalar İçin
1. **Zafiyet Analizi**: `docs/analysis.md` dosyasını inceleyin
2. **Kod İncelemesi**: `poc_rust/src/main.rs` dosyasındaki unsafe Rust kullanımını analiz edin
3. **Mitigation Stratejileri**: `docs/mitigation.md` dosyasındaki çözüm önerilerini değerlendirin

### Güvenlik Eğitimi İçin
1. **Hands-on Demo**: Rust PoC'yi farklı parametrelerle çalıştırın
2. **İnteraktif Öğrenme**: Web simülasyonunu kullanarak UAF mekanizmasını görselleştirin
3. **Karşılaştırmalı Analiz**: Vulnerable ve patched senaryoları karşılaştırın

## ⚠️ Güvenlik Uyarıları

### Önemli Notlar
- Bu proje **tamamen eğitim amaçlıdır**
- Rust simülasyonu gerçek bir exploit değildir
- Unsafe Rust kodu kontrollü ortamda çalışır
- Gerçek sistemlerde zarar verici değildir

### Etik Kullanım
- Yalnızca kendi sistemlerinizde test edin
- Yetkisiz sistemlerde kullanmayın
- Akademik ve eğitim amaçlarıyla sınırlı tutun
- Sorumlu açıklama (responsible disclosure) ilkelerine uyun

## 📞 Destek ve İletişim

### Teknik Destek
- **GitHub Issues**: Proje deposunda issue açın
- **Dokümantasyon**: `docs/` klasöründeki detaylı açıklamaları inceleyin
- **Kod Yorumları**: Kaynak kodundaki açıklayıcı yorumları okuyun

### Katkıda Bulunma
1. Fork yapın
2. Feature branch oluşturun
3. Değişikliklerinizi commit edin
4. Pull request gönderin

---

**Son Güncelleme**: 25 Mayıs 2025  
**Versiyon**: 1.0.0  
**Lisans**: MIT License