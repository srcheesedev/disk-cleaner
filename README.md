# Disk Cleaner

Interactive directory size analyzer and cleanup tool. A high-performance alternative to `du -d1 -h` with multi-select capabilities similar to `fzf`.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/tests-20%20passing-green.svg)](#testing)
[![CI](https://github.com/srcheesedev/disk-cleaner/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/srcheesedev/disk-cleaner/actions)

## 🚀 Quick Start

```bash
# Download latest release
wget https://github.com/srcheesedev/disk-cleaner/releases/latest/download/disk-cleaner-x86_64-unknown-linux-gnu.tar.gz
tar -xzf disk-cleaner-x86_64-unknown-linux-gnu.tar.gz
./install.sh

# Or build from source
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner
./install.sh

# Usage
disk-cleaner                    # Analyze current directory
disk-cleaner /path/to/analyze   # Analyze specific directory
disk-cleaner --help             # Show all options
```

## ✨ Features

- 🔍 **Fast directory analysis** - Concurrent file size calculation
- 📋 **Interactive multi-select** - Choose multiple files/directories
- 🛡️ **Safe deletion** - Confirmation prompts and error handling
- 📊 **Clear visualization** - Human-readable sizes and organized tables
- ⚡ **High performance** - Built in Rust for maximum speed
- 🎯 **Flexible filtering** - By size, type, and depth
- 📱 **Cross-platform** - Works on Linux, macOS, and Windows

## 🎯 Use Cases

- Clean up large directories (Downloads, temp folders)
- Find space-consuming files before system cleanup
- Analyze project directories for optimization
- Remove old build artifacts and cache files
- Identify duplicate or unnecessary files

## 📋 Prerequisites

- **Rust 1.70+** (automatically installed by `install.sh`)
- **Modern terminal** with color support (recommended)

## 🔧 Installation

### Option 1: Automatic Installation (Recommended)
```bash
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner
./install.sh
```

### Option 2: Manual Installation
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner
cargo build --release

# Run
./target/release/disk-cleaner-rs --help
```

## 📖 Usage Examples

### Basic Analysis
```bash
# Analyze current directory
disk-cleaner

# Analyze specific directory
disk-cleaner /home/user/Downloads
```

### Advanced Filtering
```bash
# Show only files larger than 10MB
disk-cleaner --min-size 10485760

# Show only directories
disk-cleaner --dirs-only

# Show only files
disk-cleaner --files-only

# Analyze with deeper recursion
disk-cleaner --depth 2 /var/log
```

### Interactive Workflow
1. **Run analysis**: `disk-cleaner /path/to/clean`
2. **Review results**: Sorted table by size
3. **Select items**: Use SPACE to select, ENTER to confirm
4. **Confirm deletion**: Review selection and confirm
5. **See results**: View freed space and any errors

## 🏗️ Architecture

### Project Structure
```
src/
├── main.rs         # CLI interface and orchestration
├── analyzer.rs     # Directory analysis and size calculation
└── file_manager.rs # User interaction and file operations
```

### Design Principles
- **Test-Driven Development** - 20 comprehensive tests
- **Memory Safety** - Rust's ownership system prevents crashes
- **Concurrency** - Async analysis for better performance
- **Error Handling** - Graceful handling of permissions and missing files

## 🧪 Testing

Run the comprehensive test suite:
```bash
cargo test
```

**Test Coverage:**
- ✅ **20 tests** covering core functionality
- ✅ **Unit tests** for individual components
- ✅ **Integration tests** for component interaction
- ✅ **Edge cases** for error conditions
- ✅ **CLI parsing** for user input validation

See [TESTS.md](TESTS.md) for detailed testing documentation.

## 🔒 Safety Features

- **Confirmation prompts** before any deletion
- **Validation checks** ensure files exist before deletion
- **Partial failure handling** continues operation when some files fail
- **Clear error messages** explain what went wrong
- **Non-destructive analysis** - read-only directory scanning

## ⚡ Performance

- **Concurrent analysis** - Multiple directories processed in parallel
- **Memory efficient** - Streaming file processing
- **Fast startup** - Compiled binary with minimal overhead
- **Scalable** - Handles directories with thousands of files

## 🆚 Comparison with Alternatives

| Feature | disk-cleaner | `du + rm` | GUI tools |
|---------|-------------|-----------|-----------|
| Interactive selection | ✅ | ❌ | ✅ |
| Multi-select | ✅ | ❌ | ✅ |
| Performance | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| Safety | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| CLI integration | ✅ | ✅ | ❌ |
| Cross-platform | ✅ | ❌ | ✅ |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development Setup
```bash
git clone https://github.com/srcheesedev/disk-cleaner.git
cd disk-cleaner
cargo build
cargo test
```

### Code Standards
- Follow Rust conventions (`cargo fmt`, `cargo clippy`)
- Add tests for new functionality
- Update documentation for user-facing changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by `du`, `fzf`, and other Unix utilities
- Built with amazing Rust crates: `clap`, `tokio`, `dialoguer`, `humansize`
- Developed following Test-Driven Development principles

## ⚠️ Disclaimer

**USE AT YOUR OWN RISK**: This tool can permanently delete files. Always review your selections carefully and ensure you have backups of important data. The authors are not responsible for any data loss.

---

*Made with ❤️ and ⚡ Rust*