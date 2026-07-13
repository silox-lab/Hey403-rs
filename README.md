# Hey 403 RS 🔧

> 🦀 A Rust rewrite of [Hey 403](https://github.com/Diramid/hey403) with both CLI and GUI support.  
> ⚠️ **Work in Progress** — Not ready for production use yet.

---

## 🚧 Status

| Component | Status |
|---|---|
| Core DNS engine | 🔄 In Progress |
| CLI interface | 🔄 In Progress |
| GUI interface | 🔄 In Progress |
| Cross-platform builds | ⏳ Planned |
| Documentation | 🔄 In Progress |

---

## 🎯 What We're Building

**Hey 403 RS** is a ground-up rewrite of the original Hey 403 DNS diagnostic tool, built in **Rust** for maximum performance and safety. This version will feature:

- 🖥️ **Dual Interface** — Powerful CLI + intuitive GUI
- ⚡ **Blazing Speed** — Thanks to Rust's zero-cost abstractions
- 🌍 **Global DNS Testing** — 15+ DNS providers
- 🛡️ **Memory Safety** — No runtime crashes, no data races
- 📦 **Single Binary** — Easy distribution, no dependencies

---

## 🏗️ Tech Stack

- **Language:** Rust
- **CLI Framework:** [clap](https://github.com/clap-rs/clap) (planned)
- **GUI Framework:** [egui](https://github.com/emilk/egui) / [tauri](https://tauri.app) / [iced](https://iced.rs) (evaluating)
- **Async Runtime:** [tokio](https://tokio.rs) (planned)
- **DNS Resolution:** [trust-dns-resolver](https://github.com/bluejekyll/trust-dns) (planned)

---

## 🚀 Roadmap

- [x] Project scaffolding
- [ ] Core DNS resolution engine
- [ ] Parallel DNS testing
- [ ] CLI interface
- [ ] GUI prototype
- [ ] Cross-platform builds (Windows, macOS, Linux)
- [ ] Export formats (JSON, CSV, HTML)
- [ ] First alpha release
- [ ] First stable release

---

## 💻 Development

```bash
# Clone the repository
git clone https://github.com/Diramid/hey403-rs.git
cd hey403-rs

# Build (requires Rust toolchain)
cargo build

# Run tests
cargo test

# Run CLI (when available)
cargo run -- --help
```

---

## 🤝 Contributing

This project is in early development. Contributions, ideas, and feedback are welcome!

- 🐛 Found a bug? Open an issue
- 💡 Have an idea? Start a discussion
- 🔧 Want to contribute? Check open issues

---

## 📜 License

MIT License — same as the original [Hey 403](https://github.com/Diramid/hey403).

---

## 🔗 Related

- Original project: [Diramid/hey403](https://github.com/Diramid/hey403)
