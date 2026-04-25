## MODIFIED Requirements

### Requirement: Relative offset label is displayed when selection is not current hour

The system SHALL render a single indicator row below all timezone blocks that displays a human-readable string describing how far the selection is from the current time. The row SHALL only be visible when the selection is non-zero (i.e. `cell_offset != 0`, equivalent to `selected_offset_minutes != 0`). The label SHALL express the offset using the most natural unit combination for the active interval:

- Pure-hour offsets (multiple of 60 minutes) → "in N hour(s)" / "N hour(s) ago".
- Pure sub-hour offsets (less than 60 minutes) → "in N minutes" / "N minutes ago".
- Mixed offsets (>= 60 minutes and not a clean hour multiple) → "in H hour(s) M minutes" / "H hour(s) M minutes ago".

Singular/plural rules SHALL apply to each unit independently ("1 hour", "2 hours", "1 minute", "30 minutes").

#### Scenario: Selection is 2 hours in the future (60-minute interval)
- **WHEN** the user navigates 2 hours to the right at 60-minute interval (`selected_offset_minutes == 120`)
- **THEN** the indicator row displays "in 2 hours"

#### Scenario: Selection is 3 hours in the past (60-minute interval)
- **WHEN** the user navigates 3 hours to the left at 60-minute interval (`selected_offset_minutes == -180`)
- **THEN** the indicator row displays "3 hours ago"

#### Scenario: Selection is exactly 1 hour in the future
- **WHEN** `selected_offset_minutes == 60`
- **THEN** the indicator row displays "in 1 hour" (singular)

#### Scenario: Selection is exactly 1 hour in the past
- **WHEN** `selected_offset_minutes == -60`
- **THEN** the indicator row displays "1 hour ago" (singular)

#### Scenario: Selection is 30 minutes in the future
- **WHEN** the active interval is 30 minutes and the user navigates one cell right (`selected_offset_minutes == 30`)
- **THEN** the indicator row displays "in 30 minutes"

#### Scenario: Selection is 15 minutes in the past
- **WHEN** the active interval is 15 minutes and the user navigates one cell left (`selected_offset_minutes == -15`)
- **THEN** the indicator row displays "15 minutes ago"

#### Scenario: Selection is exactly 1 minute (defensive, not currently reachable)
- **WHEN** `selected_offset_minutes == 1`
- **THEN** the indicator row displays "in 1 minute" (singular)

#### Scenario: Mixed offset in the future
- **WHEN** the active interval is 15 minutes and the selection is at `selected_offset_minutes == 75`
- **THEN** the indicator row displays "in 1 hour 15 minutes"

#### Scenario: Mixed offset in the past
- **WHEN** the active interval is 30 minutes and the selection is at `selected_offset_minutes == -150`
- **THEN** the indicator row displays "2 hours 30 minutes ago"

#### Scenario: Selection is the current cell
- **WHEN** `selected_offset_minutes == 0`
- **THEN** no indicator row is rendered

### Requirement: Indicator is horizontally aligned with selected column

The indicator label SHALL be horizontally positioned to align with the selected cell column in the timeline grid, regardless of the active interval.

#### Scenario: Label aligns with selected cell
- **WHEN** the indicator row is rendered
- **THEN** the label's leading padding matches `info_w + TIMELINE_GAP + (selected_cell_index * CELL_WIDTH)` so it sits under the highlighted column

#### Scenario: Alignment at 15-minute interval
- **WHEN** the active interval is 15 minutes and the selected cell is the 6th visible cell
- **THEN** the label is padded to the same column position as the 6th cell, exactly as it would be at 60-minute interval
