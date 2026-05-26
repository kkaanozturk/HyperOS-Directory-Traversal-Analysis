# CVE-2025-21082 & Ağ/Bellek Dedektifliği: 50 Adımda Analiz

Bu doküman, HyperOS AVCodec Use-After-Free zafiyetini (CVE-2025-21082) ve siber güvenlikteki analiz yaklaşımlarını (Polis Dedektifi benzetimi) 50 küçük ve sıralı adımda açıklamaktadır. Her bir adım, bir önceki adımın üzerine inşa edilmiştir.

---

## 📂 BÖLÜM 1: Temel Kavramlar (Bellek ve Thread Yönetimi)

### 1. Bellek Bölgeleri (Stack vs. Heap)
Bilgisayar programları verilerini depolamak için iki ana bellek alanı kullanır: **Stack (Yığın)** ve **Heap (Dinamik Bellek)**. Stack, boyutu önceden bilinen ve hızlı yönetilen geçici veriler içindir. Heap ise çalışma zamanında boyutu değişebilen dinamik veriler için ayrılmış büyük bir bellek alanıdır.

### 2. Dinamik Bellek Tahsisi (Allocation)
C/C++ gibi dillerde, programcılar Heap alanından yer ayırmak için `malloc` veya `new` komutlarını kullanırlar. Bu işlem sonucunda işletim sistemi programcıya belleğin başlangıç adresini gösteren bir **Pointer (İşaretçi)** teslim eder.

### 3. Belleği Serbest Bırakma (Free)
İşimiz bittiğinde, Heap üzerinde ayırdığımız belleği sisteme geri vermemiz gerekir (`free` veya `delete` ile). Eğer bunu yapmazsak **Memory Leak (Bellek Sızıntısı)** oluşur ve program zamanla bilgisayarın tüm RAM'ini tüketebilir.

### 4. Dangling Pointer (Boşta Kalan İşaretçi)
Bir bellek bölgesi serbest bırakıldıktan sonra, o bölgenin adresini tutmaya devam eden işaretçilere **Dangling Pointer** denir. Bu işaretçi artık geçerli olmayan bir adresi göstermektedir ancak hâlâ o adrese yazma/okuma yetkisine sahip gibi davranır.

### 5. Use-After-Free (UAF) Nedir?
Eğer bir program, serbest bırakılmış (`free` edilmiş) bir bellek alanına, o alanı gösteren bir dangling pointer üzerinden tekrar erişmeye (okuma veya yazmaya) çalışırsa, buna **Use-After-Free (CWE-416)** zafiyeti denir.

### 6. Eşzamanlılık ve Thread Kavramı
Modern işlemciler aynı anda birden fazla işi yapabilmek için **Thread (İş parçacığı)** yapısını kullanır. Ana thread kullanıcı arayüzünü veya genel akışı kontrol ederken, arka plandaki worker thread'ler ağır işleri (video işleme, dosya indirme vb.) asenkron olarak yürütür.

### 7. Paylaşılan Bellek Alanları
Birden fazla thread, aynı anda Heap üzerindeki ortak bir nesneye erişebilir. Bu paylaşılan bellek alanı düzgün yönetilmezse thread'ler birbirlerinin verilerini bozabilir.

### 8. Race Condition (Yarış Durumu)
İki veya daha fazla thread'in aynı bellek adresine aynı anda erişmeye çalışması ve bu erişimlerin sırasının işlem sonucunu etkilemesi durumuna **Race Condition** denir. UAF zafiyetleri genellikle race condition durumları ile birleştiğinde tetiklenir.

---

## ⚙️ BÖLÜM 2: Nasıl Çalışır? (AVCodec Mimari Hatası)

### 9. HyperOS AVCodec ve Yapay Zeka Katmanı
HyperOS AVCodec, video ve ses dosyalarını çözmek için tasarlanmış bir medya framework'üdür. Performansı artırmak için yapay zeka destekli bir iyileştirme katmanı (AI Enhancement) barındırır ve bu katman asenkron (arka planda) çalışır.

