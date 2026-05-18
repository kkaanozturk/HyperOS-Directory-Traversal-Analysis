# Proje Zaman Çizelgesi: CVE-2025-2844 (HyperOS Theme Manager Directory Traversal)

Bu belge, HyperOS Theme Manager üzerindeki Directory Traversal zafiyetini (CVE-2025-2844) inceleyen final projesi için 1 Haziran'a kadar olan günlük çalışma planını içermektedir.

## 📅 Zaman Çizelgesi (18 Mayıs - 1 Haziran)

### 1. Aşama: Temellendirme ve Araştırma (18-20 Mayıs)
- **18 Mayıs:** Proje reposunun başlatılması, zaman çizelgesinin (`plan.md`) güncellenmesi ve `.git` yapısının kurulması. Temel klasör hiyerarşisinin (`/simulated_server`, `/poc_python`, `/poc_rust`, `/docs`) oluşturulması.
- **19 Mayıs:** Directory Traversal zafiyet mekanizmasının ve HyperOS Theme Manager yapısının detaylı literatür taraması. Profesyonel `README.md` iskeletinin oluşturulması.
- **20 Mayıs:** Zafiyetli test sunucusunun (Flask ile simüle edilmiş "HyperOS Theme Manager") geliştirilmesi. Hedef dosya olarak hassas yapılandırma dosyasının (`/var/hyperos/secret_key.pem` ve `/etc/shadow` simülasyonu) sisteme kurgulanması.

### 2. Aşama: Python ile PoC Geliştirme ve Test (21-24 Mayıs)
- **21 Mayıs:** Python ile PoC (Proof of Concept) exploit kodunun temel iskeletinin yazılması.
- **22 Mayıs:** Directory Traversal payload'larının (`../../../` varyasyonları, URL encoding bypass vb.) Python koduna entegrasyonu ve simüle edilmiş sunucu üzerinde test edilmesi.
- **23 Mayıs:** Python scriptinin komut satırı argümanları (CLI) alacak şekilde geliştirilmesi (hedef URL, okunacak dosya vb.).
- **24 Mayıs:** Hata ayıklama (Debugging), hata yönetimi (try-except) ve kodun belgelendirilmesi (docstrings). Zafiyetin tam olarak çalıştığının doğrulanması.

### 3. Aşama: Rust ile Optimizasyon ve Profesyonelleştirme (25-28 Mayıs)
- **25 Mayıs:** Çalışan Python mantığının Rust diline çevrilmesi (hız, performans ve düşük boyut için).
- **26 Mayıs:** Rust projesinin (`cargo new`) yapılandırılması, HTTP istekleri için gerekli kütüphanelerin (`reqwest` vb.) entegrasyonu.
- **27 Mayıs:** Rust tabanlı PoC'nin simüle edilmiş sunucuda test edilmesi ve derlenmesi (release build).
- **28 Mayıs:** Rust kodunun optimize edilmesi, CLI argümanlarının (`clap` crate) eklenmesi ve dokümantasyon.

### 4. Aşama: Dokümantasyon ve Demo Hazırlığı (29 Mayıs - 1 Haziran)
- **29 Mayıs:** `README.md` dosyasının detaylı kurulum (Python ve Rust için ayrı ayrı), kullanım ve zafiyet açıklamasıyla birlikte son haline getirilmesi. Zafiyetin nasıl kapatılabileceğine dair güvenli kodlama önerilerinin (Mitigation/Patch) eklenmesi.
- **30 Mayıs:** Demo video senaryosunun hazırlanması. (Arka plan bilgisi -> Zafiyetli sunucunun başlatılması -> Rust/Python PoC çalıştırılması -> Gizli anahtarın ele geçirilmesi).
- **31 Mayıs:** Demo videosunun çekilmesi, README'ye eklenmesi ve projenin GitHub reposuna temiz bir şekilde commitlenmesi.
- **1 Haziran:** Son kontroller ve projenin teslimi.
