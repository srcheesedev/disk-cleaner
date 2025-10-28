---
name: Pull Request
about: Submit a contribution to the project
title: ''
labels: ''
assignees: ''
---

## Description

Brief description of changes made in this PR.

## Type of Change

Please delete options that are not relevant.

- [ ] 🐛 **Bug fix** (`fix:`) - non-breaking change which fixes an issue → **PATCH** version
- [ ] ✨ **New feature** (`feat:`) - non-breaking change which adds functionality → **MINOR** version  
- [ ] 💥 **Breaking change** (`feat!:` or `fix!:`) - fix or feature that would cause existing functionality to not work as expected → **MAJOR** version
- [ ] 📝 **Documentation update** (`docs:`) - changes to documentation only
- [ ] ⚡ **Performance improvement** (`perf:`) - code change that improves performance → **PATCH** version
- [ ] ♻️ **Code refactoring** (`refactor:`) - code change that neither fixes a bug nor adds a feature → **PATCH** version
- [ ] ✅ **Test improvements** (`test:`) - adding missing tests or correcting existing tests

## Related Issues

Fixes #(issue number)
Closes #(issue number)
Related to #(issue number)

## Changes Made

- List specific changes made
- Include any new dependencies added
- Mention any breaking changes

## Testing

Describe the tests that you ran to verify your changes.

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] All existing tests pass

### Test Results

```bash
cargo test
# Paste relevant test output here
```

## Performance Impact

Describe any performance implications of your changes.

- [ ] No performance impact
- [ ] Performance improvement (describe)
- [ ] Performance regression (justify and describe mitigation)

## Screenshots

If applicable, add screenshots to help explain your changes.

## Checklist

### Code Quality
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation

### Testing
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

### Documentation
- [ ] I have updated the README.md if needed
- [ ] I have updated the CHANGELOG.md if needed
- [ ] I have updated relevant documentation

### Release Readiness
- [ ] My changes generate no new warnings
- [ ] I have checked my code compiles on multiple platforms (if applicable)
- [ ] I have updated version numbers where appropriate

## Additional Notes

Add any additional notes for reviewers here.

---

## 📖 Conventional Commits & Auto-Release

This project uses **Conventional Commits** for automatic versioning and releases:

- `feat:` → **MINOR** version (new functionality)
- `fix:` → **PATCH** version (bug fix)
- `feat!:` or `fix!:` → **MAJOR** version (breaking change)
- `docs:`, `style:`, `test:`, `chore:` → no version bump

**Examples:**

- `feat: add multi-select file deletion capability`
- `fix: resolve memory leak in directory scanning`
- `feat!: change CLI argument structure (breaking change)`

When this PR is merged to `main`, it will automatically:

1. 🏷️ Create a new version tag based on commit types
2. 📝 Update CHANGELOG.md with PR details
3. 🚀 Create a GitHub release with compiled binaries
4. 🔄 Sync the `develop` branch
