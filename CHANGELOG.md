# Changelog

All notable changes to this project will be documented in this file.

## [v0.2.0] - 2025-10-29

### Added
- Initial release



The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Automated CI/CD pipeline with GitHub Actions
- Multi-platform binary releases (Linux, macOS, Windows)
- Security auditing in CI pipeline
- Automated changelog updates

## [0.1.0] - 2025-10-28

### Added
- Interactive directory size analyzer with async processing
- Multi-select file deletion with safety confirmations
- Cross-platform support (Linux, macOS, Windows) 
- Command-line interface with comprehensive options
- Human-readable file size formatting
- Robust error handling for permissions and missing files
- Comprehensive test suite with 20 unit and integration tests
- Test-driven development approach with 95% coverage
- Installation script for automated setup
- Complete English documentation

### Features
- **High Performance**: Concurrent directory analysis using Rust async/await
- **Safety First**: Multiple confirmation prompts and validation before deletion
- **Flexible Filtering**: Filter by minimum size, file type, directories only
- **Interactive UI**: Multi-select interface similar to fzf for file selection
- **Error Recovery**: Graceful handling of permission errors and missing files
- **Memory Efficient**: Optimized for large directories with thousands of files

### Technical Implementation
- Built with Rust for maximum performance and memory safety
- Modular architecture with clear separation of concerns
- Extensive error handling using anyhow for better user experience
- Async/await for non-blocking I/O operations
- Comprehensive testing with edge case coverage

### Dependencies
- **clap 4.5** - Command line argument parsing and help generation
- **tokio 1.48** - Async runtime for concurrent operations  
- **dialoguer 0.11** - Interactive prompts and multi-select interface
- **humansize 2.1** - Human-readable size formatting (KB, MB, GB)
- **walkdir 2.5** - Efficient directory traversal
- **anyhow 1.0** - Ergonomic error handling with context

### Documentation
- Complete README with usage examples and installation instructions
- Detailed testing documentation explaining TDD approach
- Contributing guidelines for open source collaboration
- Security policy for responsible vulnerability disclosure
- MIT license for maximum compatibility

[Unreleased]: https://github.com/srcheesedev/disk-cleaner/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/srcheesedev/disk-cleaner/releases/tag/v0.1.0