## Context

The TUI renders timezone blocks in a vertical list, each exactly `BLOCK_HEIGHT` (3) rows tall: day labels, hour
numbers + city info, and am/pm markers + region. Users navigate left/right to shift `hour_offset`, but there is no
visual cue showing how far from "now" the selection sits. The main layout is header (1 row) → body (flex) → footer (1
row).

## Goals / Non-Goals

**Goals:**

- Display a single indicator row below all timezone blocks when `hour_offset != 0`.
- Show a human-readable relative string (e.g., "in 2 hours", "3 hours ago").
- Use the selection color (`SELECTED_FG` / `SELECTED_BG`) so it visually ties to the highlighted column.
- Horizontally position the label to align with the selected column in the timeline grid.

**Non-Goals:**

- Per-timezone offset indicators (one global row is sufficient).
- Minute-level granularity — only whole hours.
- Persisting or configuring the indicator visibility.

## Decisions

### 1. Render location: bottom of body area, not a new layout row

The indicator row is rendered inside `render_body` after the timezone blocks, consuming 1 row of the body area. This
avoids changing the top-level layout split and keeps the indicator naturally scrollable if many timezones push it
off-screen.

**Alternative considered**: A dedicated layout row between body and footer. Rejected because it would always reserve
vertical space even when `hour_offset == 0`, and would require conditional layout changes.

### 2. Formatting function: pure function `format_hour_offset(offset: i32) -> String`

A standalone function that returns the human-readable string:

- `offset > 0` → `"in {n} hour(s)"`
- `offset < 0` → `"{n} hour(s) ago"`
- `offset == 0` → empty (caller skips rendering)

Singular/plural: "1 hour" vs "2 hours".

### 3. Horizontal alignment via the same left-pad + cell math

Reuse the existing `info_w + TIMELINE_GAP + cell offset` calculation from `render_timezone_block` to position the label
at the selected column. The label is placed at the selected cell's x-position within a single-line `Paragraph`.

### 4. Style matches selection exactly

Use `selected_style()` (already defined) for the label span so color stays consistent if the theme changes.

## Risks / Trade-offs

- **[Vertical space]** The indicator row reduces the visible timezone count by 1 when active. → Acceptable since it only
  appears during navigation and provides high-value context.
- **[Long strings]** At extreme offsets (e.g., 100+ hours) the string could be wide. → At that point the user is
  deliberately exploring far out; truncation is unnecessary since the string stays compact ("in 100 hours" = 12 chars).
