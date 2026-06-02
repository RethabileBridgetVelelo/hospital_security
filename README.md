# 🏥 Motherly Institute: Secure IoMT Gateway

An ultra-secure, memory-safe data entry and triage routing system written in **Rust** for hospital environments. This application acts as an intelligent gatekeeper for Internet of Medical Things (IoMT) devices—such as ICU bed monitors—ensuring all incoming patient telemetry data is strictly validated, clear of memory injection vectors, and structured into human-readable triage dashboards.

---

## 🛡️ Cyber-Medical Security Features

* **Compile-Time Memory Protection:** Leverages Rust's core strict ownership rules to completely eliminate buffer overflows, data races, and use-after-free software vulnerabilities without needing a performance-throttling garbage collector.
* **Malicious Payload Defense Matrix:** Employs an isolated security engine boundaries check (`security_engine.rs`) that drops malformed, out-of-bounds, or poisoned input data packets before they hit the database heap.
* **Automated Triage Classification:** Dynamically maps patient diagnostics across a 3-tier severity matrix (`STABLE`, `ELEVATED`, `CRITICAL EMERGENCY`) to immediately surface high-risk patients.
* **Crash-Proof Error Handling:** Uses Rust's explicit `Result` and `Option` enums to catch input formatting mistakes gracefully, keeping critical hospital systems online without downtime.

---

## 📁 Multi-File System Architecture

The project is structured modularly to isolate internal validation systems from external data collection operations:

```text
hospital_security/
├── Cargo.toml                # Project configuration & operational metadata
└── src/
    ├── main.rs               # Interactive CLI control dashboard & input interface
    └── security_engine.rs    # Core validation logic & secure data structures
