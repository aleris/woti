### Requirement: Selection highlight extends across full day label
When the selected hour falls on a day boundary (midnight, `hour_in_day == 0`), the selection highlight in the day row SHALL extend across all character positions belonging to that day label, not just the single cell at the selected hour.

#### Scenario: Selected hour at midnight with short label
- **WHEN** the selected hour is midnight and the day label is "WED 19" (6 characters spanning 3 cells)
- **THEN** all 6 label characters receive the selected highlight style (SELECTED_FG on SELECTED_BG, bold)

#### Scenario: Selected hour at midnight with month suffix
- **WHEN** the selected hour is midnight crossing into a different month and the day label is "WED 1, April" (13 characters)
- **THEN** all 13 label characters receive the selected highlight style across however many cells the label spans

#### Scenario: Selected hour at midnight with year suffix
- **WHEN** the selected hour is midnight crossing into a different year and the day label is "THU 1, January, 2027"
- **THEN** all label characters receive the selected highlight style

#### Scenario: Selected hour NOT at midnight
- **WHEN** the selected hour is not a day boundary (e.g., 14:00)
- **THEN** the day row highlight remains limited to the single selected cell (existing behavior unchanged)

#### Scenario: Label truncated at timeline edge
- **WHEN** the selected hour is at midnight and the day label extends beyond the visible timeline
- **THEN** only the visible portion of the label receives the selected highlight style

### Requirement: Local-hour highlight extends across full day label
When the local hour (current time) falls on a day boundary (midnight) and the user has scrolled away (`hour_offset != 0`), the local-hour highlight in the day row SHALL extend across all character positions belonging to that day label.

#### Scenario: Local hour at midnight
- **WHEN** the local hour is midnight and hour_offset is non-zero
- **THEN** all characters of that midnight's day label receive the local-hour highlight style (LOCAL_BG with DAY_LABEL foreground, bold)

#### Scenario: Local hour not at midnight
- **WHEN** the local hour is not a day boundary
- **THEN** the day row local highlight remains limited to the single local-hour cell (existing behavior unchanged)

### Requirement: Gap spaces between label characters remain unhighlighted
Character positions that are not part of the day label text (gap spaces at `pos_in_cell == 0` that are not overwritten by label text) SHALL NOT receive the extended highlight, preserving visual separation between cells.

#### Scenario: Leading space not overwritten by label
- **WHEN** a gap space at `pos_in_cell == 0` is not part of the day label text (`day_is_label[pos] == false`)
- **THEN** that position does not receive selection or local highlight background
