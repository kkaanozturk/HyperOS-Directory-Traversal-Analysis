<div align="center">

# 🚨 CVE-2025-2844: HyperOS Theme Manager Directory Traversal

[![Python](https://img.shields.io/badge/Python-3.8%2B-blue?logo=python&logoColor=white)](https://python.org)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://rust-lang.org)
[![Flask](https://img.shields.io/badge/Flask-Server-black?logo=flask&logoColor=white)](https://flask.palletsprojects.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

**Bir Siber Güvenlik Analizi ve Kavram Kanıtı (Proof of Concept) Projesi**

</div>

---

## 📖 Proje Hakkında

Bu repository, (simüle edilmiş) HyperOS Theme Manager uygulamasında tespit edilen **Directory Traversal (Dizin Atlama)** zafiyeti olan **CVE-2025-2844**'ün detaylı incelemesini, istismar yöntemlerini (Exploitation) ve çözüm önerilerini (Mitigation) içermektedir. Üniversite final ödevi kapsamında, tamamen profesyonel ve akademik standartlara uygun olarak hazırlanmıştır.

Zafiyetin temel sebebi, uygulamanın `/api/themes/download` uç noktasında (endpoint), kullanıcıdan gelen `theme` parametresini filtrelemeden dosya yolu çözümlemesine dahil etmesidir.

## 📂 Depo Yapısı (Repository Structure)

| Klasör / Dosya | Açıklama |
| :--- | :--- |
| 🖥️ `/simulated_server/` | Zafiyeti barındıran Flask tabanlı, "Fake Root" sistemli test sunucusu. |
| 🐍 `/poc_python/` | Zafiyeti istismar eden CLI tabanlı Python aracı. |
| 🦀 `/poc_rust/` | Performans için derlenmiş Rust tabanlı istismar aracı. |
| 📚 `/docs/` | Zafiyetin güvenlik ihlallerini ve kodlama hatalarını inceleyen analiz belgeleri. |
| 📝 `TODO.md` | Projenin gelecek planları ve yapılacaklar listesi. |

---

## 🚀 Kurulum ve Test Ortamının Hazırlanması

Zafiyeti test etmek için önce simüle edilmiş sunucuyu ayağa kaldırmanız gerekmektedir.

### 1. Zafiyetli Sunucuyu Başlatma
```bash
cd simulated_server
pip install -r requirements.txt
python app.py
```
*Sunucu `http://127.0.0.1:5000` adresinde ayağa kalkacaktır.*

### 2. Python PoC Kullanımı
Yeni bir terminal açın ve exploit scriptini çalıştırın:
```bash
cd poc_python
pip install -r requirements.txt

# /etc/shadow simülasyonunu okumak için:
python exploit.py -u http://127.0.0.1:5000 -f etc/shadow

# Gizli anahtarı okumak için:
python exploit.py -u http://127.0.0.1:5000 -f var/hyperos/secret_key.pem
```

### 3. Rust PoC Kullanımı (Opsiyonel / Yüksek Performans)
Eğer sisteminizde Rust (`cargo`) yüklüyse:
```bash
cd poc_rust
cargo build --release

# Windows için:
.\target\release\poc_rust.exe -u http://127.0.0.1:5000 -f etc/shadow

# Linux/Mac için:
./target/release/poc_rust -u http://127.0.0.1:5000 -f etc/shadow
```

---

## 🔒 Güvenlik Dokümantasyonları

- **[Zafiyet Analizi ve Etki Raporu (Vulnerability Analysis)](docs/vulnerability_analysis.md)**: Zafiyetin kök nedeni, CVSS skorlaması ve olası saldırı senaryoları.
- **[Çözüm Önerileri (Mitigation)](docs/mitigation.md)**: Güvenli kodlama yöntemleri (Secure Coding) ve zafiyetin kapatılması.

---

## ⚠️ Yasal Uyarı (Disclaimer)

Bu repodaki kodlar ve dökümanlar tamamen **eğitim amaçlı** ve **üniversite final projesi** kapsamında oluşturulmuştur. Buradaki bilgi ve araçların izinsiz sistemlerde kullanılması yasadışıdır. Ortaya çıkabilecek herhangi bir zarardan geliştirici sorumlu tutulamaz. Lütfen siber güvenliği etik kurallar çerçevesinde öğrenin.
