### Requirement: Relative offset label is displayed when selection is not current hour

The system SHALL render a single indicator row below all timezone blocks that displays a human-readable string
describing how many hours the selection is from the current time. The row SHALL only be visible when `hour_offset != 0`.

#### Scenario: Selection is 2 hours in the future

- **WHEN** the user navigates 2 hours to the right (`hour_offset == 2`)
- **THEN** the indicator row displays "in 2 hours"

#### Scenario: Selection is 3 hours in the past

- **WHEN** the user navigates 3 hours to the left (`hour_offset == -3`)
- **THEN** the indicator row displays "3 hours ago"

#### Scenario: Selection is exactly 1 hour in the future

- **WHEN** `hour_offset == 1`
- **THEN** the indicator row displays "in 1 hour" (singular)

#### Scenario: Selection is exactly 1 hour in the past

- **WHEN** `hour_offset == -1`
- **THEN** the indicator row displays "1 hour ago" (singular)

#### Scenario: Selection is the current hour

- **WHEN** `hour_offset == 0`
- **THEN** no indicator row is rendered

### Requirement: Indicator uses selection color

The indicator label SHALL be styled with the same foreground and background colors as the selected cell (`SELECTED_FG` /
`SELECTED_BG`, bold).

#### Scenario: Style matches selected column

- **WHEN** the indicator row is visible
- **THEN** the label span uses `selected_style()` (fg: `SELECTED_FG`, bg: `SELECTED_BG`, bold)

### Requirement: Indicator is horizontally aligned with selected column

The indicator label SHALL be horizontally positioned to align with the selected cell column in the timeline grid.

#### Scenario: Label aligns with selected cell

- **WHEN** the indicator row is rendered
- **THEN** the label's leading padding matches `info_w + TIMELINE_GAP + (selected_cell_index * CELL_WIDTH)` so it sits
  under the highlighted column
