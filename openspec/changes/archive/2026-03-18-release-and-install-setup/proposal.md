## Why

The current release process (`scripts/release.sh`) builds only for the host machine, producing a single-platform tarball. Users on other OS/arch combinations can't install woti from a release. There's also no way for users to install woti without cloning the repo and building from source. Moving builds to CI and adding a shell installer makes woti distributable to all supported platforms with a one-liner.

## What Changes

- Add a GitHub Actions workflow (`.github/workflows/release.yml`) that triggers on `v*` tags and builds release binaries for linux x86_64, linux aarch64, macOS x86_64, and macOS aarch64, then publishes them as GitHub release assets.
- Add a `curl | sh` installer script (`scripts/install.sh`) that detects the user's OS and architecture, downloads the correct binary from the latest GitHub release, and installs it.
- Simplify `scripts/release.sh` to only handle tagging and pushing — CI takes over building and publishing.
- Minor Makefile review (no functional changes expected).

## Capabilities

### New Capabilities
- `ci-release`: GitHub Actions workflow for multi-platform release builds triggered by version tags.
- `shell-installer`: Standalone shell script for one-command installation from GitHub releases.

### Modified Capabilities

_None — no existing spec-level requirements are changing._

## Impact

- **New files**: `.github/workflows/release.yml`, `scripts/install.sh`
- **Modified files**: `scripts/release.sh`, potentially `Makefile`
- **Dependencies**: GitHub Actions runners (ubuntu-latest, macos-latest), `gh` CLI (used in workflow), Rust Docker images for cross-compilation
- **User-facing**: Users gain a `curl | sh` install path; release assets now cover 4 platform targets instead of 1
