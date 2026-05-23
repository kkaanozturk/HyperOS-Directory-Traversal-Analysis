# 📝 Proje Yapılacaklar (TODO) Listesi

Bu dosya, projeyi aktif olarak yönetmek, takip etmek ve ileride geliştirilebilecek özellikleri listelemek amacıyla oluşturulmuştur.

## 🔄 Devam Eden Görevler
- [x] Flask ile simüle edilmiş zafiyetli sunucunun (`simulated_server`) kodlanması.
- [x] Python tabanlı ilk exploit PoC'sinin yazılması.
- [x] Rust tabanlı, yüksek performanslı PoC aracının geliştirilmesi.
- [x] Etki Analizi (Vulnerability Analysis) dokümanının yazılması.
- [x] Çözüm Önerileri (Mitigation) rehberinin oluşturulması.

## 🚀 Gelecek Planları (Future Enhancements)
- [ ] **Docker Desteği:** `simulated_server` için tek tıkla kurulum sağlayan bir `Dockerfile` ve `docker-compose.yml` oluşturulması. Böylece test ortamı tamamen izole bir şekilde ayağa kalkabilir.
- [ ] **Gelişmiş Payload Listesi (Fuzzing):** Python PoC aracına dışarıdan bir `payloads.txt` (örneğin Wfuzz veya SecLists tabanlı) dosyası verebilme özelliğinin eklenmesi.
- [ ] **Sembolik Link Bypass Testleri:** Klasik `../` haricinde, sembolik linkleri (Symlink) takip ederek yapılan atlatma yöntemlerinin analizi ve PoC'ye entegrasyonu.
- [ ] **Otomatik Raporlama:** PoC araçlarının başarıyla sızdığı hedef bilgileri ve çektiği dosyaları otomatik olarak PDF veya JSON formatında raporlaması.

## 🐛 Bilinen Sorunlar (Known Issues)
- Şu anda Windows ortamında path separator (`\` vs `/`) sebebiyle simülasyonda ufak uyumsuzluklar olabilir, PoC'ler Unix tabanlı (`/`) yollar düşünülerek yazılmıştır.
- Çok derin dizinlerde (ör. 20 kademe `../`) Flask bazen 400 Bad Request dönebilmektedir, bu sunucu ayarlarından (Werkzeug limitleri) kaynaklanabilir.
