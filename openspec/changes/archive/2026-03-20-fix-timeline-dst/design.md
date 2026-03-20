## Context

The timeline rendering in `src/tui/render.rs` has three builder functions — `build_hour_spans`, `build_ampm_spans`, and
`build_day_spans` — that each compute the displayed hour with pure arithmetic: `((h % 24) + 24) % 24`. This modular
arithmetic assumes uniform 24-hour days and produces incorrect results when the user scrolls across a DST transition
boundary.

A DST-aware function `compute_datetime_for_hour` already exists in the same file (line 17) and is used correctly by
`copy.rs` and by `build_day_spans` for the day-label text (but **not** for the midnight boundary check itself). The
function adds `chrono::Duration::hours(offset)` to the current timezone-aware time, so chrono handles all DST
folding/gaps automatically.

## Goals / Non-Goals

**Goals:**

- Every rendered timeline element (hour digit, AM/PM suffix, working-hours shade color, day-boundary detection) uses the
  wall-clock hour from `compute_datetime_for_hour` instead of arithmetic.
- Visual output matches the text produced by `copy.rs` for the same hour offset.

**Non-Goals:**

- Handling sub-hour DST offsets (e.g., Lord Howe Island's 30-minute shift). These are out of scope; the existing
  `compute_datetime_for_hour` already returns the correct time, but the cell grid is hourly.
- Changing the data model or adding new structs. This is a localized fix inside three existing functions.

## Decisions

**Use `compute_datetime_for_hour` per cell instead of adding a precomputed cache.**

Each of the three builder functions iterates `num_cells` (typically ~40 visible columns). Calling
`compute_datetime_for_hour` per cell adds a `chrono::Duration` addition per iteration — trivially cheap. A precomputed
`Vec<u32>` of resolved hours would save nothing meaningful and would add coupling between the builders. Keeping the call
inline in each loop matches the existing pattern in `build_day_spans` (line 422) and `copy.rs` (line 26).

**Derive `hour_in_day` as `dt.hour() as i32` for compatibility with existing helper signatures.**

`hour_fg_color` and `ampm_fg_color` accept `i32`. Casting `u32 → i32` from `.hour()` is safe (range 0–23). This avoids
changing the helper signatures and keeps the diff minimal.

**Midnight detection: check `dt.hour() == 0` instead of arithmetic `hour_in_day == 0`.**

During spring-forward, arithmetic midnight (`h % 24 == 0`) is correct because the skipped hour is never 0. During
fall-back, the repeated hour is 1 (in US zones), so midnight detection is also unaffected. However, using
`dt.hour() == 0` is semantically correct for all zones and makes the code consistent with the rest of the fix.

## Risks / Trade-offs

**[Minor perf cost]** → One `compute_datetime_for_hour` call per cell per builder (3× calls per cell). Negligible for <
50 cells with simple `DateTime + Duration` arithmetic; no system calls involved.

**[Behavioral change at DST gaps]** → During spring-forward, hour 2 (which doesn't exist) will no longer appear in the
timeline — the display will jump from 1 to 3, matching wall-clock reality. This is the correct behavior but is a visible
change. → No mitigation needed; this is the intended fix.

**[Behavioral change at DST overlaps]** → During fall-back, the repeated hour will display correctly since
`compute_datetime_for_hour` returns the later offset (chrono's default). The timeline will show the hour once, matching
the linear scrolling model. → Acceptable; the timeline is a linear hour-offset slider, not a wall-clock playback.
