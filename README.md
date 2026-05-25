# CVE-2025-21082: HyperOS AVCodec Use-After-Free

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

A comprehensive analysis and proof-of-concept demonstration of the Use-After-Free vulnerability in Xiaomi HyperOS AVCodec framework (CVE-2025-21082).

## 📋 Overview

This repository contains a detailed security analysis of CVE-2025-21082, a critical Use-After-Free vulnerability discovered in Xiaomi HyperOS's AVCodec media processing framework. The vulnerability allows attackers to achieve remote code execution through heap manipulation and race condition exploitation.

## 🏗️ Repository Structure

| Directory/File | Description |
|----------------|-------------|
| `docs/` | Comprehensive vulnerability analysis and documentation |
| `poc_python/` | Python-based analysis tools and utilities |
| `poc_rust/` | Rust-based UAF simulation and demonstration |
| `simulation.html` | Interactive web-based UAF race condition visualization |

## 🚀 Quick Start

### Prerequisites

- Python 3.8+
- Rust 1.70+

### Running the Rust UAF Simulation

```bash
cd poc_rust
cargo build --release

# Test vulnerable scenario
./target/release/cve_2025_21082_uaf_poc --mode vulnerable --verbose

# Test patched scenario  
./target/release/cve_2025_21082_uaf_poc --mode patched --verbose
```

### Python Analysis Tools

```bash
cd poc_python
pip install -r requirements.txt
python exploit.py --analyze --target hyperos_avcodec
```

## 📚 Documentation

- [Vulnerability Analysis](docs/analysis.md) - Detailed CVE-2025-21082 technical analysis
- [Architecture Overview](docs/architecture.md) - HyperOS AVCodec pipeline and attack vectors  
- [Mitigation Strategies](docs/mitigation.md) - Recommended patches and defensive measures

## ⚠️ Disclaimer

This project is for educational and research purposes only. The Rust simulation demonstrates the vulnerability mechanism but does not constitute a working exploit. Do not use these tools against systems you do not own or have explicit permission to test.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
