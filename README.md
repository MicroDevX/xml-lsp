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

## 🚀 Getting Started

### Requirements

* **Rust** `1.99.0` or newer
* **Cargo**
* An LSP-compatible editor or client for integration

You can verify your Rust installation with:

```bash
rustc --version
cargo --version
```

### Build from Source

Clone the repository:

```bash
git clone https://github.com/MicroDevX/xml-lsp.git
cd xml-lsp
```

Build the project in release mode:

```bash
cargo build --release
```

The resulting executable will be available at:

```text
target/release/xml-lsp
```

### Run the Server

```bash
./target/release/xml-lsp --start
```

The server communicates through **stdio** and can therefore be integrated with LSP-compatible editors and development environments.

---

## 🧩 Current Features

The project is actively evolving. Current and planned functionality is organized around the following components:

### XML Processing

* XML document parsing
* Document tree construction
* XML diagnostics
* Document formatting
* Indentation handling

### LSP Features

* Tag completion
* Attribute completion
* Hover information
* Symbol discovery
* Document outline
* XML tag renaming

### Schema Support

* DTD parsing
* DTD validation
* XSD parsing / validation
* Schema-aware completion
* Schema-aware diagnostics

> ⚠️ Feature completeness and stability may change rapidly while the project is under active development.

---

## 🗺️ Roadmap

The roadmap is intentionally iterative and may evolve as the architecture matures.

### Core XML Engine

* [ ] Robust XML parsing
* [ ] Incremental document updates
* [ ] Improved diagnostics
* [ ] XML formatting
* [ ] Namespace handling
* [ ] Entity handling

### LSP

* [ ] Completion
* [ ] Hover
* [ ] Rename
* [ ] Document symbols
* [ ] Go to definition
* [ ] Find references
* [ ] Code actions
* [ ] Diagnostics
* [ ] Formatting
* [ ] Semantic tokens

### Schema Support

* [ ] DTD parsing
* [ ] DTD validation
* [ ] XSD parsing
* [ ] XSD validation
* [ ] Schema-aware completion
* [ ] Schema-aware hover information
* [ ] Schema discovery and resolution

### Performance & Reliability

* [ ] Incremental parsing
* [ ] Performance benchmarks
* [ ] Memory profiling
* [ ] Large-document testing
* [ ] Comprehensive integration tests
* [ ] Fuzz testing

### Editor Integration

* [ ] Neovim
* [ ] Helix
* [ ] VS Code
* [ ] Zed
* [ ] Other LSP-compatible editors

---

## 🔬 Development

Build the project:

```bash
cargo build
```

Run in development mode:

```bash
cargo run -- --start
```

Run tests:

```bash
cargo test
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Format the code:

```bash
cargo fmt
```

Check formatting without modifying files:

```bash
cargo fmt -- --check
```

---

## 🤝 Contributing

Contributions are welcome.

Areas where contributions can be particularly valuable include:

* XML parser improvements
* XSD implementation
* DTD implementation
* LSP feature development
* Incremental parsing
* Diagnostics
* Performance optimization
* Test coverage
* Editor integrations
* Documentation
* Bug reports and reproducible test cases

### Contribution Workflow

1. Fork the repository.
2. Create a feature branch.
3. Implement your changes.
4. Add or update tests where appropriate.
5. Run formatting and checks.
6. Commit your changes.
7. Open a Pull Request.

Example:

```bash
git checkout -b feature/my-feature

cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings

git add .
git commit -m "feat: implement my feature"
git push origin feature/my-feature
```

Then open a Pull Request against the main repository.

---

## 📋 Project Status

`xml-lsp` is **experimental software**.

The architecture, APIs, internal modules, and feature set may change without notice while the project is being developed.

It should currently be considered a **development and research project**, rather than a production-ready XML language server.

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
