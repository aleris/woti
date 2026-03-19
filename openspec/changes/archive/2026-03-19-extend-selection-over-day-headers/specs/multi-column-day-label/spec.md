## MODIFIED Requirements

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
