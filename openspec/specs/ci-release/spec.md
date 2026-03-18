### Requirement: Multi-platform release build on tag push
The CI workflow SHALL trigger when a tag matching `v*` is pushed to the repository. It SHALL build release binaries for all four supported targets: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, and `aarch64-apple-darwin`.

#### Scenario: Tag push triggers workflow
- **WHEN** a tag matching `v*` is pushed to the repository
- **THEN** the release workflow starts and builds binaries for all four targets in parallel

#### Scenario: Non-tag push does not trigger workflow
- **WHEN** a regular commit is pushed (no tag)
- **THEN** the release workflow does not run

### Requirement: Each build job produces a tarball artifact
Each matrix job SHALL compile the binary with `cargo build --release --target <target>` and package it into a tarball named `woti-{tag}-{os}-{arch}.tar.gz` following the naming convention in the design. The tarball SHALL be uploaded as a GitHub Actions artifact.

#### Scenario: Successful build and packaging
- **WHEN** a build job completes for target `aarch64-apple-darwin` with tag `v0.4.0`
- **THEN** a tarball named `woti-v0.4.0-darwin-aarch64.tar.gz` is uploaded as an artifact containing the `woti` binary

#### Scenario: Build failure stops the job
- **WHEN** `cargo build --release` fails for any target
- **THEN** that matrix job fails and no tarball artifact is uploaded for that target

### Requirement: Publish job creates GitHub release with all assets
A publish job SHALL run after all build jobs complete successfully. It SHALL download all tarball artifacts and create a GitHub release for the tag, attaching all tarballs and generating release notes.

#### Scenario: All builds succeed
- **WHEN** all four build matrix jobs complete successfully
- **THEN** a GitHub release is created for the tag with all four tarballs attached and auto-generated release notes

#### Scenario: Any build fails
- **WHEN** one or more build matrix jobs fail
- **THEN** the publish job does not run and no GitHub release is created

### Requirement: Release script only tags and pushes
The simplified `scripts/release.sh` SHALL perform pre-flight checks (correct branch, clean working tree, tag does not exist), prompt for confirmation, create an annotated tag, and push it. It SHALL NOT build binaries, create tarballs, or publish GitHub releases.

#### Scenario: Successful tag and push
- **WHEN** a developer runs `scripts/release.sh` on the main branch with a clean tree and a new version
- **THEN** the script creates an annotated tag and pushes it to origin, then exits

#### Scenario: Pre-flight check failure
- **WHEN** the developer is not on the main branch, has uncommitted changes, or the tag already exists
- **THEN** the script prints an error and exits with a non-zero status without creating or pushing a tag
