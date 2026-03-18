## 1. CI Release Workflow

- [x] 1.1 Create `.github/workflows/release.yml` with tag-trigger (`on: push: tags: ["v*"]`) and a build matrix covering the four targets (linux x86_64, linux aarch64, macOS x86_64, macOS aarch64)
- [x] 1.2 Configure each matrix entry with the correct runner and container (rust:latest for linux x86_64, cross-rs image for linux aarch64, macos-latest for both macOS targets)
- [x] 1.3 Add build steps: checkout, `rustup target add` (where needed), `cargo build --release --target <target>`, package into `woti-{tag}-{os}-{arch}.tar.gz`, and upload as artifact
- [x] 1.4 Add publish job that depends on all build jobs: download all artifacts and run `gh release create` with all tarballs attached and `--generate-notes`

## 2. Shell Installer Script

- [x] 2.1 Create `scripts/install.sh` with OS detection (`uname -s`) and arch detection (`uname -m`), mapping to the tarball naming convention
- [x] 2.2 Add latest-version fetching from the GitHub API (`/repos/{owner}/{repo}/releases/latest`) with `--version` flag override
- [x] 2.3 Add tarball download, extraction, and binary placement to `~/.local/bin` (non-root) or `/usr/local/bin` (root)
- [x] 2.4 Add post-install message with PATH instructions when the install directory is not in `$PATH`
- [x] 2.5 Handle error cases: unsupported platform, download failure, missing `curl`

## 3. Simplify Release Script

- [x] 3.1 Remove `cargo build`, tarball creation, and `gh release create` from `scripts/release.sh`
- [x] 3.2 Keep pre-flight checks (branch, clean tree, tag uniqueness) and confirmation prompt
- [x] 3.3 Keep tag creation (`git tag -a`) and push (`git push origin`), verify CI takes over messaging

## 4. Makefile Review

- [x] 4.1 Verify `make release` still works with the simplified script; update target if needed
