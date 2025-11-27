# Deployment Guide

## Overview

This project supports automated multi-platform deployment through GitHub Actions.

## Supported Platforms

- **Linux**: x86_64 (glibc)
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

## Deployment Process

### 1. CI/CD Workflows

#### CI Workflow (`.github/workflows/ci.yml`)
- **Trigger**: Push/PR to main
- **Actions**:
  - Run tests on all platforms
  - Clippy linting
  - Code formatting check
  - Build artifacts

#### Release Workflow (`.github/workflows/release.yml`)
- **Trigger**: Version tag push (`v*.*.*`)
- **Actions**:
  - Build binaries for all platforms
  - Create GitHub Release
  - Upload binaries and checksums
  - Publish to crates.io automatically

### 2. Creating a Release

```bash
# 1. Create version tag
git tag v0.1.0

# 2. Push tag (triggers release process automatically)
git push origin v0.1.0
```

GitHub Actions will automatically:
1. Build binaries for all platforms
2. Create GitHub Release
3. Upload binaries and checksums
4. Publish to crates.io (if configured)

## Configuration Requirements

### GitHub Secrets (for crates.io publishing)

Configure in Repository Settings → Secrets and variables → Actions:

- `CARGO_REGISTRY_TOKEN`: crates.io API token
- Generate at https://crates.io/settings/tokens

### Version Management

Keep `Cargo.toml` version and git tag in sync:

```toml
[package]
version = "0.1.0"  # This version
```

```bash
git tag v0.1.0     # Should match this tag
```

## Release Checklist

- [ ] Update `Cargo.toml` version
- [ ] Update `cli/args.rs` version constant
- [ ] Update `CHANGELOG.md`
- [ ] Run `cargo test` and ensure all tests pass
- [ ] Run `cargo clippy -- -D warnings` and fix issues
- [ ] Run `cargo fmt -- --check` and format code
- [ ] Commit and push to `main` branch
- [ ] Create and push version tag
- [ ] Verify GitHub Actions workflow
- [ ] Write/edit release notes
- [ ] Test installation on each platform

## Troubleshooting

### crates.io Publishing Failure
- Verify `CARGO_REGISTRY_TOKEN` is set correctly
- Check `Cargo.toml` metadata (description, license, etc.)

### Cross-Compilation Failure
- Check platform-specific dependencies
- Review GitHub Actions runner logs

### Installation Script Errors
- Verify GitHub Release was created
- Confirm binary name matches (`doum` vs `doum.exe`)

## Future Distribution Channels

### Windows
- [ ] winget (Windows Package Manager)

### macOS
- [ ] Homebrew

### Linux
- [ ] APT (Debian/Ubuntu)
- [ ] YUM/DNF (RHEL/Fedora)