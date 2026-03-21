## Why

When navigating hours left/right, it's not immediately obvious how far from "now" the selection is. A compact
relative-time label ("in 2 hours", "3 hours ago") anchored to the selection gives instant context without requiring
mental math across timezone rows.

## What Changes

- Add a new row rendered below all timezone blocks that displays a human-readable relative offset string (e.g., "in 2
  hours", "5 hours ago").
- The row is only visible when the selection is not the current hour (`hour_offset != 0`).
- The label uses the same color as the selected cell (`SELECTED_FG` / `SELECTED_BG`).
- The label is horizontally aligned with the selected column in the timeline.

## Capabilities

### New Capabilities

- `hour-offset-indicator`: Render a relative-time indicator row below timezone blocks showing how far the selection is
  from now.

### Modified Capabilities

## Impact

- `src/tui/render.rs` — new rendering logic for the indicator row, minor changes to body layout.
- `src/tui/mod.rs` — `BLOCK_HEIGHT` may remain unchanged; the indicator is outside per-timezone blocks.
- No new dependencies or API changes.
