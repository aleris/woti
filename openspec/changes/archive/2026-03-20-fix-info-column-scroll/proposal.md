## Why

When the user scrolls the timeline (`hour_offset != 0`), the info column on the left side of each timezone row continues
to display properties derived from the current wall-clock time rather than the selected time. This causes the timezone
abbreviation, UTC offset badge, displayed time, and date to be wrong whenever scrolling crosses a DST boundary or date
boundary.

## What Changes

- Derive `tz_abbr`, `time_str`, `date_str`, and `offset_m` from the selected datetime (computed via
  `compute_datetime_for_hour`) when `hour_offset != 0`, instead of from `now_tz`.
- Update `build_info_line` to receive the correct datetime so the UTC offset badge reflects the selected time.
- Ensure `TimelineParams.offset_m` is also derived from the selected datetime, fixing the superscript-minutes indicator
  for zones with sub-hour DST transitions (e.g., Australia/Lord_Howe).

## Capabilities

### New Capabilities

- `scroll-aware-info-column`: Info column values (abbreviation, time, date, UTC offset) reflect the selected datetime
  when the timeline is scrolled.

### Modified Capabilities

## Impact

- `src/tui/render.rs`: `render_timezone_row` (lines ~176-254) — change where `tz_abbr`, `time_str`, `date_str`,
  `offset_m` are sourced; `build_info_line` (line ~504) — its `now_tz` parameter becomes the selected datetime.
- No API, dependency, or configuration changes.
