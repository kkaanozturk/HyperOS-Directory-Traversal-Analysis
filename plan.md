# Proje Zaman Çizelgesi: CVE-2025-2844 (HyperOS Theme Manager Directory Traversal)

Bu belge, HyperOS Theme Manager üzerindeki Directory Traversal zafiyetini (CVE-2025-2844) inceleyen final projesi için 1 Haziran'a kadar olan günlük çalışma planını içermektedir.

## 📅 Zaman Çizelgesi (18 Mayıs - 1 Haziran)

### 1. Aşama: Temellendirme ve Araştırma (18-21 Mayıs)
- **18 Mayıs:** Proje reposunun başlatılması, zaman çizelgesinin (`plan.md`) oluşturulması ve zafiyetin kapsamının (gerçek/simüle) netleştirilmesi.
- **19 Mayıs:** Directory Traversal zafiyet mekanizmasının ve HyperOS Theme Manager yapısının detaylı literatür taraması.
- **20 Mayıs:** Zafiyetin teorik altyapısı ve etki analizi hakkında rapor taslağının oluşturulması. Profesyonel `README.md` yapısının kurgulanması.
- **21 Mayıs:** Hedef ortamın (gerçek sistem veya zafiyetli simülasyon sunucusu) belirlenip test mimarisinin kurgulanması.

### 2. Aşama: Kodlama ve PoC (Proof of Concept) Geliştirme (22-26 Mayıs)
- **22 Mayıs:** Zafiyetli test sunucusunun (varsa simülasyon) geliştirilmesi (Örn: Flask veya Express.js ile temsili Theme Manager).
- **23 Mayıs:** PoC (Proof of Concept) exploit kodunun mimarisinin tasarlanması ve Python ile iskeletin kurulması.
- **24 Mayıs:** Directory Traversal payload'larının (`../../../` varyasyonları, URL encoding bypass vb.) koda entegrasyonu.
- **25 Mayıs:** Yazılan PoC'nin test ortamında denenmesi. Scriptin komut satırı argümanları (CLI) alacak şekilde profesyonelleştirilmesi (hedef URL, payload vb.).
- **26 Mayıs:** Hata ayıklama (Debugging), hata yönetimi (try-except) ve kodun belgelendirilmesi (docstrings).

### 3. Aşama: Dokümantasyon ve Profesyonelleştirme (27-29 Mayıs)
- **27 Mayıs:** `README.md` dosyasının detaylı kurulum, kullanım ve zafiyet açıklamasıyla birlikte son haline getirilmesi.
- **28 Mayıs:** Zafiyetin nasıl kapatılabileceğine dair güvenli kodlama önerilerinin (Mitigation/Patch) eklenmesi.
- **29 Mayıs:** Tüm repodaki kodların ve dokümanların gözden geçirilmesi, klasör hiyerarşisinin düzenlenmesi (`/src`, `/docs`, `/poc` vb.).

### 4. Aşama: Demo Hazırlığı ve Son Kontroller (30 Mayıs - 1 Haziran)
- **30 Mayıs:** Demo video senaryosunun hazırlanması. (Arka plan bilgisi -> Zafiyetli sistemin gösterimi -> Exploit'in çalıştırılması -> Ele geçirilen verinin gösterimi).
- **31 Mayıs:** Demo videosunun çekilmesi, README'ye eklenmesi ve projenin GitHub reposuna commitlenmesi.
- **1 Haziran:** Son kontroller ve projenin teslimi.
