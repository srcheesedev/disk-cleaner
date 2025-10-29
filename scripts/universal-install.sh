#!/bin/bash
set -e

# Universal installer script for Disk Cleaner
# Supports Linux, macOS, and Windows (via WSL/Git Bash)

VERSION="0.2.0"
REPO="srcheesedev/disk-cleaner"
INSTALL_DIR="/usr/local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Main installation flow
main() {
    echo "🧹 Disk Cleaner Universal Installer"
    echo "======================================"
    
    log_info "For the complete installation experience, please visit:"
    log_info "🌐 https://github.com/${REPO}/releases/latest"
    
    echo ""
    log_info "Available installation options:"
    echo ""
    echo "� macOS:"
    echo "  • PKG Installer: Download disk-cleaner-macos-v0.2.0.pkg from releases"
    echo "  • Homebrew: Coming soon to official tap"
    echo ""
    echo "🐧 Linux:"
    echo "  • DEB Package: Download disk-cleaner_0.2.0_amd64.deb from releases"
    echo "  • RPM Package: Download disk-cleaner-0.2.0-1.x86_64.rpm from releases"
    echo "  • AUR: Coming soon (yay -S disk-cleaner)"
    echo ""
    echo "🪟 Windows:"
    echo "  • MSI Installer: Download disk-cleaner-windows-v0.2.0.msi from releases"
    echo "  • Chocolatey: Coming soon (choco install disk-cleaner)"
    echo ""
    echo "📦 Manual binaries also available for all platforms"
    
    log_success "🎉 Choose your preferred method above!"
}

# Run installer
main "$@"