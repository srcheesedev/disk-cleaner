# Contributing to Disk Cleaner

Thank you for your interest in contributing to Disk Cleaner! This guide will help you get started with contributing to this high-performance directory analyzer and cleanup tool.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

Please be respectful and professional in all interactions. We aim to create a welcoming environment for all contributors, regardless of experience level, gender, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

## Getting Started

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - Version control
- **Basic CLI knowledge** - Understanding of command-line tools

### Development Setup

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/srcheesedev/disk-cleaner.git
   cd disk-cleaner
   ```

2. **Install Rust toolchain**:
   ```bash
   rustup toolchain install stable
   rustup component add rustfmt clippy
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests to ensure everything works**:
   ```bash
   cargo test
   ```

5. **Install development tools**:
   ```bash
   cargo install cargo-audit  # Security auditing
   cargo install cargo-tarpaulin  # Code coverage (Linux only)
   ```

## Development Workflow

### 1. Choose Your Contribution

- **🐛 Bug Fixes** - Check [Issues](https://github.com/YOUR_USERNAME/disk-cleaner/issues) labeled `bug`
- **✨ Features** - Look for `enhancement` or `good first issue` labels
- **📚 Documentation** - Improve README, code comments, or guides
- **🧪 Tests** - Add test coverage for existing functionality

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
# or
git checkout -b docs/documentation-improvement
```

### 3. Development Cycle

#### Write Code
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Write clear, self-documenting code
- Add appropriate error handling
- Consider performance implications

#### Test Your Changes
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

#### Manual Testing
```bash
# Build and test the binary
cargo build --release
./target/release/disk-cleaner-rs --help

# Test with different scenarios
./target/release/disk-cleaner-rs /tmp
./target/release/disk-cleaner-rs --min-size 1000
./target/release/disk-cleaner-rs --dirs-only
```

### 4. Commit Your Changes

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```bash
git add .
git commit -m "feat: add directory filtering by date modified"
git commit -m "fix: handle permission denied errors gracefully"
git commit -m "docs: improve installation instructions"
git commit -m "test: add edge cases for empty directories"
```

**Commit Types**:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring without functionality changes
- `test:` - Adding or modifying tests
- `chore:` - Maintenance tasks, dependencies

### 5. Push and Create Pull Request

```bash
git push origin your-branch-name
```

Then create a Pull Request on GitHub with:
- Clear title describing the change
- Detailed description of what was changed and why
- Reference any related issues (`Fixes #123`)
- Screenshots for UI changes (if applicable)

## Coding Standards

### Rust Guidelines

**Code Style**:
```bash
# Format your code
cargo fmt

# This should pass without warnings
cargo clippy -- -D warnings
```

**Naming Conventions**:
- `snake_case` for functions, variables, modules
- `PascalCase` for types, structs, enums
- `SCREAMING_SNAKE_CASE` for constants
- Descriptive names over short ones

**Error Handling**:
```rust
// Use Result<T> for operations that can fail
fn analyze_directory(path: &Path) -> Result<Vec<DirectoryEntry>> {
    // Implementation
}

// Use anyhow for application errors
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    std::fs::read_to_string("config.toml")
        .context("Failed to read configuration file")?;
    // ...
}
```

**Documentation**:
```rust
/// Calculates the total size of a directory recursively.
///
/// # Arguments
/// * `path` - The directory path to analyze
///
/// # Returns
/// * `Result<u64>` - Total size in bytes, or error if inaccessible
///
/// # Examples
/// ```
/// let size = calculate_size("/home/user")?;
/// println!("Directory size: {} bytes", size);
/// ```
fn calculate_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    // Implementation
}
```

### Architecture Principles

**Separation of Concerns**:
- `analyzer.rs` - Directory analysis logic
- `file_manager.rs` - User interaction and file operations
- `main.rs` - CLI interface and orchestration

**Error Propagation**:
- Use `?` operator for error propagation
- Provide context with `.context()` from anyhow
- Handle errors at the appropriate level

**Performance Considerations**:
- Use async/await for I/O operations
- Avoid unnecessary allocations
- Profile with `cargo bench` for critical paths

## Testing Guidelines

### Test Structure

We follow the **AAA pattern** (Arrange, Act, Assert):

```rust
#[test]
fn test_calculate_directory_size() {
    // Arrange - Set up test data
    let temp_dir = create_test_structure().unwrap();
    
    // Act - Execute the function under test
    let size = DiskAnalyzer::calculate_size(&temp_dir).unwrap();
    
    // Assert - Verify the outcome
    assert_eq!(size, expected_size);
}
```

### Test Categories

**Unit Tests** (in each module):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

**Integration Tests**:
```rust
// tests/integration_test.rs
use disk_cleaner::*;

#[test]
fn test_full_workflow() {
    // Test complete user workflows
}
```

### Test Requirements

- **Coverage**: Aim for >90% line coverage on critical paths
- **Edge Cases**: Test empty directories, permission errors, large files
- **Error Conditions**: Verify proper error handling
- **Performance**: No regressions in performance-critical functions

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test analyzer

# With output
cargo test -- --nocapture

# Documentation tests
cargo test --doc

# Benchmark tests
cargo bench
```

## Documentation

### Code Documentation

- **Public APIs**: Must have doc comments
- **Complex Logic**: Inline comments explaining why, not what
- **Examples**: Include usage examples in doc comments
- **Error Cases**: Document when functions can fail

### User Documentation

- **README.md**: Keep updated with new features
- **CHANGELOG.md**: Follow [Keep a Changelog](https://keepachangelog.com/) format
- **Installation Guide**: Test on different platforms
- **Usage Examples**: Real-world scenarios

### Documentation Testing

```bash
# Test code examples in documentation
cargo test --doc

# Generate and check documentation
cargo doc --no-deps --open
```

## Pull Request Process

### Before Submitting

- [ ] Tests pass: `cargo test`
- [ ] No linting issues: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for significant changes)
- [ ] No merge conflicts with main branch

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests pass locally
```

### Review Process

1. **Automated Checks**: CI pipeline must pass
2. **Code Review**: At least one maintainer review
3. **Testing**: Reviewer may test changes locally
4. **Documentation**: Verify docs are accurate and complete

### Review Criteria

- **Functionality**: Does it work as intended?
- **Performance**: Any performance regressions?
- **Security**: Are there security implications?
- **Maintainability**: Is the code readable and well-structured?
- **Testing**: Adequate test coverage?
- **Documentation**: Clear and accurate?

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH` (e.g., `1.0.0`)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Workflow

1. **Update Version**: In `Cargo.toml`
2. **Update CHANGELOG.md**: Document all changes
3. **Create PR**: For version bump
4. **Merge to Main**: After approval
5. **Create Tag**: `git tag v1.0.0`
6. **Push Tag**: `git push origin v1.0.0`
7. **GitHub Actions**: Automatically creates release with binaries

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Tag created and pushed
- [ ] Release notes written
- [ ] Binaries built for all platforms

## Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and general discussion
- **Code Review**: Comments on PRs

### Learning Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Async Programming**: https://rust-lang.github.io/async-book/
- **Testing**: https://doc.rust-lang.org/book/ch11-00-testing.html

## Recognition

Contributors are recognized in:
- **GitHub Contributors List**
- **CHANGELOG.md** (for significant contributions)
- **README.md** acknowledgments section

## License

By contributing to Disk Cleaner, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Disk Cleaner! Your efforts help make this tool better for everyone. 🚀