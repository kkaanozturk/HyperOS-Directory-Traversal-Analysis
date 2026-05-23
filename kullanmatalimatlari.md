# 🚀 Adım Adım Kullanma Talimatları (En Basit Haliyle)

Bu rehber, projeyi hayatında daha önce hiç kod çalıştırmamış birinin bile rahatlıkla açıp test edebilmesi için çok basit adımlarla hazırlanmıştır. Lütfen adımları sırasıyla takip edin.

---

## 🛠️ Ön Hazırlık: Ne Yüklü Olmalı?
Bu projeyi çalıştırabilmeniz için bilgisayarınızda **Python** programının yüklü olması gerekiyor. 
- Yüklü değilse [python.org/downloads](https://www.python.org/downloads/) adresinden indirip kurun. Kurarken "Add Python to PATH" (Python'ı PATH'e ekle) kutucuğunu işaretlemeyi **kesinlikle unutmayın**.

---

## 🟢 ADIM 1: Zafiyetli Sunucuyu (Siteyi) Ayağa Kaldırmak

Saldırıyı yapabilmemiz için önce saldıracağımız sitenin çalışıyor olması lazım.

1. `HyperOS-Directory-Traversal-Analysis` (Proje) klasörünün içine girin.
2. `simulated_server` isimli klasörün içine girin.
3. Bu klasörün içindeyken klasör yolunun yazdığı üstteki çubuğa (adres çubuğuna) tıklayın, `cmd` yazın ve **Enter**'a basın. Siyah bir komut penceresi açılacak.
4. Açılan siyah pencereye şu komutu yazıp **Enter**'a basın (Bu işlem gerekli kütüphaneleri indirecek):
   ```bash
   pip install -r requirements.txt
   ```
5. Kurulum bitince siteyi çalıştırmak için şu komutu yazıp **Enter**'a basın:
   ```bash
   python app.py
   ```
6. Ekranda `* Running on http://127.0.0.1:5000` yazısını gördüyseniz, **TEBRİKLER!** Siteniz şu an çalışıyor. 
*(Önemli: Bu siyah pencereyi ASLA KAPATMAYIN, siteyi test ettiğiniz süre boyunca arka planda hep açık kalsın.)*

---

## 🔴 ADIM 2: Görsel Arayüz Üzerinden Hack (En Kolay Yöntem)

Şimdi ayağa kaldırdığımız bu şaşalı siteye girip zafiyeti gözümüzle görelim.

1. İstediğiniz bir internet tarayıcısını (Chrome, Edge, Safari vb.) açın.
2. Adres çubuğuna şunu yazıp siteye girin: **`http://127.0.0.1:5000`**
3. Karşınıza muazzam tasarımlı "HyperOS Theme Manager" sitesi çıkacak.
4. Ekranın sağ tarafındaki kırmızı renkli **"Zafiyet Test Paneli (Hacker Mode)"** kısmına bakın.
5. Oradaki kutucuğa varsayılan olarak şu yazılıdır: `../../../etc/shadow`
6. **"Exploit Gönder"** butonuna basın.
7. Alttaki siyah terminal animasyonunda yeşil yazılarla sistemin şifrelerinin (`root:$6...` gibi) döküldüğünü göreceksiniz. Zafiyeti başarıyla sömürdünüz!

---

## 🐍 ADIM 3: Python Aracı ile Komut Satırından Hack (Hacker Yöntemi)

Eğer bunu siteden değil de, tam bir hacker gibi siyah terminal ekranından yapmak isterseniz:

1. Ana proje klasörüne (`HyperOS-Directory-Traversal-Analysis`) geri dönün.
2. Bu sefer `poc_python` klasörünün içine girin.
3. Yine üstteki adres çubuğuna `cmd` yazıp **Enter**'a basarak burada YENİ bir siyah pencere açın.
4. Gerekli araçları kurmak için şunu yazıp **Enter**'a basın:
   ```bash
   pip install -r requirements.txt
   ```
5. Şimdi saldırı aracımızı ateşliyoruz! Şu komutu kopyalayıp yapıştırın ve **Enter**'a basın:
   ```bash
   python exploit.py -u http://127.0.0.1:5000 -f etc/shadow
   ```
6. Ekranda renkli yazılarla hedefe bağlanıldığını ve gizli dosyaların çekildiğini göreceksiniz.
7. Başka bir gizli dosyayı (örneğin anahtarı) çekmek isterseniz şu komutu kullanabilirsiniz:
   ```bash
   python exploit.py -u http://127.0.0.1:5000 -f var/hyperos/secret_key.pem
   ```

---

## 🦀 ADIM 4: Rust Aracı İle Hack (İsteğe Bağlı - Ekstra)

Bu adım sadece bilgisayarında "Rust" programlama dili kurulu olanlar içindir. Kurulu değilse bu adımı tamamen atlayabilirsiniz, Python aracı aynı işi yapmaktadır.

1. Ana proje klasöründen `poc_rust` klasörüne girin.
2. Burada `cmd` açın.
3. Programı derlemek için şunu yazın (biraz sürebilir):
   ```bash
   cargo build --release
   ```
4. Derleme bittikten sonra aracı çalıştırmak için şunu yazın:
   ```bash
   .\target\release\poc_rust.exe -u http://127.0.0.1:5000 -f etc/shadow
   ```

---

**ÖZET:** Önce `simulated_server` içindeki `app.py`'yi çalıştırarak siteyi açıyorsunuz. Sonra isterseniz tarayıcıdan (`http://127.0.0.1:5000`), isterseniz de `poc_python` içindeki araçla bu siteye saldırıyorsunuz. Hepsi bu kadar!
