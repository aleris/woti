## MODIFIED Requirements

### Requirement: Day marker displays date alongside day name
The timeline day marker row SHALL display the short weekday name followed by a space and the numeric day-of-month (e.g., "WED 18", "THU 1") at each day boundary. When the displayed date's month differs from today's month in that timezone, the label SHALL append a comma, space, and the full month name (e.g., "WED 1, April"). When the displayed date's year also differs from today's year in that timezone, the label SHALL further append a comma, space, and the four-digit year (e.g., "WED 1, April, 2027").

#### Scenario: Day marker format at midnight boundary (same month)
- **WHEN** the timeline crosses midnight into Wednesday March 18 and today is also in March
- **THEN** the day marker row displays "WED 18" starting at the midnight column

#### Scenario: Single-digit date (same month)
- **WHEN** the timeline crosses midnight into Thursday March 1 and today is also in March
- **THEN** the day marker row displays "THU 1" starting at the midnight column

#### Scenario: Day marker when month changes
- **WHEN** the timeline crosses midnight into Wednesday April 1 and today is in March
- **THEN** the day marker row displays "WED 1, April" starting at the midnight column

#### Scenario: Day marker when year changes
- **WHEN** the timeline crosses midnight into Thursday January 1, 2027 and today is in 2026
- **THEN** the day marker row displays "THU 1, January, 2027" starting at the midnight column

#### Scenario: Day marker when month changes but year is same
- **WHEN** the timeline crosses midnight into Sunday June 1 and today is March 19 of the same year
- **THEN** the day marker row displays "SUN 1, June" without a year suffix

### Requirement: Day label spans multiple columns
The day marker label SHALL span across as many 3-character columns as needed to display the full label text (including any month/year suffix), without being truncated by column boundaries.

#### Scenario: Label spanning columns with month suffix
- **WHEN** "WED 1, April" (13 characters) is rendered starting at a column boundary
- **THEN** the label text occupies character positions across adjacent 3-character columns

#### Scenario: Label truncated at timeline edge
- **WHEN** a day marker label (with or without month/year suffix) would extend beyond the right edge of the visible timeline
- **THEN** the label is truncated to fit within the available space
