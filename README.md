[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/peaceara777-star/rust-wifi?style=social)](https://github.com/peaceara777-star/rust-wifi)

# 🛜 Rust-WiFi - Professional WiFi Toolkit

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()

<div dir="rtl">

## 📡 أداة احترافية لتشخيص وإصلاح مشاكل الواي فاي

أداة مكتوبة بلغة Rust لتشخيص وإصلاح جميع مشاكل الواي فاي بواجهة رسومية عصرية.

</div>

## ✨ Features | المميزات

- 📡 **Network Scanner**: Scan and display all nearby WiFi networks with signal strength
- 🔧 **Diagnostics**: Comprehensive connection testing (Gateway, DNS, Internet)
- 🛠️ **Repair Tools**: TCP/IP reset, DNS management, cache clearing
- 📊 **Channel Analyzer**: Find the best WiFi channel to avoid interference
- 🌙 **Dark/Light Mode**: Full theme support
- 🌍 **RTL Support**: Arabic interface support
- 📄 **Report Export**: Save diagnostic reports in JSON format

## 📸 Screenshots

| Dashboard | Scanner | Diagnostics |
|-----------|---------|-------------|
| ![Dashboard](screenshots/dashboard.png) | ![Scanner](screenshots/scanner.png) | ![Diagnostics](screenshots/diagnostics.png) |

## 📦 Installation | التثبيت

### Prerequisites
- Rust 1.75 or higher
- Cargo

```bash
# Clone the repository
git clone https://github.com/peaceara777-star/rust-wifi.git
cd rust-wifi

# Build the project
cargo build --release

# Run the application
cargo run --release# Development mode
cargo run

# Production mode
cargo run --release

# Build standalone executable
cargo build --release
# Find executable in target/release/rust-wifi/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Main application
│   ├── pages/           # UI Pages
│   │   ├── dashboard.rs
│   │   ├── scanner.rs
│   │   ├── diagnostics.rs
│   │   ├── tools.rs
│   │   └── advanced.rs
│   ├── wifi/            # WiFi logic
│   │   ├── scanner.rs
│   │   ├── diagnostics.rs
│   │   └── utils.rs
│   └── ui/              # UI components
│       ├── theme.rs
│       └── widgets.rs
├── assets/              # Icons and fonts
├── Cargo.toml           # Dependencies
├── LICENSE              # MIT License
└── README.md            # Documentation🔧 Platform Support | دعم المنصات

Feature Windows Linux macOS
Network Scanning ✅ ✅ 🚧
Ping Tests ✅ ✅ ✅
DNS Management ✅ ✅ 🚧
TCP/IP Reset ✅ 🚧 🚧
WiFi Restart ✅ ✅ 🚧

🤝 Contributing | المساهمة

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (git checkout -b feature/AmazingFeature)
3. Commit your changes (git commit -m 'Add some AmazingFeature')
4. Push to the branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

📄 License | الترخيص

This project is dual-licensed under:

· MIT License (LICENSE)
· Apache License 2.0

👤 Author | المؤلف

peaceara777-star

· GitHub: @peaceara777-star
· Repository: rust-wifi

⭐ Show your support | الدعم

Give a ⭐️ if this project helped you!

---

<div dir="rtl">
صنع بـ 🦀 Rust و ❤️
</div>

