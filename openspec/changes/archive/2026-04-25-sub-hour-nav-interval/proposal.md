## Why

The timeline currently shows one cell per whole hour, so navigating left/right always moves the selection by exactly 1
hour. Users planning meetings, calls, or events at quarter-hour or half-hour boundaries (the most common real-world
meeting cadences) have no way to land the selection on `:15`, `:30`, or `:45` and no visual cue for sub-hour boundaries
between hours. A configurable navigation interval — selectable per session via a CLI flag and per-view via a TUI
shortcut — adds finer-grained navigation without changing the default.

## What Changes

- Add a CLI flag `--interval <minutes>` accepting `60`, `30`, or `15`. Like `--date` and `--time`, it is a top-level
  flag that applies only to the TUI launch path; combining it with a subcommand is an error.
- Add a TUI shortcut `i` that cycles the active interval `60 → 30 → 15 → 60`. The interval picker is rendered in the
  footer aligned to the right alongside the existing `Shade` (`w`) and Format (`f mx│am│24`) switchers.
- The interval is **persisted** in `config.toml` (new field `interval = 60 | 30 | 15`, default `60`). Pressing `i`
  writes the new value to disk on each cycle, matching how `f` (time format) and `w` (shading) already behave.
- The `--interval` flag is a **session override** that does not write back to the config file (mirroring `--date` /
  `--time`); the saved value is preserved for the next launch.
- On launch the resolution order is: `--interval` flag if present, otherwise the persisted `config.interval`, otherwise
  `60` (default).
- The cell width (`CELL_WIDTH = 3`) and per-cell content slot (2 chars) are unchanged. What changes per interval is *
  *how many cells are rendered between two adjacent hour cells**, with each Left/Right press advancing the selection by
  exactly one cell.
- Per-interval rendering rules:
    - **60 min (default):** the view is **completely unchanged** — no intermediate cells, no tick glyphs, no minute
      markers. Every cell is an hour cell. Each Left/Right press = 1 hour. This must be byte-for-byte identical to
      today.
    - **30 min:** **one** intermediate cell is inserted between every pair of hour cells. The hour row renders `·` in
      that cell; the sub-row renders the wall-clock minute as a superscript (`³⁰` for whole-hour zones at `:30`, etc.).
      Hour cells render the hour digit and keep their existing sub-row content (am/pm, blank, `³⁰`/`⁴⁵`, `½a` …). Each
      Left/Right press = 30 min.
    - **15 min:** **three** intermediate cells are inserted between every pair of hour cells, matching the example in
      `view.md` (`20  ·  ·  ·  21` on the hour row, `   ¹⁵ ³⁰ ⁴⁵   ` on the sub-row). Each Left/Right press = 15 min.
- Hour cells in sub-hour modes keep their existing sub-row glyph unchanged (am/pm, blank, `³⁰`/`⁴⁵`, `½a`/`¾p`, …). For
  whole-hour-offset zones in 24h mode, that means the hour-cell sub-row stays blank (`"  "`) in every interval — the
  hour digit on the row above already marks the slot, and leaving the sub-row blank gives stronger visual contrast with
  the `¹⁵`/`³⁰`/`⁴⁵` markers on the intermediate cells.
- The hour-offset indicator label (currently "in N hours" / "N hours ago") is extended to render sub-hour offsets —
  e.g. "in 30 minutes", "1 hour 15 minutes ago" — and to align with the selected sub-hour cell.
- The reference time displayed in each timezone's info column (the 2nd-row time string) continues to reflect the
  selected cell, so navigating to a `:15` slot updates the displayed time accordingly.

## Capabilities

### New Capabilities

- `sub-hour-nav-interval`: Configurable timeline navigation interval (60/30/15 min) via `--interval` flag and `i`
  shortcut, with intermediate tick rendering and minute markers between hours.

### Modified Capabilities

- `accelerated-nav`: Single-press navigation step is now expressed in interval-cells rather than hardcoded hours; with
  `--interval 15` one tap = 15 minutes, acceleration ramps multiply the cell step exactly as before.
- `hour-offset-indicator`: The label SHALL format sub-hour offsets as "in N minutes" / "N minutes ago" or composed "H
  hour(s) M minutes" forms when the active interval is < 60 min.
- `half-hour-timeline-indicator`: Hour-cell sub-row content rules are clarified for sub-hour intervals — for
  whole-hour 24h zones the hour cell stays blank (the hour digit above marks the slot). For fractional-offset
  zones (Nepal, India) intermediate cells that land on wall `:00` render `⁰⁰`, since the row above only shows
  a `·` tick on intermediates and the sub-row needs to mark the natural hour boundary. All other hour-cell
  glyphs (`am`/`pm`, `½a`, `³⁰`, `⁴⁵`) are unchanged.

## Impact

- `src/cli.rs` — add `--interval` flag on `Cli` (only valid without a subcommand).
- `src/main.rs` — parse `--interval`, validate accepted values, thread into `cmd_tui()`.
- `src/tui/mod.rs` — module-level constants and possibly a new `NavInterval` enum.
- `src/tui/app.rs` — add an `interval` field to `App`; `App::new` accepts an initial value; cycle helper for the `i`
  shortcut.
- `src/tui/event.rs` — handle the `i` key, replace hardcoded "1 hour per cell" arithmetic on Left/Right with
  `interval_minutes` per cell; rename `hour_offset` to a cell-based offset internally (or keep the name but interpret as
  cells).
- `src/tui/render.rs` — `TimelineParams` gains `interval_minutes`; `build_hour_spans` emits `·` on intermediate cells;
  `build_ampm_spans` emits superscript minute markers on intermediate cells (`⁰⁰` for `:00`, `¹⁵` for `:15`,
  `³⁰` for `:30`, `⁴⁵` for `:45`); whole-hour 24h zones never reach the `:00` intermediate path so `⁰⁰` only
  appears for fractional zones; `format_hour_offset` extended for minute granularity; the hour-offset indicator alignment uses the
  new cell math.
- `src/tui/copy.rs` — copy text uses the selected sub-hour time (no separate flag needed; the selection already reflects
  the cell).
- Footer (`render_footer`) — add the right-aligned `i 60│30│15` switcher next to `Shade` and the format switcher.
- `src/config.rs` — add an `interval` field on `AppConfig` (with a serde default of 60) and a small enum/validator so
  unknown values fall back to the default on load.
- No new dependencies.
