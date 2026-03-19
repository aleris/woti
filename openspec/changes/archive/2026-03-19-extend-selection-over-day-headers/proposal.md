## Why

When the selected hour lands on a day boundary (midnight / start of a new day), the day label text (e.g., "WED 19") spans multiple hour cells in the top row, but the selection highlight only covers the single selected cell. This causes the highlight to cut off mid-label, creating an awkward visual break in the middle of the day name or number.

## What Changes

- Extend the selection highlight in the day row (`build_day_spans`) to cover the full extent of any day label that overlaps the selected cell, rather than just the single cell at `base_hour`.
- When the selected cell is at `hour_in_day == 0` (midnight), the entire day label text (day name, number, optional month/year) should receive the selection background so the highlight looks continuous and natural.
- Apply the same logic for the local-hour highlight when it falls on a day boundary.

## Capabilities

### New Capabilities

- `day-label-selection-extension`: Extend the selection/local highlight across the full day label span when the selected or local hour is at a day boundary.

### Modified Capabilities

- `multi-column-day-label`: The existing day label rendering needs to be aware of selection state across its full span, not just per-cell.

## Impact

- `src/tui/render.rs` — `build_day_spans` function: the `style_for` closure needs to know which positions belong to a day label that started at the selected (or local) cell, and extend the highlight background across all of them.
- No new dependencies, no API changes, no breaking changes.
