<div align="center">

# 🧹 Disk Cleaner

[![CI/CD Pipeline](https://github.com/srcheesedev/disk-cleaner/actions/workflows/ci.yml/badge.svg)](https://github.com/srcheesedev/disk-cleaner/actions/workflows/ci.yml)
[![Security Audit](https://github.com/srcheesedev/disk-cleaner/actions/workflows/security.yml/badge.svg)](https://github.com/srcheesedev/disk-cleaner/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Cross Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/srcheesedev/disk-cleaner/releases)

**A lightning-fast, interactive disk space analyzer and cleanup tool built with Rust** ⚡

*Find space hogs, clean up clutter, and reclaim your storage with confidence*

</div>

---

## ✨ Features

<div align="center">

| 🚀 **Performance** | 🎯 **Interactive** | 🔒 **Safe** | 🌍 **Cross-Platform** |
|:---:|:---:|:---:|:---:|
| Async directory scanning | Multi-select interface | Permission validation | Windows, Linux, macOS |
| Blazing fast analysis | Intuitive UI/UX | Confirmation flows | Native binaries |
| Memory efficient | Real-time feedback | Atomic operations | Platform-specific handling |

</div>

### 🎯 **Core Capabilities**

- **📊 Smart Analysis**: Recursive directory scanning with configurable depth limits
- **🎮 Interactive Selection**: Beautiful multi-select interface powered by `fzf`-like functionality
- **🛡️ Safety First**: Comprehensive permission checking and validation before any operations
- **💾 Human-Readable**: Elegant size formatting (KB, MB, GB, TB) with intelligent rounding
- **⚡ High Performance**: Async I/O operations that don't block on large filesystems
- **🔍 Advanced Filtering**: Filter by size thresholds, file types, and directory depth
- **📈 Detailed Reporting**: Comprehensive success/failure reporting with specific error messages

### 🌟 **Why Disk Cleaner?**

- **🎯 Purpose-Built**: Designed specifically for interactive disk cleanup workflows
- **🔒 Production Ready**: Comprehensive testing with 25+ test cases and CI/CD pipeline
- **⚡ Rust Performance**: Memory-safe, blazingly fast, with zero-cost abstractions
- **🎨 Beautiful UX**: Intuitive interface that makes disk cleanup actually enjoyable
- **🌍 Universal**: Single binary that works identically on all major platforms

---

## 🚀 Quick Start

### 📦 Installation

#### **Option 1: Download Pre-built Binaries** *(Recommended)*

```bash
# Download the latest release for your platform
curl -L https://github.com/srcheesedev/disk-cleaner/releases/latest/download/disk-cleaner-linux.tar.gz | tar xz
./disk-cleaner-rs --help
```

#### **Option 2: Install from Source**

```bash
# Requires Rust 1.70+
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner
cargo install --path .
```

#### **Option 3: Use the Install Script**

```bash
# Auto-detects your platform and installs to ~/.local/bin
curl -fsSL https://github.com/srcheesedev/disk-cleaner/raw/main/install.sh | bash
```

### 🎮 Basic Usage

```bash
# Analyze current directory
disk-cleaner

# Analyze specific directory with custom depth
disk-cleaner /path/to/analyze --depth 3

# Find files larger than 100MB
disk-cleaner --min-size 104857600

# Show only directories
disk-cleaner --dirs-only

# Show only files
disk-cleaner --files-only
```

---

## 📖 Documentation

### 🎯 **Usage Examples**

<details>
<summary><b>🔍 Basic Directory Analysis</b></summary>

```bash
# Analyze current directory (depth 1)
$ disk-cleaner
🔍 Disk Cleaner - Interactive Directory Analysis
📁 Analyzing: /home/user/Downloads

   SIZE TYPE NAME
  2.3 GB  DIR  node_modules
  1.1 GB FILE large_video.mp4
  456 MB  DIR  old_projects
  123 MB FILE presentation.pptx
   45 MB  DIR  images
```

</details>

<details>
<summary><b>🎮 Interactive Selection Interface</b></summary>

```bash
# After analysis, select files to delete
Select items to delete:
❯ ◯ 2.3 GB  DIR  node_modules
  ◯ 1.1 GB FILE large_video.mp4  
  ◯ 456 MB  DIR  old_projects
  ◯ 123 MB FILE presentation.pptx
  ◯ 45 MB   DIR  images

# Use space to select, enter to confirm
✓ 2 items selected (2.8 GB total)
```

</details>

<details>
<summary><b>🛡️ Safety Confirmations</b></summary>

```bash
🚨 WARNING: The following items will be permanently deleted:

  2.3 GB  DIR  /home/user/Downloads/node_modules
  1.1 GB FILE /home/user/Downloads/large_video.mp4

📊 Total size: 3.4 GB
❓ Are you sure you want to delete these 2 items? (y/N)
```

</details>

<details>
<summary><b>📊 Advanced Filtering</b></summary>

```bash
# Find space hogs larger than 1GB
disk-cleaner --min-size 1073741824 --depth 5

# Analyze only directories (exclude individual files)
disk-cleaner --dirs-only /var/log

# Find large files in home directory
disk-cleaner ~/Downloads --files-only --min-size 50000000
```

</details>

### ⚙️ **Command Line Options**

| Option | Description | Example |
|--------|-------------|---------|
| `path` | Directory to analyze | `disk-cleaner /home/user` |
| `--depth, -d` | Maximum depth to scan | `--depth 3` |
| `--min-size, -m` | Minimum size filter (bytes) | `--min-size 104857600` |
| `--dirs-only` | Show only directories | `--dirs-only` |
| `--files-only` | Show only files | `--files-only` |
| `--help, -h` | Show help information | `--help` |
| `--version, -V` | Show version | `--version` |

### 📏 **Size Format Examples**

| Bytes | Human Readable | Use Case |
|-------|----------------|----------|
| `1048576` | 1.0 MB | Small files |
| `104857600` | 100 MB | Medium files |
| `1073741824` | 1.0 GB | Large files |
| `10737418240` | 10 GB | Very large files |

---

## 🏗️ Development

### 🛠️ **Building from Source**

```bash
# Clone the repository
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner

# Build optimized release binary
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

### 🧪 **Testing**

```bash
# Run all tests with verbose output
cargo test --verbose

# Run specific test module
cargo test analyzer::tests

# Run integration tests
cargo test --test integration

# Generate test coverage report
cargo tarpaulin --out Html
```

### 📊 **Code Quality**

The project maintains high code quality standards:

- **✅ 25+ Comprehensive Tests**: Unit, integration, and platform-specific tests
- **🔍 Clippy Clean**: Zero warnings with strict linting rules
- **📝 100% Documented**: Every public API has comprehensive documentation
- **🔒 Security Audited**: Regular security audits with `cargo audit`
- **🎯 Memory Safe**: Rust's ownership system prevents common bugs

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### 🐛 **Bug Reports**

Found a bug? Please [open an issue](https://github.com/srcheesedev/disk-cleaner/issues/new?template=bug_report.md) with:

- **Environment**: OS, version, and hardware details
- **Steps to Reproduce**: Clear, numbered steps
- **Expected vs Actual**: What should happen vs what happens
- **Logs**: Any error messages or relevant output

### 💡 **Feature Requests**

Have an idea? [Request a feature](https://github.com/srcheesedev/disk-cleaner/issues/new?template=feature_request.md) with:

- **Use Case**: Why this feature would be valuable
- **Implementation Ideas**: Technical approach if you have one
- **Examples**: Similar features in other tools

### 🔄 **Pull Requests**

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes with tests
4. Ensure CI passes: `cargo test && cargo clippy`
5. Submit a pull request

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **🦀 Rust Community**: For the amazing ecosystem and tools
- **📦 Crate Authors**: Especially `clap`, `tokio`, `dialoguer`, and `walkdir`
- **🧪 Contributors**: Everyone who has contributed code, issues, and feedback
- **💡 Inspiration**: Tools like `du`, `ncdu`, and `fzf` that inspired this project

---

<div align="center">

**Made with ❤️ and ⚡ Rust**

[⭐ Star us on GitHub](https://github.com/srcheesedev/disk-cleaner) • [📥 Download Latest Release](https://github.com/srcheesedev/disk-cleaner/releases) • [🐛 Report Issues](https://github.com/srcheesedev/disk-cleaner/issues)

</div>