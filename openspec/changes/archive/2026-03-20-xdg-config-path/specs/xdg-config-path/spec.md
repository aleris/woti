## ADDED Requirements

### Requirement: Config file resolved from XDG path first

The system SHALL look for the config file at `~/.config/woti/config.toml` before checking any platform-native path.

#### Scenario: XDG config file exists

- **WHEN** `~/.config/woti/config.toml` exists on disk
- **THEN** the system SHALL load configuration from that file

#### Scenario: XDG config file does not exist but legacy path does

- **WHEN** `~/.config/woti/config.toml` does not exist
- **AND** the platform-native legacy config file exists (e.g. `~/Library/Application Support/woti/config.toml` on macOS)
- **THEN** the system SHALL load configuration from the legacy path

#### Scenario: No config file exists anywhere

- **WHEN** neither the XDG path nor the legacy path contains a config file
- **THEN** the system SHALL use the built-in default configuration

### Requirement: New config files created at XDG path

The system SHALL always write new or updated config files to `~/.config/woti/config.toml`, creating the `~/.config/woti/` directory if it does not exist.

#### Scenario: Saving config for the first time

- **WHEN** no config file exists and the user triggers a config save
- **THEN** the system SHALL create `~/.config/woti/config.toml` with the current configuration

#### Scenario: Saving config when loaded from legacy path

- **WHEN** the config was loaded from the legacy platform-native path
- **AND** the user triggers a config save
- **THEN** the system SHALL write the updated config to `~/.config/woti/config.toml` (not the legacy path)

### Requirement: Home directory unavailable gracefully handled

The system SHALL gracefully handle the case where the user's home directory cannot be determined.

#### Scenario: Home directory is unresolvable

- **WHEN** the system cannot determine the user's home directory
- **THEN** the system SHALL fall back to the built-in default configuration for loading
- **AND** SHALL return an error when attempting to save

### Requirement: directories crate removed

The `directories` crate dependency SHALL be replaced by the `dirs` crate, using only `dirs::home_dir()` for home directory resolution.

#### Scenario: Dependency footprint

- **WHEN** the project is compiled
- **THEN** `directories` SHALL NOT appear in `Cargo.toml` dependencies
- **AND** `dirs` SHALL appear as a dependency
