## Why

The TUI always opens anchored to the current wall-clock time (`Utc::now()`). There is no way to launch it showing a
different date or time — for example, to check what time zones look like at a future meeting time or to review a past
event. Adding `--date` and `--time` CLI options lets users pin the display to an arbitrary point in time while keeping
the current behavior as the default.

## What Changes

- Add a `--date` option (`YYYY-MM-DD` ISO 8601) to the CLI that sets the initial date shown in the TUI. When only `--date` is given, the time defaults to the current local time of day.
- Add a `--time` option (`HH:MM` or `HH:MM:SS` ISO 8601) to the CLI that sets the initial time shown in the TUI. When only `--time` is given, the date defaults to today.
- When neither flag is provided, behavior is unchanged — the TUI uses `Utc::now()` as today.
- When a flag is provided, the TUI anchors to the specified date/time in the local timezone instead of live-updating
  from the system clock.
- The `hour_offset` navigation (Left/Right keys) continues to work relative to the anchored time.
- The copy feature reflects the anchored time rather than the live clock.

## Capabilities

### New Capabilities

- `cli-datetime-override`: Accept `--date` and `--time` flags on the root command, parse ISO 8601 values, and thread the
  resulting anchor time through the TUI so it replaces live `Utc::now()` calls.

### Modified Capabilities

## Impact

- `src/cli.rs` — new optional fields on `Cli`.
- `src/main.rs` — pass parsed date/time into `cmd_tui()`.
- `src/tui/app.rs` — `App` stores an optional anchor `DateTime<Utc>` and exposes a `now()` helper.
- `src/tui/render.rs` — replace bare `Utc::now()` calls with the anchor-aware helper.
- `src/tui/copy.rs` — same replacement.
- `chrono` parsing utilities used; no new crate dependencies.
