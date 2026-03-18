## ADDED Requirements

### Requirement: TUI launch on bare invocation
The system SHALL launch an interactive TUI when invoked with no subcommand (`woti`).

#### Scenario: Launch TUI
- **WHEN** user runs `woti` with configured timezones
- **THEN** the system enters a full-screen terminal UI showing all configured timezone rows

### Requirement: Header display
The TUI SHALL display a header row with the title "woti" left-aligned and the current local date, time, and timezone abbreviation right-aligned. The header SHALL update live when the time changes.

#### Scenario: Header shows live clock
- **WHEN** the TUI is running and the minute changes
- **THEN** the header right side updates to reflect the new time

### Requirement: Timezone info row
Each configured timezone SHALL be displayed as a block with two lines of info:
- Line 1: `$CITY / $ZONE` (left) and `$TIME` (right), separated by 3 spaces minimum
- Line 2: `$REGION` (left) and `$DATE` (right), formatted as short weekday + month + day (e.g., "Wed, Mar 18")

#### Scenario: Display timezone info
- **WHEN** timezone America/Los_Angeles is configured with city "San Jose"
- **THEN** the TUI shows "San Jose / PDT" on line 1 with current time, and "United States, California" on line 2 with the current date

#### Scenario: Time format matches convention
- **WHEN** showing time for a timezone
- **THEN** time is displayed in the locale-appropriate format (e.g., 18:43 for 24h or 6:43p for 12h)

### Requirement: Hour timeline strip
Each timezone block SHALL include a 24-segment hour timeline strip below the info rows. The strip has 3 rows:
- Row 1: Day markers shown only at hours where the day changes, displaying short day name and numeric date (e.g., "WED 18")
- Row 2: Two-digit hour numbers (0-23)
- Row 3: AM/PM indicators when using 12h format (dimmed style), or empty for 24h format

#### Scenario: Timeline shows 24 hours
- **WHEN** the TUI renders a timezone strip
- **THEN** 24 hour segments are displayed starting from a base hour

#### Scenario: Day change marker
- **WHEN** the hour timeline crosses midnight
- **THEN** row 1 shows the new day name and date above the midnight hour

#### Scenario: AM/PM row for 12h format
- **WHEN** the timezone is displayed in 12h format
- **THEN** row 3 shows "am" or "pm" in a dimmed style below each hour

### Requirement: Current hour highlighting
The column corresponding to the current hour SHALL be visually highlighted across all rows of the timeline strip.

#### Scenario: Current hour stands out
- **WHEN** the current hour in a timezone is 13
- **THEN** the entire column for hour 13 (day marker, hour number, am/pm) is rendered with a highlight style

### Requirement: Color scheme
The TUI SHALL use terminal colors to enhance readability:
- Header and footer backgrounds are styled distinctly
- Current hour column uses inverted or bold highlight
- AM/PM indicators are dimmed
- Day change markers use a distinct color
- Timezone city/zone names are styled for emphasis

#### Scenario: Colors render in 256-color terminal
- **WHEN** the TUI runs in a terminal supporting 256 colors
- **THEN** all color styles are applied correctly

### Requirement: Footer with keyboard shortcuts
The TUI SHALL display a footer row showing available keyboard shortcuts with styled key symbols and action text: `← Previous Hour`, `→ Next Hour`, `q Exit`.

#### Scenario: Footer displays shortcuts
- **WHEN** the TUI is running
- **THEN** the footer shows key hints for left arrow, right arrow, and q/x to exit

### Requirement: Exit the TUI
The system SHALL exit the TUI and restore the terminal when the user presses `q` or `x`.

#### Scenario: Quit with q
- **WHEN** user presses `q`
- **THEN** the TUI exits cleanly and the terminal is restored

#### Scenario: Quit with x
- **WHEN** user presses `x`
- **THEN** the TUI exits cleanly and the terminal is restored
