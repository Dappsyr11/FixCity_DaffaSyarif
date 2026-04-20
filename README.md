# 🛣️ JalanKu — Indonesia Infrastructure Complaint Platform

A road damage and public facility complaint website built with **Rust → WebAssembly (WASM)**
using the **Yew** framework (React-like framework for Rust).

---

## 🏗️ Architecture

```text
jalanku/
├── src/
│   ├── lib.rs               # WASM entry point (wasm_bindgen_start)
│   ├── main.rs              # Placeholder binary (for cargo check)
│   ├── models.rs            # Data types: Report, User, Status, etc.
│   ├── data.rs              # Sample data & statistics
│   └── components/
│       ├── mod.rs
│       ├── navbar.rs        # Top navigation
│       ├── beranda.rs       # Homepage (hero section, statistics, CTA)
│       ├── kartu_laporan.rs # Reusable report card component
│       ├── buat_laporan.rs  # 3-step new report form
│       ├── daftar_laporan.rs # Report list + filters
│       ├── dashboard.rs     # Government dashboard + analytics
│       └── profil.rs        # User profile + point reward system
├── static/
│   └── style.css            # Complete stylesheet
├── index.html               # HTML entry for Trunk
├── Trunk.toml               # Build configuration
└── Cargo.toml               # Rust dependencies
```

---

## 🚀 Build & Run

### 1. Install Rust (via rustup)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Add WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Trunk (build tool for Yew)

```bash
cargo install trunk
```

### 4. Run development server

```bash
cd jalanku
trunk serve
```

The browser will automatically open at: **http://localhost:8080**

### 5. Build for production

```bash
trunk build --release
# Output will be generated in the dist/ folder
```

---

## 📦 Main Dependencies

| Crate          | Version | Function                          |
| -------------- | ------- | --------------------------------- |
| `yew`          | 0.21    | Rust/WASM UI framework            |
| `wasm-bindgen` | 0.2     | Rust ↔ JavaScript binding         |
| `web-sys`      | 0.3     | Web APIs (DOM, Geolocation, etc.) |
| `js-sys`       | 0.3     | JavaScript built-ins              |
| `serde`        | 1       | JSON serialization                |
| `chrono`       | 0.4     | Date & time handling              |
| `uuid`         | 1       | UUID generator                    |
| `gloo-*`       | various | Helper utilities                  |

---

## 🎨 Application Features

### Homepage

* Hero section with animated map
* Real-time statistics (total reports, in progress, completed, users)
* Top 3 priority reports (highest public support)
* 4-step reporting guide
* Government institution explanation (PUPR, Transportation Office, Environmental Office)
* CTA section with point reward system

### Create Report Form (3 Steps)

* **Step 1**: Title, category, severity, description
* **Step 2**: Address, GPS location detection, photo upload
* **Step 3**: Confirmation + institution info + reward points

### Report List

* Filter by: Status, Category, Keyword
* Sort by: Most Popular, Newest, Severity
* Priority #1 report banner
* Interactive report cards with support button

### Government Dashboard

* KPI cards: total, pending, in progress, completed
* Category distribution bar chart
* Citizen participation statistics
* Institution division (PUPR / Transportation Office / Environmental Office)
* Report filter tabs

### User Profile

* Avatar with level badge
* Progress bar to next level
* Statistics: reports, completed fixes, points, ranking
* Level system: Beginner → Contributor → Active Citizen → City Hero → Legend
* Reward explanation
* User’s report history

---

## 🔧 System Concepts

| Concept                | Implementation                                                                           |
| ---------------------- | ---------------------------------------------------------------------------------------- |
| Priority system        | Sorted by number of citizen supports                                                     |
| Institution assignment | Automatically assigned by category (PUPR / Transportation Office / Environmental Office) |
| Status transparency    | Status badge: Pending / In Progress / Completed                                          |
| Fake report prevention | Quantity validation + public support system                                              |
| Reward points          | +25 submit, +50 completed, +5 per support                                                |
| User roles             | Citizens & Government (different dashboards)                                             |
| Geolocation            | GPS detection + interactive map pin                                                      |

---

## 📱 Responsive Design

Supports multiple screen sizes: Desktop → Tablet → Mobile

---

Contract ID: CCFJJQ7B7EWXVKY2VHCH27DPMOKTCNLRO2ODLP7S7XH25TGC7T5H6OID
