## Context

`render_timezone_row` in `src/tui/render.rs` computes `tz_abbr`, `time_str`, `date_str`, and `offset_m` from `now_tz` (the current wall-clock time in each timezone). The timeline itself already uses `compute_datetime_for_hour` to render the correct labels for scrolled hours, but the info column never consults `hour_offset` to shift its own values.

The same `now_tz` is also passed into `build_info_line`, which re-derives the UTC offset for the badge display — so the badge is equally stale when scrolling.

`copy.rs` already demonstrates the correct pattern: it calls `compute_datetime_for_hour(tz, now_tz, self.hour_offset)` and derives all display values from the result.

## Goals / Non-Goals

**Goals:**

- When `hour_offset != 0`, the info column (timezone abbreviation, time, date, UTC offset badge, superscript minutes) reflects the selected datetime, not the current time.
- When `hour_offset == 0`, behavior is unchanged — values come directly from `now_tz` as before.

**Non-Goals:**

- Changing the timeline rendering logic (it already works correctly).
- Altering `compute_datetime_for_hour` semantics.
- Changing behavior of the copy feature (it already uses the selected datetime).

## Decisions

**Decision 1: Compute a `selected_dt` alongside `now_tz` in `render_timezone_row`**

Follow the same pattern as `copy.rs`: call `compute_datetime_for_hour(tz, now_tz, self.hour_offset)` to get a `selected_dt`. Derive `tz_abbr`, `time_str`, `date_str`, and `offset_m` from `selected_dt` instead of `now_tz`.

*Alternative considered*: Pass `hour_offset` into `build_info_line` and let it compute internally. Rejected because the caller already needs `offset_m` for `TimelineParams`, so computing once and passing derived values is cleaner.

**Decision 2: Pass `selected_dt` to `build_info_line` instead of `now_tz`**

`build_info_line` receives its datetime parameter and derives the UTC offset from it. Changing the argument from `now_tz` to `selected_dt` fixes the offset badge with zero signature change — the parameter is already `chrono::DateTime<Tz>`.

**Decision 3: Keep `now_tz` for `current_hour` and `TimelineParams.now_tz`**

The timeline rendering still needs the real current hour to mark the "local now" column. `now_tz` remains the source for `current_hour` and `TimelineParams.now_tz`. Only the info-column derivations change.

## Risks / Trade-offs

- **[Low] Display when scrolled past DST transition is approximate**: `compute_datetime_for_hour` adds `Duration::hours(offset)` to `now_tz`. If a DST transition falls within the offset window, chrono automatically adjusts, so abbreviation and offset will be correct at the target hour. No additional risk.
- **[Low] `time_str` shows `:00` minutes when scrolled**: The selected datetime is always on the hour boundary. This is consistent with the existing timeline cells and copy behavior. If sub-hour precision were desired later, the info column could show just the hour; this is out of scope for this fix.
