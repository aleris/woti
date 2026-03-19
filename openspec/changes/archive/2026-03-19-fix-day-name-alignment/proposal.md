## Why

The day label (e.g., "MON 5") on the top row of each timezone block starts at the cell boundary, which is the gap-space column before the hour digit. This makes it visually misaligned — the "M" of "MON" sits above the inter-cell gap rather than above the first character of the hour number below it. The label should start at the same column as the hour digits to look correct.

## What Changes

- Shift the day label start position in `build_day_spans` by +1 character so it aligns with the first character of the 2-char hour number field, not the inter-cell gap space.
- Handle both single-digit dates ("MON 5", 5 chars) and double-digit dates ("TUE 12", 6 chars) correctly at the new offset.
- Update the existing `multi-column-day-label` spec to reflect the corrected alignment rule.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `multi-column-day-label`: The "Day label alignment with hour row" requirement needs updating — the label should start at the hour-digit column (cell_start + 1), not at the cell boundary (cell_start).

## Impact

- `src/tui/render.rs` — `build_day_spans` function (start position calculation and potential bounds adjustment).
- `openspec/specs/multi-column-day-label/spec.md` — alignment requirement wording.
