# xml-lsp 🦀⚡

A lightweight, high-performance **XML Language Server Protocol (LSP)** implementation written entirely in **Rust**.

> ⚠️ **Project Status: Under Active Development**
>
> `xml-lsp` is currently in an early experimental stage and is **not yet suitable for production use**. Core features are actively being implemented, tested, and refined.

> 🤝 **Contributions Welcome!**
>
> Contributions are welcome across XML parsing, schema validation (**XSD/DTD**), LSP capabilities, performance optimization, testing, and architecture. If you are interested in Rust, parsers, XML tooling, or language servers, feel free to open an issue, submit a pull request, or start a discussion.

---

## ✨ Overview

`xml-lsp` aims to provide a modern, lightweight, and efficient XML development experience through the **Language Server Protocol**.

The project is designed from the ground up in Rust with a focus on:

* ⚡ Fast startup and execution
* 🦀 Native Rust performance
* 🧠 Low memory overhead
* 🧩 Modular architecture
* 📄 XML parsing and diagnostics
* 🔍 Schema-aware tooling
* 🛠️ LSP-based editor integration
* 🚀 Extensibility for future XML tooling

The long-term goal is to provide a capable XML language server without requiring a JVM runtime or the overhead associated with larger implementations.

---

## ⚡ Key Highlights

| Feature / Aspect            | **xml-lsp**                   | **Eclipse LemMinX**     |
| :-------------------------- | :---------------------------- | :---------------------- |
| **Implementation Language** | Rust 🦀                       | Java ☕                  |
| **Runtime**                 | Native executable             | JVM                     |
| **Startup**                 | Fast / near-instant           | JVM startup overhead    |
| **Memory Footprint**        | Designed for low memory usage | Higher runtime overhead |
| **Architecture**            | Modular Rust components       | Eclipse ecosystem       |
| **License**                 | MIT                           | EPL-2.0                 |
| **XML Tooling**             | Actively developing           | Mature                  |
| **Production Readiness**    | 🚧 Experimental               | ✅ Production-oriented   |

### Why Rust?

Rust provides several properties that make it particularly suitable for a language server:

* Native execution without a managed runtime
* Predictable memory usage
* Strong compile-time guarantees
* Excellent performance for parsing and analysis
* Efficient concurrency primitives
* Easy distribution as a standalone binary
* Strong ecosystem for parsers and developer tooling

---

## 🏗️ Architecture

`xml-lsp` follows a modular architecture where XML processing, schema validation, and LSP functionality are isolated into dedicated components.

```text
src
├── main.rs
│   └── CLI entry point
│       ├── clap integration
│       └── stdio setup
│
├── server.rs
│   └── LSP server backend
│       ├── Tower-LSP integration
│       └── request / notification handlers
│
├── features/
│   ├── mod.rs
│   ├── completion.rs
│   │   └── Tag & attribute completion
│   │
│   ├── hover.rs
│   │   └── Schema & documentation information
│   │
│   ├── rename.rs
│   │   └── XML tag renaming
│   │
│   └── symbols.rs
│       └── Document symbols & outline tree
│
├── schema/
│   ├── mod.rs
│   ├── dtd.rs
│   │   └── DTD parser & validator
│   │
│   └── xsd.rs
│       └── XSD validation engine
│
└── xml/
    ├── mod.rs
    ├── parser.rs
    │   └── XML document tree & diagnostics
    │
    └── formatting.rs
        └── XML indentation & formatting
```

### Architectural Layers

```text
┌─────────────────────────────────────┐
│             LSP Client               │
│ VS Code • Neovim • Helix • Zed • ...│
└──────────────────┬──────────────────┘
                   │
                   │ LSP / JSON-RPC
                   ▼
┌─────────────────────────────────────┐
│            LSP Server                │
│             server.rs                │
└──────────────────┬──────────────────┘
                   │
          ┌────────┴────────┐
          ▼                 ▼
┌─────────────────┐  ┌─────────────────┐
│    Features     │  │   XML Engine    │
│                 │  │                 │
│ Completion      │  │ Parser          │
│ Hover           │  │ Diagnostics     │
│ Rename          │  │ Formatting      │
│ Symbols         │  │                 │
└────────┬────────┘  └────────┬────────┘
         │                    │
         └──────────┬─────────┘
                    ▼
          ┌───────────────────┐
          │ Schema Validation │
          │                   │
          │ XSD               │
          │ DTD               │
          └───────────────────┘
```

---

## 📄 License

This project is licensed under the **MIT License**.

See the [`LICENSE`](LICENSE) file for the complete license text.

---

## 🔗 Related Projects

* **Eclipse LemMinX** — [XML Language Server](https://github.com/eclipse-lemminx/lemminx)
* **Language Server Protocol** — [Microsoft LSP Specification](https://microsoft.github.io/language-server-protocol/)
* **Rust** — [The Rust Programming Language](https://www.rust-lang.org/)
* **Tower-LSP** — Rust implementation for building LSP servers

---

## ⭐ Project

If you find `xml-lsp` useful or are interested in its development, consider starring the repository and contributing to its evolution.

**Repository:** https://github.com/MicroDevX/xml-lsp

> 🦀 Built with Rust.
> ⚡ Designed for speed.
> 🧩 Built for modern development environments.
