# Distribution Strategy

## Current Approach: Direct Downloads

We've simplified our distribution strategy to focus on direct downloads via GitHub Releases.

### Available Installers

#### macOS

- **PKG Installer**: Professional installer with GUI
- **Universal Binary**: Supports both Intel and Apple Silicon
- **Install Command**: `sudo installer -pkg disk-cleaner-macos-v0.2.0.pkg -target /`

#### Windows

- **MSI Installer**: Windows Installer package
- **Install Command**: `msiexec /i disk-cleaner-windows-v0.2.0.msi /quiet`

#### Linux

- **DEB Package**: For Ubuntu/Debian systems
  - Install: `sudo dpkg -i disk-cleaner_0.2.0_amd64.deb`
- **RPM Package**: For RHEL/Fedora systems
  - Install: `sudo rpm -i disk-cleaner-0.2.0-1.x86_64.rpm`

### Future Package Managers

These will be implemented in future releases:

- **Homebrew**: Official tap integration
- **Chocolatey**: Windows package manager
- **AUR**: Arch Linux user repository (PKGBUILD ready)

### Automation

- **GitHub Actions**: Automated builds and releases
- **Cross-platform**: All platforms built in CI/CD
- **Checksums**: SHA256 verification for all packages
- **Signing**: Code signing for Windows/macOS (TODO)

### Why This Approach

1. **Immediate Availability**: Users can download and install immediately
2. **Professional Experience**: Proper installers for each platform
3. **Simplified Maintenance**: No external repository management
4. **Future Flexibility**: Can add package managers incrementally

### Release Process

1. Tag new version
2. GitHub Actions builds all platforms
3. Creates installers for each OS
4. Publishes to GitHub Releases
5. Updates README with new version links