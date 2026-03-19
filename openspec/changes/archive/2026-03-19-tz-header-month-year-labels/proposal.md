## Why

When scrolling the timeline across month or year boundaries, the day label (e.g., "WED 1") gives no indication that the month or year has changed. This makes it easy to misread dates — "WED 1" could be the current month or next month. Adding contextual month/year labels removes ambiguity.

## What Changes

- Day labels in the timeline header row gain a conditional month suffix when the displayed date falls in a different month than today's date (e.g., `WED 1, April` instead of `WED 1`).
- Day labels gain a conditional year suffix when the displayed date falls in a different year than today's year (e.g., `WED 1, April, 2027`).
- Same-month dates remain unchanged (`THU 19`).
- Copy-to-clipboard output applies the same conditional formatting for day suffixes on different-month/year dates.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `multi-column-day-label`: Day label format adds conditional month name when month differs from today, and conditional year when year differs from today.
- `copy-selection`: Day suffix in copied text adds conditional month and year following the same rules as the TUI label.

## Impact

- `src/tui/render.rs` — `build_day_spans` function: day label string construction changes.
- `src/tui/copy.rs` — `build_copy_text` method: day suffix formatting changes.
- Existing tests for day label alignment and copy output need updating.
