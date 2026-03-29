# 🛡️ OpenSCAP Compliance Hub

**A self-hostable, multi-tenant web dashboard for Managed Security Service Providers (MSSPs) to track Linux system compliance using raw OpenSCAP ARF XML files.**

Built from the ground up in modern Rust following the NIST Secure SDLC (SP 800-160 Rev. 1). Designed to be packaged and submitted to the Fedora Project as both an RPM (systemd service) and Flatpak.

![Dashboard Screenshot]()  


## ✨ Current Planned Features (NIST Development Phase)

- ✅ Secure Axum + Tokio web server (zero-cost, memory-safe)
- ✅ Beautiful server-rendered dashboard with Askama + Tailwind CSS + HTMX + Alpine.js
- ✅ Full compile-time safety (`cargo clippy --all-targets --all-features -- -D warnings`)
- ✅ Ready for multi-tenant organizations, hosts, and ARF scan uploads
- ✅ Open-source friendly (GPL-3.0-or-later) and Fedora packaging ready

## 🛠 Tech Stack (fixed — never changed without explicit approval)

- **Backend**: Rust 2021 + Axum 0.7 + Tower + SQLx (PostgreSQL)
- **Templating & UI**: Askama + HTMX + Tailwind CSS + Alpine.js (no heavy JS frameworks)
- **ARF Parsing**: quick-xml + Serde
- **Auth**: Session-based RBAC (coming next)
- **Database**: PostgreSQL (multi-tenant schema)
- **Packaging**: RPM (systemd) + Flatpak

## 🚀 Quick Start (Development on Fedora Minimal Server)

1. Clone the repo:
   ```bash
   git clone https://github.com/AvalonWaveSoftware/Avalon-Scap-Compliance-Hub.git
   cd Avalon-Scap-Compliance-Hub