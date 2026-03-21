### Requirement: Print subcommand outputs timezone text to stdout
The CLI SHALL accept a `print` subcommand that writes the formatted timezone text to stdout and exits with status 0. The output SHALL be identical to the text produced by the TUI copy (`c` key) feature for the same reference time, config, and time format.

#### Scenario: Basic print with default time
- **WHEN** the user runs `woti print` with no flags
- **THEN** stdout SHALL contain one line per configured timezone using the current time as the reference, in the same format as the TUI copy output, and the process SHALL exit with status 0

#### Scenario: Print produces same output as TUI copy
- **WHEN** the user runs `woti print --date 2026-04-15 --time 14:00` and separately launches `woti --date 2026-04-15 --time 14:00` and presses `c`
- **THEN** the stdout text from `print` SHALL be identical to the clipboard text from the TUI copy

### Requirement: Print subcommand accepts --date flag
The `print` subcommand SHALL accept an optional `--date` flag with the same format and validation as the root-level `--date` flag (ISO 8601: `YYYY-MM-DD`).

#### Scenario: Valid date flag
- **WHEN** the user runs `woti print --date 2026-04-15`
- **THEN** the output SHALL use April 15, 2026 as the reference date, with the current local time as the time component

#### Scenario: Invalid date flag
- **WHEN** the user runs `woti print --date 04-15-2026`
- **THEN** the process SHALL exit with a non-zero status and print an error indicating the expected format

### Requirement: Print subcommand accepts --time flag
The `print` subcommand SHALL accept an optional `--time` flag with the same format and validation as the root-level `--time` flag (ISO 8601: `HH:MM` or `HH:MM:SS`, 24-hour).

#### Scenario: Valid time flag
- **WHEN** the user runs `woti print --time 15:30`
- **THEN** the output SHALL use today's date at 15:30 local time as the reference

#### Scenario: Invalid time flag
- **WHEN** the user runs `woti print --time 3:30pm`
- **THEN** the process SHALL exit with a non-zero status and print an error indicating the expected format

### Requirement: Print subcommand combines --date and --time
When both flags are provided, the `print` subcommand SHALL combine them into a single reference datetime interpreted in the user's local timezone and converted to UTC, matching the root-level flag behavior.

#### Scenario: Both flags provided
- **WHEN** the user runs `woti print --date 2026-04-15 --time 14:00`
- **THEN** the output SHALL use 2026-04-15 14:00 local time as the reference datetime

### Requirement: Print uses configured time format
The `print` subcommand SHALL use the time format from the user's config file (defaulting to `Mixed` if unset), matching the TUI default.

#### Scenario: Config has 24h format
- **WHEN** the user's config specifies `time_format = "24h"` and runs `woti print`
- **THEN** times SHALL be formatted in 24-hour style (e.g., `14:00`)

#### Scenario: Config has 12h format
- **WHEN** the user's config specifies `time_format = "12h"` and runs `woti print`
- **THEN** times SHALL be formatted in 12-hour style (e.g., `2pm`)

### Requirement: Print does not launch TUI
The `print` subcommand SHALL NOT enable raw mode, switch to the alternate screen, or initialize any TUI components.

#### Scenario: No terminal manipulation
- **WHEN** the user runs `woti print`
- **THEN** the process SHALL write to stdout and exit without altering terminal state

### Requirement: Print output is newline-terminated
The `print` subcommand output SHALL end with a trailing newline, following Unix convention for text output.

#### Scenario: Trailing newline
- **WHEN** the user runs `woti print` and captures stdout
- **THEN** the output SHALL end with a newline character
