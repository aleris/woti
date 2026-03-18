### Requirement: Copy selected hour to clipboard
The system SHALL copy the currently highlighted hour column for all configured timezones to the system clipboard when the user presses the `c` key.

#### Scenario: Basic copy with all timezones on the same day
- **WHEN** the user presses `c` with the selected hour column highlighting 4pm across timezones that all fall on the same calendar day
- **THEN** the clipboard SHALL contain one line per timezone in format `City / Abbreviation Hour[am/pm]` (e.g., `Bucharest / EET 4pm`)

#### Scenario: Copy when a timezone falls on a different day
- **WHEN** the user presses `c` and one or more timezones' selected hour falls on a different calendar day than the first timezone
- **THEN** those lines SHALL append the abbreviated day name (uppercase 3-letter) and day-of-month number (e.g., `San Jose / PDT 8pm WED 19`)

#### Scenario: First timezone never shows day suffix
- **WHEN** the user presses `c`
- **THEN** the first timezone line SHALL never include a day suffix, as it serves as the reference day

### Requirement: Time format matches display setting
The copied text SHALL use the same 12h/24h format as the current TUI display.

#### Scenario: Copy in 12h mode
- **WHEN** the app is in 12h mode and the user presses `c`
- **THEN** times SHALL be formatted as `4pm` or `11am` (no space before am/pm, no leading zero)

#### Scenario: Copy in 24h mode
- **WHEN** the app is in 24h mode and the user presses `c`
- **THEN** times SHALL be formatted as `16:00` or `09:00`

### Requirement: Visual feedback on copy
The system SHALL display a brief confirmation in the footer when a copy operation succeeds.

#### Scenario: Successful copy shows confirmation
- **WHEN** the user presses `c` and the clipboard write succeeds
- **THEN** the footer SHALL display a "Copied!" message that reverts to the normal shortcut bar after approximately 2 seconds

#### Scenario: Failed copy shows no crash
- **WHEN** the user presses `c` but clipboard access fails (e.g., headless environment)
- **THEN** the app SHALL NOT crash and MAY show an error hint in the footer

### Requirement: Copy shortcut shown in footer
The footer shortcut bar SHALL include the `c` key with a "Copy" label.

#### Scenario: Footer displays copy shortcut
- **WHEN** the TUI is displayed
- **THEN** the footer SHALL show ` c ` with label ` Copy ` alongside existing shortcuts
