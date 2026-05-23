# CVE-2025-2844: HyperOS Theme Manager Directory Traversal Zafiyet Analizi

## 1. Zafiyetin Tanımı ve Kapsamı
**Directory Traversal (Dizin Atlama)**, saldırganların bir web sunucusu üzerinde hedeflenen dizin dışında kalan dosyalara erişmesine olanak tanıyan kritik bir web güvenliği zafiyetidir. 

**CVE-2025-2844**, HyperOS Theme Manager API'sinde tespit edilen, dışarıdan alınan `theme` parametresinin yeterli filtreleme ve sınır denetimlerinden geçirilmemesinden kaynaklanan bir güvenlik zafiyetini temsil eder.

### 1.1. Etkilenen Bileşen
- **Bileşen:** `HyperOS Theme Manager API`
- **Uç Nokta (Endpoint):** `/api/themes/download`
- **Parametre:** `theme`
- **Zafiyet Türü:** CWE-22: Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')

## 2. Teknik Analiz ve Kök Neden (Root Cause)
Normal şartlarda, kullanıcılar yalnızca belirli bir klasör altındaki (`/var/hyperos/themes/`) tema dosyalarına erişmelidir. Ancak uygulamanın zafiyetli kod bloğu şu şekildedir:

```python
theme_name = request.args.get('theme')
# Girdi hiçbir doğrulama veya sterilizasyon işlemine tabi tutulmuyor.
file_path = os.path.join(THEMES_DIR, theme_name)
```

`os.path.join` fonksiyonu, kendisine verilen girdilerde `../` (üst dizine çıkma) karakterlerini çözümleyerek dosya yolunu geri doğru takip edebilir. Uygulamanın geliştiricisi `os.path.abspath` veya regex ile girdi filtreleme uygulamadığı için, bir saldırgan `theme` parametresine `../../../etc/shadow` değerini vererek kök dizine (root) kadar inebilir.

## 3. Güvenlik İhlalleri ve Olası Senaryolar (Threat Modeling)
Bu zafiyetin sömürülmesi durumunda oluşabilecek güvenlik ihlalleri şunlardır:

### 3.1. Hassas Veri İfşası (Information Disclosure)
Saldırganlar, sistem yapılandırma dosyalarını okuyabilir. Örneğin:
- `/etc/shadow` dosyasına erişilerek sistem kullanıcılarının parola hash'lerinin çalınması ve çevrimdışı (offline) şifre kırma saldırılarına maruz kalınması.
- `/var/hyperos/secret_key.pem` gibi uygulamanın şifreleme ve yetkilendirme (JWT vb.) süreçlerinde kullandığı özel anahtarların (Private Key) ele geçirilmesi. Bu durum, saldırganın sistemde yetkili bir kullanıcı gibi davranmasına yol açar.

### 3.2. Kaynak Kod İfşası
Saldırgan, uygulamanın kendi kaynak kodunu (`/var/www/html/app.py` veya yapılandırma dosyalarını) okuyabilir. Bu da ileride yapılabilecek daha kompleks saldırılar (RCE, SQLi) için ciddi bir bilgi toplama aracıdır.

### 3.3. Sistem Uzlaşması (System Compromise)
Eğer saldırgan, sunucudaki hassas anahtarları veya SSH anahtarlarını (`~/.ssh/id_rsa`) elde ederse, bu zafiyet salt okunur bir "Dizin Atlama" zafiyetinden çıkıp, sunucunun tamamen ele geçirilmesine (Remote Code Execution / RCE) giden yolu açabilir.

## 4. CVSS v3.1 Skorlaması (Tahmini)
- **Saldırı Vektörü (AV):** Ağ (Network)
- **Saldırı Karmaşıklığı (AC):** Düşük (Low)
- **Gerekli Ayrıcalık (PR):** Yok (None)
- **Kullanıcı Etkileşimi (UI):** Yok (None)
- **Kapsam (Scope):** Değişmedi (Unchanged)
- **Gizlilik Etkisi (C):** Yüksek (High)
- **Bütünlük Etkisi (I):** Yok (None) - (Sadece dosya okuma olduğu için)
- **Erişilebilirlik Etkisi (A):** Yok (None)
- **Tahmini Temel Skor:** **7.5 (High)**

## 5. Sonuç
CVE-2025-2844, uygulamadaki basit ama ölümcül bir girdi denetimi eksikliğinin sisteme ne kadar büyük zararlar verebileceğinin kritik bir kanıtıdır. Güvenli yazılım geliştirme döngüsünde (SSDLC), girdi denetimlerinin her zaman "Zero Trust" (Sıfır Güven) prensibi ile yapılması gerekmektedir.
