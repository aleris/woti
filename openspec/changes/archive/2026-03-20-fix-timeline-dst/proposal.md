## Why

The timeline hour labels, AM/PM indicators, and day-boundary markers are computed using pure modular arithmetic (
`((h % 24) + 24) % 24`), which assumes every day has exactly 24 uniform hours. When a user scrolls across a DST
transition (e.g., spring-forward or fall-back), the displayed hours diverge from the actual wall-clock time. Meanwhile,
`copy.rs` correctly uses `compute_datetime_for_hour` and `.hour()`, so the copied text shows a different hour than
what's on screen.

## What Changes

- Replace arithmetic `hour_in_day` computation with `compute_datetime_for_hour(...).hour()` in `build_hour_spans`,
  `build_ampm_spans`, and the midnight check in `build_day_spans`.
- Working-hours shading and AM/PM labels will derive their hour from the DST-aware datetime instead of raw arithmetic.
- The midnight boundary check (`hour_in_day == 0`) will use the DST-aware hour so day labels shift correctly across DST
  transitions.

## Capabilities

### New Capabilities

- `dst-aware-timeline`: Ensure all rendered timeline elements (hour digits, AM/PM labels, working-hours shading, day
  boundaries) reflect wall-clock time accounting for DST transitions.

### Modified Capabilities

## Impact

- `src/tui/render.rs`: `build_hour_spans`, `build_ampm_spans`, `build_day_spans` — the three functions that currently
  use arithmetic hour computation.
- No API, dependency, or configuration changes.
- No breaking changes — the fix makes displayed hours match what `compute_datetime_for_hour` already returns.
