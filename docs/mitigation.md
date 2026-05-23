# CVE-2025-2844: Zafiyet Çözüm ve İyileştirme Rehberi (Mitigation)

Directory Traversal zafiyetini kalıcı olarak gidermek için (Patching), dışarıdan gelen (kullanıcı kontrollü) hiçbir veriye güvenilmemeli ve dosya yolları oluşturulurken "White-listing" (Beyaz Liste) ile güçlü sınır denetimleri (Boundary Checks) uygulanmalıdır.

## 1. Güvenli Kodlama Stratejileri (Python/Flask)

Zafiyeti düzeltmek için uygulamamız gereken en güvenli yöntem, istenen dosyanın kesinlikle hedeflenen `THEMES_DIR` dizini içinde kaldığından emin olmaktır.

### Yanlış (Zafiyetli) Kullanım:
```python
theme_name = request.args.get('theme')
file_path = os.path.join(THEMES_DIR, theme_name)
return send_file(file_path)
```

### Doğru (Güvenli) Kullanım:
```python
import os
from werkzeug.utils import secure_filename

@app.route('/api/themes/download', methods=['GET'])
def download_theme():
    theme_name = request.args.get('theme')
    
    if not theme_name:
        return jsonify({"error": "Missing theme parameter"}), 400

    # 1. Aşama: secure_filename kullanımı. 
    # Bu fonksiyon dosya adındaki "/", "\" ve "../" gibi tehlikeli karakterleri temizler.
    safe_theme_name = secure_filename(theme_name)
    
    # 2. Aşama: Yolu oluştur.
    file_path = os.path.join(THEMES_DIR, safe_theme_name)
    
    # 3. Aşama: Ek Güvenlik Kontrolü (Boundary Check)
    # Elde edilen mutlak (absolute) yolun gerçekten THEMES_DIR ile başlayıp başlamadığını kontrol edin.
    # Bu, sembolik link (symlink) saldırılarına karşı da koruma sağlar.
    real_base = os.path.realpath(THEMES_DIR)
    real_path = os.path.realpath(file_path)
    
    if not real_path.startswith(real_base):
        return jsonify({"error": "Access denied! Invalid path detected."}), 403

    if not os.path.exists(real_path):
        return jsonify({"error": "File not found."}), 404
        
    return send_file(real_path, as_attachment=True)
```

## 2. Sunucu Seviyesinde Alınabilecek Önlemler

Kod tarafındaki onarımlara ek olarak (Defense in Depth / Derinlemesine Savunma prensibi gereği), sistem yöneticileri şu önlemleri almalıdır:

### 2.1. Least Privilege (En Az Ayrıcalık Prensibi)
Uygulamanın çalıştığı servis hesabı (örn. `www-data` veya `hyperos-api`), kök dizindeki `/etc/shadow` veya `/root/` gibi klasörlere okuma yetkisine sahip olmamalıdır. Sadece `/var/hyperos/themes/` klasörüne kısıtlı erişimi olmalıdır.

### 2.2. Chroot Jail ve Docker İzolasyonu
Uygulama bir chroot ortamına veya izole bir Docker konteynerine hapsedilmelidir. Böylece saldırgan bir şekilde `../../../` ile kök dizine ulaşsa bile, asıl sunucunun kök dizinine değil, sadece konteynerin kısıtlı kök dizinine ulaşabilir.

### 2.3. WAF (Web Application Firewall) Kurulumu
Uygulamanın önüne konumlandırılacak bir WAF, HTTP isteklerindeki `../`, `%2e%2e%2f` gibi bilinen Directory Traversal payload'larını tespit edip engellemek üzere yapılandırılmalıdır.

## 3. Sonuç
Zafiyetin kök nedeni zayıf girdi doğrulamasıdır. Hem uygulama (Code Level) tarafında `secure_filename()` ve `os.path.realpath` ile sıkı doğrulamalar yapılmalı, hem de sistem mimarisinde en az ayrıcalık ve konteynerizasyon kullanılarak tam güvenlik sağlanmalıdır.
