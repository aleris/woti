## ADDED Requirements

### Requirement: Day marker displays date alongside day name
The timeline day marker row SHALL display the short weekday name followed by a space and the numeric day-of-month (e.g., "WED 18", "THU 1") at each day boundary.

#### Scenario: Day marker format at midnight boundary
- **WHEN** the timeline crosses midnight into Wednesday March 18
- **THEN** the day marker row displays "WED 18" starting at the midnight column

#### Scenario: Single-digit date
- **WHEN** the timeline crosses midnight into Thursday March 1
- **THEN** the day marker row displays "THU 1" starting at the midnight column

### Requirement: Day label spans multiple columns
The day marker label SHALL span across as many 3-character columns as needed to display the full "DAY DD" text, without being truncated by column boundaries.

#### Scenario: Label spanning two columns
- **WHEN** "WED 18" (6 characters) is rendered starting at a column boundary
- **THEN** the label text occupies character positions across two adjacent 3-character columns

#### Scenario: Label truncated at timeline edge
- **WHEN** a day marker label would extend beyond the right edge of the visible timeline
- **THEN** the label is truncated to fit within the available space

### Requirement: Day label alignment with hour row
The day marker label SHALL start at the same character position as the hour number for the midnight (hour 0) column, preserving vertical alignment between rows.

#### Scenario: Vertical alignment of day and hour
- **WHEN** the midnight column for a day is at position N in the timeline
- **THEN** the day label "WED 18" starts at character position N and the hour "0" also starts at character position N

### Requirement: Day label styling preserved
The day marker label SHALL use the same styling rules as the current implementation: magenta bold for normal days, cell highlight style for the selected hour column, and local-hour background style when applicable.

#### Scenario: Selected hour at midnight
- **WHEN** the selected hour falls on a midnight boundary
- **THEN** the day label characters within the selected column use the selected highlight style (black on yellow)

#### Scenario: Normal day marker styling
- **WHEN** a day label is displayed outside the selected and local hour columns
- **THEN** the label text is styled in magenta bold
