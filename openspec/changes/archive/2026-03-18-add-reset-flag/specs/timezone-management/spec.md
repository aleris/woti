## ADDED Requirements

### Requirement: Reset to defaults
The system SHALL support a `--reset` flag on the `remove` subcommand that removes all user-added timezones and restores the configuration to its default state (Local + UTC only).

#### Scenario: Reset with custom timezones configured
- **GIVEN** user has PST, EET, and JST added alongside defaults
- **WHEN** user runs `woti remove --reset`
- **THEN** the system removes PST, EET, and JST from config, keeps Local and UTC, saves the config, and prints a confirmation message indicating how many timezones were removed

#### Scenario: Reset when only defaults exist
- **GIVEN** user has no custom timezones (only Local and UTC)
- **WHEN** user runs `woti remove --reset`
- **THEN** the system prints a message indicating there are no custom timezones to remove

#### Scenario: Reset flag conflicts with zone argument
- **WHEN** user runs `woti remove --reset PST`
- **THEN** the system prints a clap error indicating that `--reset` and a zone argument cannot be used together

#### Scenario: Remove with no arguments
- **WHEN** user runs `woti remove` (no zone, no `--reset`)
- **THEN** the system prints a clap error indicating that a zone or `--reset` is required

#### Scenario: Reset help
- **WHEN** user runs `woti remove --help`
- **THEN** the help output includes the `--reset` flag with a description
