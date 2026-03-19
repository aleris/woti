## MODIFIED Requirements

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
