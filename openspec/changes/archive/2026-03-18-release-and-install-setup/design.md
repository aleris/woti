## Context

`scripts/release.sh` currently builds a single-platform binary on the developer's machine, packages it as a tarball, tags the commit, and publishes a GitHub release with that one asset. There is no CI pipeline, no cross-compilation, and no user-facing install mechanism beyond "clone and build."

The project is a Rust binary (`woti`) with no C dependencies beyond libc, making cross-compilation straightforward. The existing tarball naming convention is `woti-v0.3.0-darwin-arm64.tar.gz`.

## Goals / Non-Goals

**Goals:**
- Automate multi-platform release builds via GitHub Actions on tag push
- Support four targets: linux x86_64, linux aarch64, macOS x86_64, macOS aarch64
- Provide a `curl | sh` installer for end users
- Simplify the local release script to tag-and-push only

**Non-Goals:**
- Windows support
- Package manager distribution (Homebrew, apt, etc.)
- Automatic version bumping or changelog generation
- Signed binaries or notarization

## Decisions

### 1. CI build strategy: container-based Rust environments

Use pre-built Docker images with Rust toolchains already installed rather than installing Rust as a workflow step.

- **Linux x86_64**: `container: rust:latest` on `ubuntu-latest` — official Rust image, zero setup.
- **Linux aarch64**: `container: ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest` on `ubuntu-latest` — cross-rs image with the correct linker and Rust target pre-configured. Requires `rustup target add aarch64-unknown-linux-gnu` inside the container.
- **macOS x86_64 / aarch64**: `macos-latest` runner natively (no container). macOS runners include Rust via Homebrew; use `rustup target add` for the non-native architecture.

**Why not `cross`?** `cross` wraps Docker and adds complexity. Since we only need four targets and GitHub provides both Linux and macOS runners, direct `cargo build --target` is simpler and more transparent.

### 2. Workflow structure: build matrix + final publish job

A matrix job builds each target in parallel. A subsequent `publish` job (using `needs:`) downloads all artifacts and creates the GitHub release in one shot.

**Why not per-job releases?** A single publish job avoids race conditions and produces a clean release with all assets attached atomically.

### 3. Asset naming: `woti-{tag}-{os}-{arch}.tar.gz`

Standardize on lowercase `{os}` and architecture from `uname -m` conventions:

| Target                      | os     | arch    |
| --------------------------- | ------ | ------- |
| x86_64-apple-darwin         | darwin | x86_64  |
| aarch64-apple-darwin        | darwin | aarch64 |
| x86_64-unknown-linux-gnu    | linux  | x86_64  |
| aarch64-unknown-linux-gnu   | linux  | aarch64 |

### 4. Installer target directory: `~/.local/bin` (user) or `/usr/local/bin` (root)

Follows XDG conventions and avoids requiring `sudo` for normal installs. The script checks `$EUID` to decide.

### 5. Release script simplification

Strip `cargo build`, tarball creation, and `gh release create` from `scripts/release.sh`. Keep the pre-flight checks (branch, clean tree, tag uniqueness) and the confirmation prompt. Tag + push is all it does — CI handles the rest.

## Risks / Trade-offs

- **cross-rs image stability** → Pin to a specific image digest if builds become flaky. For now, `:latest` is acceptable for a low-traffic project.
- **macOS runner Rust availability** → GitHub-hosted macOS runners currently ship with Rust. If this changes, add a `rustup` install step. The workflow should fail loudly if `cargo` is missing.
- **Installer assumes GitHub releases URL format** → If the repo moves or goes private, the installer breaks. Acceptable for the current project scope.
- **No checksum verification in installer** → A future enhancement. For now, HTTPS from GitHub provides transport-level integrity.
