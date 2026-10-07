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