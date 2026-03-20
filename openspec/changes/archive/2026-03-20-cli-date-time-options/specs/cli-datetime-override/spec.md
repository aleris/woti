## ADDED Requirements

### Requirement: Date flag accepts ISO 8601 date
The CLI SHALL accept an optional `--date` flag whose value is an ISO 8601 date string in `YYYY-MM-DD` format. The flag SHALL only be accepted when no subcommand is given. Invalid formats SHALL cause the process to exit with a non-zero status and an error message.

#### Scenario: Valid date flag
- **WHEN** the user runs `woti --date 2026-04-15`
- **THEN** the TUI launches with the reference date set to April 15, 2026

#### Scenario: Invalid date flag
- **WHEN** the user runs `woti --date 04-15-2026`
- **THEN** the process exits with a non-zero status and prints an error indicating the expected format

#### Scenario: Date flag rejected with subcommand
- **WHEN** the user runs `woti add PST --date 2026-04-15`
- **THEN** the CLI rejects the invocation with an error (clap conflict)

### Requirement: Time flag accepts ISO 8601 time
The CLI SHALL accept an optional `--time` flag whose value is an ISO 8601 local time string in `HH:MM` or `HH:MM:SS` format (24-hour). The flag SHALL only be accepted when no subcommand is given. Invalid formats SHALL cause the process to exit with a non-zero status and an error message.

#### Scenario: Valid time flag with minutes
- **WHEN** the user runs `woti --time 15:30`
- **THEN** the TUI launches with the reference time set to 15:30 local time on today's date

#### Scenario: Valid time flag with seconds
- **WHEN** the user runs `woti --time 09:00:00`
- **THEN** the TUI launches with the reference time set to 09:00:00 local time on today's date

#### Scenario: Invalid time flag
- **WHEN** the user runs `woti --time 3:30pm`
- **THEN** the process exits with a non-zero status and prints an error indicating the expected format

### Requirement: Date and time flags combine
When both `--date` and `--time` are provided, the system SHALL combine them into a single reference datetime interpreted in the user's local timezone and converted to UTC.

#### Scenario: Both flags provided
- **WHEN** the user runs `woti --date 2026-04-15 --time 14:00`
- **THEN** the TUI launches anchored to 2026-04-15 14:00 in the user's local timezone

### Requirement: Default to current date and time
When neither `--date` nor `--time` is provided, the TUI SHALL behave identically to the current live-clock mode (`Utc::now()` on each render).

#### Scenario: No flags
- **WHEN** the user runs `woti`
- **THEN** the TUI displays the current live time, updating on each render cycle

### Requirement: Date-only defaults time to current time
When `--date` is provided without `--time`, the system SHALL default the time component to the current local time of day.

#### Scenario: Date only
- **WHEN** the user runs `woti --date 2026-04-15` at 14:30 local time
- **THEN** the TUI launches anchored to 2026-04-15 14:30 local time

### Requirement: Time-only defaults date to today
When `--time` is provided without `--date`, the system SHALL default the date component to the current local date.

#### Scenario: Time only
- **WHEN** the user runs `woti --time 09:00`
- **THEN** the TUI launches anchored to today's date at 09:00 local time

### Requirement: Anchor time is stable across renders
When an anchor time is set, the TUI SHALL use the same reference datetime on every render cycle. The display SHALL NOT advance with the wall clock.

#### Scenario: Frozen display
- **WHEN** the user runs `woti --time 14:00` and waits 60 seconds
- **THEN** the displayed time still reads 14:00 (plus whatever `hour_offset` the user has navigated to)

### Requirement: Hour offset works relative to anchor
The Left/Right key navigation (`hour_offset`) SHALL shift relative to the anchor time, just as it currently shifts relative to `Utc::now()`.

#### Scenario: Navigate from anchor
- **WHEN** the user runs `woti --time 10:00` and presses Right once
- **THEN** the displayed time for each timezone reflects 11:00 local anchor (i.e. anchor + 1 hour)
