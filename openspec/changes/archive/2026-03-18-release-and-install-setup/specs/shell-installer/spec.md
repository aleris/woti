## ADDED Requirements

### Requirement: Detect OS and architecture
The installer script SHALL detect the operating system via `uname -s` and the CPU architecture via `uname -m`, and map them to the tarball naming convention (`darwin`/`linux` for OS, `x86_64`/`aarch64` for arch).

#### Scenario: macOS ARM detection
- **WHEN** the script runs on a macOS system with Apple Silicon (`uname -s` = "Darwin", `uname -m` = "arm64")
- **THEN** the script maps to os=`darwin` and arch=`aarch64`

#### Scenario: Linux x86_64 detection
- **WHEN** the script runs on a Linux x86_64 system (`uname -s` = "Linux", `uname -m` = "x86_64")
- **THEN** the script maps to os=`linux` and arch=`x86_64`

#### Scenario: Unsupported platform
- **WHEN** the script runs on an unsupported OS or architecture (e.g., Windows/MSYS, or an unrecognized arch)
- **THEN** the script prints an error message listing supported platforms and exits with a non-zero status

### Requirement: Fetch latest release version
The installer script SHALL query the GitHub API to determine the latest release tag, unless the user provides a specific version via a `--version` flag.

#### Scenario: Auto-detect latest version
- **WHEN** the script is invoked without a `--version` flag
- **THEN** the script fetches the latest release tag from the GitHub API and uses it for the download URL

#### Scenario: User-specified version
- **WHEN** the script is invoked with `--version v0.4.0`
- **THEN** the script uses `v0.4.0` as the release tag for the download URL

### Requirement: Download and install binary
The installer script SHALL download the correct tarball from GitHub releases, extract the `woti` binary, and place it in the appropriate directory.

#### Scenario: User-mode install
- **WHEN** the script runs as a non-root user
- **THEN** the binary is installed to `~/.local/bin/`, creating the directory if it does not exist

#### Scenario: Root-mode install
- **WHEN** the script runs as root
- **THEN** the binary is installed to `/usr/local/bin/`

#### Scenario: Download failure
- **WHEN** the tarball download fails (e.g., 404 for a nonexistent version or platform)
- **THEN** the script prints an error with the attempted URL and exits with a non-zero status

### Requirement: Post-install guidance
The installer script SHALL print a success message after installation. If the install directory is not in the user's `$PATH`, it SHALL print instructions for adding it.

#### Scenario: Directory already in PATH
- **WHEN** the install completes and the install directory is in `$PATH`
- **THEN** the script prints a success message confirming the binary location

#### Scenario: Directory not in PATH
- **WHEN** the install completes and the install directory is not in `$PATH`
- **THEN** the script prints a success message and instructions to add the directory to `$PATH`
