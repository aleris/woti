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

### Requirement: Day label alignment with hour row
The day marker label SHALL start at the same character position as the first displayed digit of the midnight hour number. Each cell is laid out as [gap-space][digit1][digit2] where the hour is formatted with `{:>2}`. The digit offset depends on whether the midnight hour display is single or double digit: +2 for single-digit (24h: "0"), +1 for double-digit (12h: "12").

#### Scenario: Vertical alignment in 24-hour format
- **WHEN** the midnight column is at cell index N and 24-hour format is active
- **THEN** the day label starts at character position `N * cell_width + 2`, aligning with the digit "0" in the right-aligned `" 0"` display

#### Scenario: Vertical alignment in 12-hour format
- **WHEN** the midnight column is at cell index N and 12-hour format is active
- **THEN** the day label starts at character position `N * cell_width + 1`, aligning with the digit "1" in the `"12"` display

#### Scenario: Label near right edge of timeline
- **WHEN** the day label would extend beyond the available timeline character positions due to the offset
- **THEN** the label is truncated to fit within the available space

### Requirement: Day label styling preserved
The day marker label SHALL use the same styling rules as the current implementation: DAY_LABEL colored bold for normal days, cell highlight style for the selected hour column, and local-hour background style when applicable. Additionally, when the selected or local hour falls on the midnight boundary that generated a day label, the highlight style SHALL extend across all character positions belonging to that label (not just the single cell).

#### Scenario: Selected hour at midnight
- **WHEN** the selected hour falls on a midnight boundary
- **THEN** all day label characters originating from that midnight use the selected highlight style (SELECTED_FG on SELECTED_BG, bold)

#### Scenario: Normal day marker styling
- **WHEN** a day label is displayed outside the selected and local hour columns and not connected to a selected/local midnight label
- **THEN** the label text is styled in DAY_LABEL colored bold

#### Scenario: Day label extends beyond selected cell
- **WHEN** the selected hour is at midnight and the day label spans multiple cells
- **THEN** the highlight extends across the full label text, not truncated at cell boundaries
