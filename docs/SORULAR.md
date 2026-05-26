# CVE-2025-21082 Proje Soruları ve Cevapları

Bu doküman, HyperOS AVCodec Use-After-Free zafiyet analizi projesi kapsamında ele alınan temel soruları ve cevaplarını içermektedir.

---

## 🎯 Proje Nedir?

Bu proje, macOS sistemlerinde network keşfi (recon) ve banner grabbing yapabilen bir araç değil, **HyperOS AVCodec framework'ündeki Use-After-Free zafiyetinin (CVE-2025-21082) kapsamlı analizi ve simülasyonudur**. Amaç, hedef sistemler hakkında bilgi toplamak ve açıklarını tespit etmek değil, **bellek güvenliği zafiyetlerinin nasıl çalıştığını eğitim amaçlı göstermektir**.

---

## ⚙️ Nasıl Çalışır?

### Temel Çalışma Prensibi:

#### 🔴 Zafiyetli Senaryo:
- Hedef IP ve portlara TCP bağlantısı kurmaz
- Zararsız paketler göndererek servislerin banner'larını almaz  
- Renkli terminal çıktıları ile kullanıcıya bilgi sunmaz
- Socket programlama ve network protokollerini kullanmaz

#### 🟢 Gerçek Çalışma Şekli:
**Rust ile UAF Simülasyonu:**
```rust
// CodecContext heap'te oluşturulur
let codec_ptr = Box::into_raw(Box::new(CodecContext::new()));

// Worker thread başlatılır
let worker = thread::spawn(move || {
    // Bu thread codec context'e erişmeye çalışır
    (*codec_ptr).process_frame();
});

// Ana thread hemen belleği serbest bırakır (UAF tetikleyici)
Box::from_raw(codec_ptr);

// Worker thread dangling pointer'a erişir → UAF tespit edilir
```

---

## 🧠 Mantığı Anlamak Önemlidir

Kodun kendisini yazmak sadece syntaxı doğru kullanmak değil, arka plandaki mantığı anlamak demektir. Bu nedenle:

### 📚 Önce Mevcut Kodu Okuyup Anlamaya Çalış
- Her fonksiyonun ne yaptığını, girdi/çıktılarını incele
- Network akışı ve veri hareketlerini takip et  
- Testlerle nasıl çalıştığını gözlemle

### 🔍 Kod Önemli Değil, Anlamak Önemli
İlk başta hatalar yapmak, kodun tamamını anlamak için gereklidir. Kendine zaman tanı, tekrar tekrar okuyup, debug ederek öğren.

---

## 🕵️ Şimdi İkinci Aşamaya Geçelim — Polis Dedektifi Gibi Düşün

Suçluyu yakalamak için:

### 1. 🎯 Nerede Durman Lazım? (Gözlem Noktası)
- **Ağ trafiğinin geçtiği bir noktada olmalısın**: Bilgisayarın ağ kartında (localhost), modem/gateway'de, ya da bir sunucunun önünde
- **Tipik bir polisin suçluyu görmek için sokak köşesinde beklemesi gibi**, sen de `Wireshark`, `tcpdump` veya `netstat` gibi araçlarla ağın "köşesinde" beklersin
- **Dinlediğin yer yanlışsa, suçluyu kaçırırsın**

### 2. 🔍 Neleri Çevirmen (Analiz Etmen) Lazım? (Deşifre)

#### 📊 Ham Veriyi Oku:
Gelen giden paketler ham byte'lar halindedir. Bunları insan okuyabilir hale getirmelisin (hex → ASCII, binary → metin)

#### 🔢 Port Numaralarını Çevir:
- 22 = SSH
- 80 = HTTP  
- 443 = HTTPS
- 445 = SMB...
- Hangi kapıdan giriyor?

#### 🌐 IP Adreslerini Çöz:
Bu IP kime ait? Yerel mi? Genel mi? Hangi ülkeden? VPN mi?

#### 📡 Protokolü Tanı:
TCP mi UDP mi? ICMP mi? Ne tür bir bağlantı kurulmaya çalışılıyor?

#### ⏰ Zaman Damgalarını Bağla:
Bu trafik saat 03:00'te mi geliyor? Bu normal mi?

### 3. 🔎 Çevirince Neyi İnceleyeceksin ki Suçlu Olduğunu Anlayasın? (Tespit)

#### 🚨 Anormallik Ara:
Normalde hiç gitmediğin bir IP'ye bilgisayarın sürekli paket mi gönderiyor? Bu şüpheli.

#### 🔍 Tarama Desenleri:
Aynı IP'den saniyeler içinde yüzlerce farklı porta bağlantı geliyorsa — bu bir port taramasıdır (recon). Saldırgan "kapı yokluyor" demektir.

#### 🎭 Banner Toplama:
Birisi gelip 22, 80, 443 portlarına sırayla bağlanıp "Kimsin?" diye soruyorsa — bu banner grabbing'dir. Keşif yapıyor.

#### 🔐 Bilinen İmzalar (Signature):
Giden paketlerin içinde "/etc/passwd", "admin", "SELECT * FROM" gibi kelimeler var mı? Bunlar saldırı desenleridir.

#### 📈 Davranışsal Analiz:
Normalde 5 dakikada 10 bağlantı yapan, şimdi 1 dakikada 200 bağlantı mı var? Trafik fırtınası — bu DDoS veya tarama olabilir.

### 4. 🚔 Polis Mantığıyla Özet:

#### 📍 Olay Yeri:
Ağ trafiğinin geçtiği nokta (network interface)

#### 🔍 Delil:
Gelen/giden paketler (raw packets)

#### 🧪 Tercüman:
Wireshark, tcpdump, snort gibi araçlar (çevirini senin için yapar)

#### 📋 Kanıt:
Anormal desenler, taramalar, bilinen saldırı imzaları, zaman dışı trafik

#### 📝 Tutuklama:
Tespit ettiğin anomaliyi raporla, kaynağını engelle (firewall kuralı)

---

Yanı kod yazmadan önce konuyu anlamak neye, ağ güvenliğinde de elindeki ham veriyi anlamak odur. Önce çevir, sonra analiz et, sonra karar ver.

**Unutma**: "Önce anla, sonra kodla" her zaman işe yarar.