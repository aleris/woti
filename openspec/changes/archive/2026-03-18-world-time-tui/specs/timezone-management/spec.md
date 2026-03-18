## ADDED Requirements

### Requirement: Detailed help and version
The system SHALL display detailed help text when invoked with `--help` or `-h`, and version information when invoked with `--version` or `-V`. The help output SHALL include the program description, all available subcommands with descriptions, usage examples, and accepted input formats.

#### Scenario: Show help
- **WHEN** user runs `woti --help`
- **THEN** the system prints the program name, version, description, available subcommands (add, remove), their arguments, and usage examples showing timezone code, city name, and IANA identifier inputs

#### Scenario: Show short help
- **WHEN** user runs `woti -h`
- **THEN** the system prints the same detailed help output as `--help`

#### Scenario: Show version
- **WHEN** user runs `woti --version`
- **THEN** the system prints the program name and version number (e.g., "woti 0.1.0")

#### Scenario: Subcommand help
- **WHEN** user runs `woti add --help`
- **THEN** the system prints help specific to the add subcommand, including accepted input formats (timezone abbreviation, city name, IANA identifier) with examples

### Requirement: Add timezone by abbreviation
The system SHALL accept timezone abbreviations (e.g., PST, EET, CET) via `woti add <abbreviation>` and resolve them to an IANA timezone identifier for storage.

#### Scenario: Add a common abbreviation
- **WHEN** user runs `woti add PST`
- **THEN** the system resolves PST to America/Los_Angeles, persists it to config, and confirms with the display name and zone

#### Scenario: Add an ambiguous abbreviation
- **WHEN** user runs `woti add CST`
- **THEN** the system resolves to the most common interpretation (America/Chicago) and confirms the resolved zone to the user

#### Scenario: Add an unrecognized abbreviation
- **WHEN** user runs `woti add XYZ`
- **THEN** the system prints an error message indicating the abbreviation is not recognized

### Requirement: Add timezone by city name
The system SHALL accept city names via `woti add <city>` and resolve them to an IANA timezone identifier using a built-in city lookup table.

#### Scenario: Add by city name
- **WHEN** user runs `woti add Bucharest`
- **THEN** the system resolves Bucharest to Europe/Bucharest, persists it, and confirms with city, region, and zone

#### Scenario: Add by multi-word city name
- **WHEN** user runs `woti add "San Jose"`
- **THEN** the system resolves to America/Los_Angeles (San Jose, California) and confirms

#### Scenario: Add unknown city
- **WHEN** user runs `woti add Atlantis`
- **THEN** the system prints an error message indicating the city was not found

### Requirement: Add timezone by IANA identifier
The system SHALL accept full IANA timezone identifiers (e.g., America/New_York) via `woti add <iana_id>`.

#### Scenario: Add valid IANA zone
- **WHEN** user runs `woti add America/New_York`
- **THEN** the system persists the zone and confirms with the resolved display name and region

#### Scenario: Add invalid IANA zone
- **WHEN** user runs `woti add Invalid/Nowhere`
- **THEN** the system prints an error message indicating the zone is invalid

### Requirement: Remove timezone
The system SHALL allow removing a previously added timezone via `woti remove <identifier>` where identifier can be the same format used to add it (abbreviation, city, or IANA name).

#### Scenario: Remove an added timezone
- **WHEN** user has PST added and runs `woti remove PST`
- **THEN** the system removes the corresponding entry from config and confirms removal

#### Scenario: Remove a timezone not in config
- **WHEN** user runs `woti remove PST` and PST is not configured
- **THEN** the system prints an error indicating the timezone is not in the list

#### Scenario: Attempt to remove Local or UTC
- **WHEN** user runs `woti remove UTC`
- **THEN** the system prints an error indicating that default timezones cannot be removed

### Requirement: Default timezones
The system SHALL include the user's local timezone and UTC as preconfigured entries that are always present.

#### Scenario: Fresh install shows defaults
- **WHEN** user runs `woti` for the first time with no config file
- **THEN** the TUI displays Local and UTC timezone rows

#### Scenario: Defaults persist after adding custom zones
- **WHEN** user adds PST and then runs `woti`
- **THEN** the TUI displays Local, UTC, and PST timezone rows

### Requirement: Persistent configuration
The system SHALL store timezone configuration in a TOML file at the platform-standard config directory (e.g., `~/.config/woti/config.toml`).

#### Scenario: Config created on first add
- **WHEN** user runs `woti add EET` and no config file exists
- **THEN** the system creates the config file with default zones plus EET

#### Scenario: Config survives restart
- **WHEN** user adds PST, exits, and runs `woti` again
- **THEN** the TUI shows PST in the timezone list

### Requirement: Duplicate prevention
The system SHALL NOT add a timezone that is already in the configuration (by resolved IANA identifier).

#### Scenario: Add duplicate zone
- **WHEN** user has America/Los_Angeles configured and runs `woti add PST`
- **THEN** the system informs the user that this timezone is already configured