### 10. Codec Context Nesnesi Oluşturma
Sistem bir video oynatmak istediğinde, Heap üzerinde bir `AVCodecContext` nesnesi oluşturur. Bu nesne video çözücünün durumunu, çözünürlüğünü ve bellek işaretçilerini tutar.

### 11. Asenkron İşleme Başlatılması
Video karelerini çözmek için ana thread, arka planda çalışacak asenkron bir worker thread tetikler (`processFrameAsync`). Bu thread, `AVCodecContext` nesnesinin adresini kullanarak bellek alanına erişir.

### 12. Asenkron İşlemin Yarım Kalması
Worker thread video karesini işlerken donanım veya ağ yavaşlığından dolayı ufak bir gecikme yaşayabilir (örneğin 50ms-100ms arası duraklama).

### 13. Erken Serbest Bırakma (Release) Çağrısı
Kullanıcı videoyu kapattığında veya durdurduğunda, ana thread hızlıca `release()` fonksiyonunu çağırır. Bu fonksiyon, video codec nesnesinin kullandığı Heap bellek alanını anında serbest bırakır (`delete buffer_`).

### 14. Senkronizasyon Eksikliği (Zafiyet Noktası)
Zafiyetli HyperOS kodunda, ana thread bellek alanını serbest bırakırken arka plandaki worker thread'in işini bitirip bitirmediğini kontrol etmez (`join()` veya kilit mekanizması yoktur).

### 15. Dangling Pointer'ın Tetiklenmesi
Bellek serbest bırakılmıştır ancak worker thread hâlâ eski `AVCodecContext` nesnesinin adresini hafızasında tutmaktadır ve oraya erişmeye çalışmaktadır.

### 16. Use-After-Free Oluşması
Worker thread, silinmiş olan adresten veri okumaya veya oraya yazmaya çalıştığı anda **Use-After-Free** gerçekleşir. Sistem artık geçersiz veya başkası tarafından doldurulmuş bir bellek bölgesini işlemektedir.

---

## 🎯 BÖLÜM 3: Zafiyet mi? (Güvenlik Riski ve Etki Analizi)

### 17. Use-After-Free Neden Bir Zafiyettir?
UAF sadece bir program hatası değil, kritik bir güvenlik açığıdır. Çünkü işletim sistemi serbest bırakılan bellek alanını başka programlara veya aynı programın başka hassas verilerine tahsis edebilir.

### 18. Bellek Düzeninin Manipüle Edilmesi (Heap Spraying)
Saldırganlar, bellek serbest bırakıldıktan hemen sonra Heap alanını kendi hazırladıkları sahte yapılarla doldurabilirler. Worker thread çalışmaya devam ettiğinde, orijinal veri yerine saldırganın yerleştirdiği zararlı veriyi okur.

### 19. vtable (Sanal Fonksiyon Tablosu) Ele Geçirme
C++ nesneleri, fonksiyonlarının adreslerini tutan bir `vtable` işaretçisine sahiptir. Saldırgan bellek alanını ele geçirdiğinde, bu `vtable` adresini kendi hazırladığı zararlı kodların adresine yönlendirir.

### 20. Control Flow Hijacking (Kontrol Akışının Ele Geçirilmesi)
İşlemci, worker thread'in bir sonraki fonksiyonunu çalıştırmak istediğinde, saldırganın değiştirdiği adrese atlar. Programın normal çalışma akışı kırılır ve saldırganın istediği yöne sapar.

### 21. Uzaktan Kod Çalıştırma (RCE) Potansiyeli
Eğer saldırgan bellek adreslerini ve içeriğini hassas bir şekilde ayarlayabilirse, hedef cihaz üzerinde tamamen kendi kontrolünde olan komutları çalıştırabilir (Remote Code Execution - RCE).

