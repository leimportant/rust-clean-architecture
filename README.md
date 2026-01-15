# Rust Clean Architecture

🚀 **Rust Backend Starter using Clean Architecture & Hexagonal Architecture (Ports and Adapters)**

Repository ini adalah starter project Rust backend yang menerapkan:
- Clean Architecture
- Hexagonal Architecture (Ports & Adapters)
- Domain-Driven Design (DDD) style
- Modular Monolith approach

Dirancang agar **mudah dikembangkan, scalable, dan testable**.

---

## ✨ Architecture Overview

Struktur project:


src/
├── main.rs
├── app.rs                     # composition root / wiring
├── config/
│   └── settings.rs            # env, db driver
│
├── domain/
│   ├── user/
|              ├── entity.rs
|              ├── repository.rs
|              ├── service.rs
|              └── mod.rs
│   ├── catalog/
|              ├── entity.rs
|              ├── repository.rs
|              ├── service.rs
|              └── mod.rs
│              
│
├── application/
│   ├── user/
|   |_______register_user.rs
|   |_______login_user.rs
|   |_______mod.rs
│   ├── catalog/
|   |_______create_catalog.rs
|   |_______mod.rs
│   └── mod.rs
│
├── adapters/
│   ├── inbound/
│   │   └── http/
│   │       ├── user.rs
│   │       └── catalog.rs
│   │
│   └── outbound/
│       └── persistence/
│           ├── mod.rs
│           ├── user_repository.rs     # SINGLE ADAPTER
│           ├── catalog_repository.rs     # SINGLE ADAPTER
│           └── sql/
│               ├── mod.rs
│               ├── mysql.rs
│               └── postgres.rs
│
├── infrastructure/
│   └── database/
│       ├── mod.rs
│       ├── mysql.rs
│       └── postgres.rs
│
└── routes.rs

## 🧠 Layer Responsibilities

### 1️⃣ Domain
- Pure business logic
- Tidak bergantung ke framework, database, atau HTTP
- Mendefinisikan **Entity**, **Repository Port**, dan **Domain Service**

### 2️⃣ Application
- Use case layer
- Mengorkestrasi domain logic
- Tidak tahu detail database / HTTP

### 3️⃣ Adapters
- **Inbound**: HTTP handlers / controllers
- **Outbound**: Implementasi repository (DB, API, dll)

### 4️⃣ Infrastructure
- Detail teknis (database driver, config, framework)

---

## 🧩 Architectural Principles

✔ Dependency Rule (arah dependency selalu ke domain)  
✔ Inversion of Control (IoC)  
✔ Test-friendly  
✔ Framework-agnostic core  
✔ Single Responsibility per module  

---

## 🚀 Getting Started

### 1️⃣ Clone repository

```bash
git clone https://github.com/leimportant/rust-clean-architecture.git
cd rust-clean-architecture
``` 
## 2️⃣ Build project
```bash
cargo build
```

## 3️⃣ Run application
```bash
cargo run
```

## 🧪 Testing
```bash
cargo test
```

### 📦 Tech Stack

Rust
Async-ready (Tokio compatible)
Database ready (MySQL / PostgreSQL)
Clean Architecture & Hexagonal Architecture

## 🛣 Roadmap
HTTP framework integration (Axum / Actix)
Database migration
Authentication & authorization
Observability (logging, tracing)
Docker support

## 🤝 Contributing
Contributions, issues, dan feature request sangat terbuka 🙌
Silakan fork dan buat pull request.