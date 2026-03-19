## MODIFIED Requirements

### Requirement: Copy selected hour to clipboard
The system SHALL copy the currently highlighted hour column for all configured timezones to the system clipboard when the user presses the `c` key.

#### Scenario: Basic copy with all timezones on the same day
- **WHEN** the user presses `c` with the selected hour column highlighting 4pm across timezones that all fall on the same calendar day
- **THEN** the clipboard SHALL contain one line per timezone in format `City / Abbreviation Hour[am/pm]` (e.g., `Bucharest / EET 4pm`)

#### Scenario: Copy when a timezone falls on a different day but same month
- **WHEN** the user presses `c` and one or more timezones' selected hour falls on a different calendar day than the first timezone, but in the same month
- **THEN** those lines SHALL append the abbreviated day name (uppercase 3-letter) and day-of-month number (e.g., `San Jose / PDT 8pm WED 19`)

#### Scenario: Copy when a timezone falls in a different month
- **WHEN** the user presses `c` and one or more timezones' selected hour falls in a different month than the first timezone's date
- **THEN** those lines SHALL append the day name, day number, comma, and full month name (e.g., `San Jose / PDT 8pm WED 1, April`)

#### Scenario: Copy when a timezone falls in a different year
- **WHEN** the user presses `c` and one or more timezones' selected hour falls in a different year than the first timezone's date
- **THEN** those lines SHALL append the day name, day number, comma, full month name, comma, and four-digit year (e.g., `San Jose / PDT 8pm WED 1, January, 2027`)

#### Scenario: First timezone never shows day suffix
- **WHEN** the user presses `c`
- **THEN** the first timezone line SHALL never include a day suffix, as it serves as the reference day