### 22. Hizmet Dışı Bırakma (DoS / Crash)
En hafif saldırı senaryosunda bile, geçersiz belleğe erişim işletim sistemi tarafından engellenir (Segmentation Fault) ve medya oynatıcısı veya tüm işletim sistemi çökerek hizmet dışı kalır.

### 23. CVE-2025-21082 Tanımlaması
Bu zafiyet MITRE tarafından **CVE-2025-21082** koduyla tescil edilmiştir. HyperOS AVCodec framework'ünün asenkron bellek yönetimindeki bu senkronizasyon hatasını resmi olarak belgeler.

### 24. CVSS Skoru: 8.1 (Yüksek)
Zafiyetin CVSS v3.1 puanı 8.1'dir. Ağ üzerinden tetiklenebilmesi (zararlı bir video dosyasını oynatarak) ve cihaz üzerinde yetkisiz kod çalıştırma riski barındırması nedeniyle "Yüksek Dereceli" olarak sınıflandırılmıştır.

---

## 🕵️ BÖLÜM 4: Ağ ve Bellek Dedektifliği (Polis Analojisi)

### 25. Polis Dedektifi Mantığı
Güvenlik analizi yapmak, bir suç mahali incelemesine benzer. Bir dedektif nasıl ipuçlarını takip ediyorsa, güvenlik analisti de sistemdeki anomalileri izler.

```
+-------------------------------------------------------------+
|               POLİS DEDEKTİFİ ANALOJİ TABLOSU               |
+----------------------+--------------------------------------+
| Polis Kavramı        | Güvenlik / Analiz Karşılığı          |
+----------------------+--------------------------------------+
| Olay Yeri            | Ağ Arayüzü / Bellek Adresi           |
| Delil                | Ağ Paketleri / Ham Byte Değerleri    |
| Tercüman             | Wireshark / ASan Log Çıktıları       |
| Kanıt                | Anormal Desenler / UAF Hata Logu     |
| Tutuklama            | Firewall Engeli / Güvenlik Yaması    |
+----------------------+--------------------------------------+
```

### 26. Olay Yeri (Gözlem Noktası / Interface)
Bir suçluyu yakalamak için onun geçeceği caddede pusuda beklemelisiniz. Ağ analizinde bu, **Network Interface** (localhost veya ağ kartı), bellek analizinde ise `0x` ile başlayan ham bellek adresidir.

### 27. Delil Toplama (Raw Data)
Dedektif suç mahalindeki parmak izlerini toplar. Güvenlik analizinde delil, ağdan geçen ham paket baytları veya bellekteki ham hex değerleridir (örneğin: `0xDEADBEEF`, `0xFEEDFACE`).

### 28. Tercüman Kullanımı (Wireshark & ASan)
Yabancı dildeki bir delili anlamak için tercümana ihtiyaç vardır. Ağ trafiği için **Wireshark** veya **tcpdump** ham paketleri anlamlı protokollere çevirir. Bellek için **AddressSanitizer (ASan)** serbest bırakılmış belleğe yapılan usulsüz erişimleri insan okuyabilir hata raporlarına dönüştürür.

### 29. Kanıt Analizi (Deşifre)
Tercüme edilen veriler incelenerek suç kanıtı aranır. Port numaralarına bakılır (22=SSH, 80=HTTP, 443=HTTPS). Bellekte ise sihirli numaralara (magic numbers) bakılarak bozulma olup olmadığı tespit edilir.

### 30. Anormallik Tespiti (Anomalies)
Şüpheli davranışlar tespit edilir. Örneğin, bir IP adresinin saniyeler içinde binlerce farklı portu sorgulaması (Port Taraması / Recon) veya bir thread'in silinmiş bir nesneye ait belleğe sürekli istek göndermesi anormal bir durumdur.

