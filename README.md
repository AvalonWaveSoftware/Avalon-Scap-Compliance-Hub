# 🛡️ OpenSCAP Compliance Hub(In Development)

**A self-hostable, multi-tenant web dashboard for Managed Security Service Providers (MSSPs) to track Linux system compliance using raw OpenSCAP ARF XML files.**

Built in modern Rust following NIST Secure SDLC practices. Designed to be packaged for Fedora as RPM + Flatpak.

## Current Status (Early Development)

The core dashboard is functional with a clean Linode/Foreman-style UI. You can:
- Create and list Organizations (Clients)
- Create Host Groups and Hosts (with organization dropdown)
- Click an Organization card to drill down and see its Host Groups and Hosts
- HTMX-powered modals for all create actions with instant refresh

**Known limitations / work in progress:**
- Top summary stats row is still static (hard-coded)
- Host-to-group membership assignment is not yet implemented
- ARF XML parser and scan upload is paused
- Some presentation polish still needed (CSS/layout tweaks)
- No authentication/RBAC yet

## ✨ Implemented Features

- ✅ Professional Foreman-style dashboard (fixed sidebar + top navbar + summary stats row)
- ✅ Multi-tenant schema (Organizations → Host Groups → Hosts)
- ✅ Full interactive CRUD for Organizations, Host Groups, and Hosts
- ✅ Clickable organization cards with drill-down view
- ✅ Modular code structure (`src/models.rs` + `src/handlers.rs` + short `main.rs`)
- ✅ PostgreSQL with peer authentication (secure, no passwords in code)
- ✅ Clean, warning-free build (`cargo clippy --all-targets --all-features -- -D warnings`)

## 🛠 Tech Stack

- **Backend**: Rust + Axum + SQLx (PostgreSQL)
- **Templating/UI**: Askama + HTMX + Tailwind CSS + Alpine.js (server-rendered)
- **Database**: PostgreSQL with multi-tenant schema

## 🚀 Quick Start (Fedora)

```bash
cargo build
runhub   # or sudo -u oscap-hub ./target/debug/avalon-scap-compliance-hub
```

Open http://your-ip:3000

📋 Roadmap

Dynamic top summary statistics
Host → Host Group membership assignment
ARF XML parser + secure scan upload
Per-host and per-group compliance reports
Session-based RBAC
Plugin system
Fedora RPM + Flatpak packaging

🤝 Contributing
Contributions welcome! The project is early-stage but has a solid, modular foundation.

📜 License
BSD 3
