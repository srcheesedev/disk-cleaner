#!/bin/bash

# Disk Cleaner - Installation Script
# Este script instala Rust (si no está instalado) y compila el disk cleaner

set -e  # Exit on any error

echo "🔍 Disk Cleaner - Installation Script"
echo "======================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    echo "✅ Rust installed successfully!"
else
    echo "✅ Rust is already installed"
fi

# Compile the project
echo "🔨 Compiling disk-cleaner..."
cargo build --release

# Create symlink for easy access
if [ -w "/usr/local/bin" ]; then
    ln -sf "$(pwd)/target/release/disk-cleaner-rs" /usr/local/bin/disk-cleaner
    echo "✅ Created symlink: /usr/local/bin/disk-cleaner"
else
    echo "⚠️  Could not create global symlink (permission denied)"
    echo "   You can run the program with: ./target/release/disk-cleaner-rs"
fi

echo ""
echo "🎉 Installation completed!"
echo ""
echo "Usage examples:"
echo "  disk-cleaner                    # Analyze current directory"
echo "  disk-cleaner /path/to/analyze   # Analyze specific directory"
echo "  disk-cleaner --help             # Show all options"
echo ""
echo "⚠️  CAUTION: This tool can permanently delete files!"
echo "   Always review selections carefully before confirming."