### 31. İmzalar ve Desenler (Signatures)
Bilinen saldırı kalıpları kontrol edilir. Paketlerin içinde `/etc/passwd` veya `SELECT * FROM` gibi zararlı ifadeler aramak veya bellek hata çıktısında `heap-use-after-free` ifadesini görmek kesin bir saldırı kanıtıdır.

### 32. Tutuklama ve Engelleme (Mitigation)
Kanıtlar doğrultusunda suçlu engellenir. Ağ güvenliğinde şüpheli IP adresi güvenlik duvarında (firewall) engellenir. Yazılımda ise zafiyetli kod yamalanarak (kilit mekanizmaları eklenerek) bellek suistimali kalıcı olarak önlenir.

---

## 🛡️ BÖLÜM 5: Nasıl Korunuruz? (Yama ve Güvenlik Mekanizmaları)

### 33. Thread Senkronizasyonu (Join Kullanımı)
En temel çözüm, ana thread'in bellek alanını serbest bırakmadan önce arka plandaki worker thread'in bitmesini beklemesidir. C++'da `worker_thread_.join()` komutu, worker thread tamamen kapanana kadar ana thread'i bloke ederek güvenliği sağlar.

### 34. Akıllı İşaretçiler (Smart Pointers)
Manuel bellek yönetimi yerine C++'da `std::shared_ptr` ve `std::weak_ptr` kullanılmalıdır. `shared_ptr`, bir nesneyi kullanan kaç aktif referans olduğunu sayar. Nesne ancak tüm thread'ler onunla işini bitirdiğinde (referans sayısı 0 olduğunda) otomatik olarak silinir.

### 35. Mutex (Karşılıklı Dışlama) Kilitleri
`std::mutex` yapısı, paylaşılan bellek alanına aynı anda sadece tek bir thread'in erişmesini sağlar. Bir thread veri yazarken veya silerken bellek bölgesini kilitler, diğer thread'lerin erişimini sıraya koyarak race condition'ı engeller.

### 36. Condition Variable (Koşul Değişkenleri)
Thread'lerin sürekli döngüde işlemciyi tüketmek yerine, bir olay gerçekleşene kadar uyku modunda beklemesini sağlar. Nesne silineceği zaman worker thread'e güvenli kapatma sinyali iletilir.

### 37. Rust Dilinin Bellek Güvenliği (Rust Safety)
C++'ın aksine, Rust dili derleme aşamasında **Ownership (Sahiplik)** ve **Borrow Checker** mekanizmalarıyla bellek güvenliğini denetler. Rust'ta aynı veriye hem yazma hakkı olan hem de asenkron erişen dangling pointer oluşturmak derleyici tarafından engellenir.

### 38. AddressSanitizer (ASan) Entegrasyonu
Derleme aşamasında eklenen `-fsanitize=address` parametresi, bellek sınırlarını ve serbest bırakılan bölgeleri işaretler. Program çalışırken herhangi bir UAF hatası oluşursa anında çöker ve hatanın oluştuğu kod satırını raporlar.

### 39. Memory Tagging Extension (MTE)
ARM64 mimarisine sahip modern işlemcilerde bulunan donanımsal bir korumadır. Bellek adreslerine ve işaretçilere 4 bitlik "etiketler" (tags) atar. İşaretçideki etiket ile bellekteki etiket eşleşmezse (örneğin UAF suistimal edilmeye çalışılırsa) donanım anında hata verir.

### 40. Control Flow Integrity (CFI)
Derleyicilerin uyguladığı bu koruma, programın çalışma yollarını (grafiğini) önceden belirler. Saldırgan sanal fonksiyon tablolarını (vtable) değiştirip programı beklenmeyen bir adrese yönlendirmeye çalışırsa, CFI bunu tespit edip işlemi durdurur.

### 41. Fortify Source ve Stack Canaries
Derleyici seviyesindeki bu güvenlik önlemleri, bellek taşmalarını ve stack üzerindeki kritik değişkenlerin (dönüş adresleri) değiştirilmesini çalışma zamanında tespit ederek saldırı zincirini kırar.

