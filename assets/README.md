# Assets Klasörü

Bu klasör, proje için gerekli medya dosyalarını içerir.

## Demo GIF'i

`demo.gif` dosyasını buraya ekleyin. Bu dosya, README.md'de gösterilecek olan CVE-2025-21082 UAF simülasyon demosunu içermelidir.

### GIF Oluşturma Talimatları

1. **Tarayıcıda simulation.html'i açın**:
   ```bash
   python -m http.server 8000
   # http://localhost:8000/simulation.html adresine gidin
   ```

2. **Ekran kaydı alın**:
   - ScreenToGif (Windows) veya LICEcap (cross-platform) kullanın
   - Simülasyonun tüm 5 sahnesini kaydedin
   - Önerilen boyut: 800×450px
   - Maksimum dosya boyutu: 10MB

3. **GIF'i kaydedin**:
   - Dosya adı: `demo.gif`
   - Bu klasöre yerleştirin: `assets/demo.gif`

4. **README.md'yi güncelleyin**:
   - GIF bölümündeki yorum satırlarını kaldırın
   - `<img src="assets/demo.gif" alt="CVE-2025-21082 UAF Simülasyon Demosu" width="800"/>` satırını aktif hale getirin

## Rust PoC Demo

Rust PoC'nin terminal çıktısının ekran kaydını da alabilirsiniz:

```bash
cd poc_rust
cargo build --release

# Zafiyetli senaryo
.\target\release\cve_2025_21082_uaf_poc.exe --mode vulnerable --verbose

# Yamalı senaryo  
.\target\release\cve_2025_21082_uaf_poc.exe --mode patched --verbose
```

Bu kayıtları da `rust-demo.gif` olarak bu klasöre ekleyebilirsiniz.