---

## 💻 BÖLÜM 6: Proje Notları ve Kod Yapısı (Pratik Uygulama)

### 42. Rust PoC Yapısı (`poc_rust/`)
Projemizdeki Rust kodu, zafiyetin mantığını kavramak amacıyla yazılmış güvenli bir simülasyondur. Rust'ın `unsafe` bloklarını kullanarak UAF zafiyetinin C++ tarafında nasıl oluştuğunu ve Rust ile nasıl engellendiğini gösterir.

### 43. CodecContext Yapısı (Rust Tarafı)
Simülasyonda `CodecContext` nesnesi oluşturulur. İçinde bellek adresini tutan `buffer_ptr`, çerçeve sayacı, durum bilgisi ve bütünlüğü doğrulamak için `magic` (0xDEADBEEF) değeri bulunur.

### 44. Zafiyetli Mod Çalışma Mantığı (`--mode vulnerable`)
Bu modda program, codec nesnesini Heap üzerinde oluşturur ve asenkron worker thread'i başlatır. Ardından, worker thread'in işini bitirmesini beklemeden (`join()` yapmadan) nesneyi serbest bırakır.

### 45. Sihirli Numara Bozulması (Magic Corruption)
Bellek serbest bırakıldığında, program nesnenin içindeki `magic` değerini `0xFEEDFACE` olarak değiştirir (belleğin bozulduğunu simüle etmek için). Worker thread bellekten okuma yaptığında bu değişikliği algılar ve zafiyetin tetiklendiğini bildirir.

### 46. Yamalı Mod Çalışma Mantığı (`--mode patched`)
Yamalı modda program, bellek temizleme işleminden önce worker thread'in tamamlanmasını garanti altına alır (`join()` çağırır). Böylece bellek silindiğinde arka planda ona erişmeye çalışan aktif hiçbir thread kalmaz.

### 47. Rust PoC Derleme Adımı
Geliştirme dizinine gidip projeyi derlemek için aşağıdaki komut çalıştırılır:
```bash
cd poc_rust
cargo build --release
```

### 48. Zafiyetli Modu Çalıştırma ve Çıktı Analizi
Zafiyeti gözlemlemek için çalıştırılan komut ve beklenen çıktı:
```bash
./target/release/cve_2025_21082_uaf_poc --mode vulnerable --verbose
```
*Çıktı:* `🚨 UAF detected! Magic number corrupted: 0xFEEDFACE` mesajı ile belleğin bozulduğu ve Use-After-Free'nin gerçekleştiği gösterilir.

### 49. Yamalı Modu Çalıştırma ve Çıktı Analizi
Güvenli çalışmayı gözlemlemek için çalıştırılan komut:
```bash
./target/release/cve_2025_21082_uaf_poc --mode patched --verbose
```
*Çıktı:* Tüm frame'ler başarıyla işlenir (`Frame processed successfully`), ardından bellek güvenli bir şekilde serbest bırakılır.

### 50. UAF Karşılaştırma Matrisi (Hızlı Özet)
Aşağıdaki özet tablosu, zafiyetli, yamalı ve bellek güvenli diller arasındaki temel farkları göstermektedir:

```
+-----------------------------------------------------------------------------------+
|                           UAF KARŞILAŞTIRMA MATRİSİ                               |
+------------------------+------------------------+---------------------------------+
| Durum / Dil            | Bellek Serbest Bırakma | Thread Durumu                   |
+------------------------+------------------------+---------------------------------+
| Zafiyetli C++          | Anında (delete)        | Kontrol edilmez, UAF oluşur.   |
| Yamalı C++             | Senkronize (delete)    | join() ile beklenir, güvenli.   |
| Standart Rust (Safe)   | Otomatik (Drop)        | Derleyici seviyesinde engellenir|
+------------------------+------------------------+---------------------------------+
```
