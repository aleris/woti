### Requirement: Interval CLI flag accepts 60, 30, or 15 minutes
The CLI SHALL accept an optional `--interval <minutes>` flag whose value is one of `60`, `30`, or `15`. The flag SHALL only be accepted when no subcommand is given (TUI launch path). Any other value SHALL cause the process to exit with a non-zero status and an error message indicating the accepted set. When the flag is omitted, the active interval SHALL default to 60 minutes (current behavior).

#### Scenario: Default interval is 60 minutes when nothing is configured
- **WHEN** the user runs `woti` with no `--interval` flag and no persisted `config.interval`
- **THEN** the TUI launches with the navigation interval set to 60 minutes and renders identically to today (no tick glyphs, no minute markers)

#### Scenario: Valid 30-minute interval
- **WHEN** the user runs `woti --interval 30`
- **THEN** the TUI launches with the navigation interval set to 30 minutes

#### Scenario: Valid 15-minute interval
- **WHEN** the user runs `woti --interval 15`
- **THEN** the TUI launches with the navigation interval set to 15 minutes

#### Scenario: Invalid interval value
- **WHEN** the user runs `woti --interval 5`
- **THEN** the process exits with a non-zero status and prints an error listing the accepted values (60, 30, 15)

#### Scenario: Interval flag rejected with subcommand
- **WHEN** the user runs `woti add PST --interval 30`
- **THEN** the CLI rejects the invocation with an error (the flag is only valid for the TUI launch path)

### Requirement: TUI shortcut `i` cycles the active interval
The TUI SHALL respond to the `i` key by cycling the active interval through the sequence `60 → 30 → 15 → 60`. The cycle SHALL re-render immediately on each press. The selected absolute time SHALL be preserved across cycles by adjusting the cell offset to the nearest cell of the new interval.

#### Scenario: Cycle from 60 to 30
- **WHEN** the active interval is 60 minutes and the user presses `i`
- **THEN** the active interval becomes 30 minutes and the timeline re-renders with intermediate ticks

#### Scenario: Cycle from 30 to 15
- **WHEN** the active interval is 30 minutes and the user presses `i`
- **THEN** the active interval becomes 15 minutes and the timeline re-renders with three intermediate ticks per hour

#### Scenario: Cycle wraps from 15 to 60
- **WHEN** the active interval is 15 minutes and the user presses `i`
- **THEN** the active interval becomes 60 minutes and the timeline re-renders without intermediate ticks

#### Scenario: Selected time is preserved when narrowing the interval
- **WHEN** the active interval is 60 minutes with `cell_offset = 1` (selection is "+1 hour")
- **AND** the user presses `i` to switch to 30 minutes
- **THEN** the selection still represents "+1 hour" (`cell_offset` is rescaled to 2 in 30-minute units)

#### Scenario: Selected time snaps when widening the interval
- **WHEN** the active interval is 15 minutes with the selection at "+25 minutes" (`cell_offset = 5/15`-mode is between 1 and 2 hours of 15-min cells)
- **AND** the user presses `i` to switch to 60 minutes
- **THEN** the selection snaps to the nearest hour (`cell_offset = 0`)

### Requirement: `i` shortcut persists the active interval to config
Pressing `i` SHALL update `config.interval` to the new value and persist the configuration to disk on every cycle, matching how the `f` (time format) and `w` (shading) shortcuts persist their toggles. Persistence failures SHALL be ignored silently (consistent with the existing toggles), so the in-memory state still reflects the user's choice.

#### Scenario: `i` cycles persist across launches
- **WHEN** the user launches `woti`, presses `i` to switch to 30 minutes, then quits and re-launches `woti`
- **THEN** the new session starts at 30 minutes

#### Scenario: Multiple cycles are persisted
- **WHEN** the user presses `i` three times (60 → 30 → 15 → 60), then quits and re-launches `woti`
- **THEN** the new session starts at 60 minutes (the last cycled value)

### Requirement: `--interval` flag is a session override and does not persist
The `--interval` flag SHALL set the active interval for the current session only. It SHALL NOT write the value back to `config.toml`; the previously persisted value (if any) SHALL be preserved on disk for the next non-flag launch. This mirrors `--date` and `--time`, which never mutate persisted config.

#### Scenario: Flag overrides persisted value for one session
- **WHEN** the persisted `config.interval` is 30 and the user runs `woti --interval 15`
- **THEN** the session uses 15 minutes
- **AND** the persisted `config.interval` is still 30 after the session ends

#### Scenario: `i` press during a flag-overridden session does persist
- **WHEN** the user runs `woti --interval 15` and then presses `i` to switch to 60 minutes
- **THEN** `config.interval` is updated to 60 on disk (the `i` handler always persists)

### Requirement: Launch resolution order for the active interval
On launch the active interval SHALL be resolved in the following order: (1) the `--interval` flag value if provided, (2) the persisted `config.interval` value if present and valid (one of 60/30/15), (3) the default value `60`. An unknown or invalid persisted value SHALL fall back to `60` rather than aborting startup.

#### Scenario: Default with no flag and no persisted value
- **WHEN** the user runs `woti` with no `--interval` flag and a `config.toml` that omits the `interval` field
- **THEN** the session starts at 60 minutes

#### Scenario: Persisted value is used when flag is absent
- **WHEN** the user runs `woti` with no `--interval` flag and a `config.toml` containing `interval = 15`
- **THEN** the session starts at 15 minutes

#### Scenario: Flag wins over persisted value
- **WHEN** the user runs `woti --interval 30` and `config.toml` contains `interval = 15`
- **THEN** the session starts at 30 minutes (and 15 remains on disk)

#### Scenario: Invalid persisted value falls back to default
- **WHEN** `config.toml` contains `interval = 7` (not in the supported set)
- **AND** the user runs `woti` with no flag
- **THEN** the session starts at 60 minutes and startup succeeds

### Requirement: Footer renders an interval switcher right-aligned
The TUI footer SHALL render an `i` shortcut group with three options `60│30│15` in the right-aligned switcher region, positioned to the left of `Shade` (`w`) and the format switcher (`f`). The active interval SHALL use the active style (`SWITCHER_ACTIVE_FG` / `SWITCHER_ACTIVE_BG`, bold); inactive options SHALL use the dim style; option separators SHALL use the `│` separator glyph with the existing separator style.

#### Scenario: 60-minute interval is highlighted
- **WHEN** the active interval is 60 minutes
- **THEN** the footer shows `i  60│30│15` with `60` rendered in the active style and `30`/`15` dimmed

#### Scenario: 30-minute interval is highlighted
- **WHEN** the active interval is 30 minutes
- **THEN** the footer shows `i  60│30│15` with `30` rendered in the active style and `60`/`15` dimmed

#### Scenario: 15-minute interval is highlighted
- **WHEN** the active interval is 15 minutes
- **THEN** the footer shows `i  60│30│15` with `15` rendered in the active style and `60`/`30` dimmed

#### Scenario: Switcher order in the footer
- **WHEN** the footer is rendered with all three switchers visible
- **THEN** they appear in the order `i 60│30│15`, then `w Shade`, then `f mx│am│24`, all right-aligned

### Requirement: 60-minute interval is byte-for-byte identical to today
When the active interval is 60 minutes, the rendered output SHALL be identical to the pre-change rendering: same number of cells per row, same hour digits, same sub-row content (am/pm/blank/`³⁰`/`⁴⁵`/`½a`/`¾p`), and no `·` glyph or superscript minute marker SHALL appear anywhere in the timeline. The hour-offset indicator SHALL use the legacy "in N hour(s)" / "N hour(s) ago" formatting (no minutes component).

#### Scenario: 60-minute interval matches pre-change UTC row
- **WHEN** the active interval is 60 minutes and a UTC row is rendered in 24h mode
- **THEN** the timeline shows hour digits in every cell with blank (`"  "`) sub-row cells (no `·`, no `⁰⁰`)

#### Scenario: 60-minute interval matches pre-change India row
- **WHEN** the active interval is 60 minutes and an India (`+5:30`) row is rendered in 12h mode
- **THEN** every cell renders an hour digit and the sub-row reads `½a`/`½p` per cell (no `·`, no superscript minutes)

### Requirement: Each timeline cell represents the active interval
With the active interval set to N minutes, every visible timeline cell SHALL represent N minutes of wall-clock time in the cell's timezone. Adjacent cells SHALL be exactly N minutes apart. The total visible time span SHALL be `num_cells * N` minutes.

#### Scenario: 60-minute cells (default)
- **WHEN** the active interval is 60 minutes
- **THEN** adjacent cells differ by 1 hour (current behavior)

#### Scenario: 30-minute cells
- **WHEN** the active interval is 30 minutes
- **THEN** adjacent cells differ by 30 minutes

#### Scenario: 15-minute cells
- **WHEN** the active interval is 15 minutes
- **THEN** adjacent cells differ by 15 minutes

### Requirement: Hour row renders a tick on intermediate cells
With a sub-hour active interval, the hour row SHALL render the hour number (`%2d` for 24h, `%I` for 12h) on exactly one cell per wall hour per timezone, with all other cells rendering the tick glyph `·` right-aligned in the 2-character content slot. The hour cell is the cell whose `[start, start + interval_minutes)` window contains the timezone's natural hour boundary (`offset_m`), formally `dt.minute() == floor(offset_m / interval_minutes) * interval_minutes`. With the 60-minute interval, the hour row is unchanged (every cell is an hour cell, no `·` ever appears).

This rule guarantees one hour digit per hour even when the interval does not divide `offset_m` evenly (e.g. Nepal `+5:45` at 30-minute interval: cells land on `:00`/`:30`, the natural `:45` boundary falls inside the `[:30, :00)` window, so the `:30` cell becomes the hour cell and stays column-aligned with whole-hour zones).

The number of intermediate (`·`) cells inserted between consecutive hour cells is exactly `(60 / interval_minutes) - 1`:
- 60 min → 0 intermediates (legacy)
- 30 min → 1 intermediate
- 15 min → 3 intermediates

#### Scenario: 15-minute interval, whole-hour zone, hour row
- **WHEN** the active interval is 15 minutes for a UTC zone (`offset_m = 0`)
- **AND** four consecutive cells start at hour `H`
- **THEN** the hour row shows `H · · · H+1` (3 intermediates between `H` and `H+1`)

#### Scenario: 30-minute interval, whole-hour zone, hour row
- **WHEN** the active interval is 30 minutes for a UTC zone
- **AND** two consecutive cells start at hour `H`
- **THEN** the hour row shows `H · H+1` (1 intermediate between `H` and `H+1`)

#### Scenario: 15-minute interval, half-hour zone, hour row
- **WHEN** the active interval is 15 minutes for India (`+5:30`, `offset_m = 30`)
- **THEN** hour cells are at wall-clock minutes `:30` and intermediate cells (at `:45`, `:00`, `:15`) render `·`

#### Scenario: 30-minute interval, quarter-hour zone, hour row
- **WHEN** the active interval is 30 minutes for Nepal (`+5:45`, `offset_m = 45`)
- **THEN** hour cells are at wall-clock minute `:30` (the cell whose window `[:30, :00 next)` contains the natural `:45` boundary) and intermediate cells at `:00` render `·` — the row still shows one hour digit per hour, column-aligned with whole-hour zones

#### Scenario: 60-minute interval keeps the legacy hour row
- **WHEN** the active interval is 60 minutes
- **THEN** every cell renders an hour digit (no `·` glyph ever appears)

### Requirement: Sub row renders superscript minute markers on intermediate cells
With a sub-hour active interval, intermediate cells in the sub row SHALL render the wall-clock minute of the cell's datetime as Unicode superscript digits: `00 → "⁰⁰"`, `15 → "¹⁵"`, `30 → "³⁰"`, `45 → "⁴⁵"`. The `⁰⁰` glyph only ever fires on intermediate cells of fractional-offset zones (Nepal `+5:45`, India `+5:30`, etc.) where wall `:00` lands between two hour cells — the hour row above only shows a `·` tick there, so the sub row needs `⁰⁰` to mark the natural hour boundary. For whole-hour zones the wall `:00` minute is the hour cell itself (handled separately and kept blank in 24h, `am`/`pm` in 12h), so `⁰⁰` is unreachable for them. Each marker occupies exactly 2 terminal columns, fitting the existing 2-character cell content slot. With the 60-minute interval, no minute markers are rendered (legacy behavior).

#### Scenario: 15-minute interval intermediate markers (whole-hour zone)
- **WHEN** the active interval is 15 minutes for UTC and four cells span one hour starting at `:00`
- **THEN** the intermediate cells (at `:15`, `:30`, `:45`) show `¹⁵`, `³⁰`, `⁴⁵` respectively in the sub row

#### Scenario: 30-minute interval intermediate marker (whole-hour zone)
- **WHEN** the active interval is 30 minutes for UTC and two cells span one hour
- **THEN** the intermediate cell (at `:30`) shows `³⁰` in the sub row

### Requirement: Hour-cell sub row content is preserved across intervals
With a sub-hour active interval, the sub-row content of an hour cell SHALL follow the existing rules from `half-hour-timeline-indicator` for that timezone and time format. Whole-hour-offset timezones in 24h mode keep the legacy blank (`"  "`) hour-cell sub-row in every interval — the hour digit on the row above already marks the slot, so leaving the sub-row blank gives stronger visual contrast with the intermediate `¹⁵`/`³⁰`/`⁴⁵` markers. Half- and quarter-hour zones keep their existing fractional glyphs (`³⁰`, `⁴⁵`, `½a`, `¾p`) on hour cells.

#### Scenario: Whole-hour zone, 24h mode, sub-hour interval
- **WHEN** the active interval is 15 minutes for UTC in 24h mode
- **THEN** hour-cell sub rows render `"  "` (blank — the row-above hour digit marks the slot)

#### Scenario: Whole-hour zone, 12h mode, sub-hour interval
- **WHEN** the active interval is 15 minutes for San Jose (PDT) in 12h mode
- **THEN** hour-cell sub rows render `am` or `pm` (unchanged)

#### Scenario: Half-hour zone, 24h mode, sub-hour interval
- **WHEN** the active interval is 15 minutes for India (`offset_m = 30`) in 24h mode
- **THEN** hour-cell sub rows render `³⁰` (unchanged)

#### Scenario: Quarter-hour zone, 24h mode, sub-hour interval
- **WHEN** the active interval is 15 minutes for Nepal (`offset_m = 45`) in 24h mode
- **THEN** hour-cell sub rows render `⁴⁵` (unchanged) and intermediate cells render `⁰⁰`, `¹⁵`, `³⁰` (the `⁰⁰` marks the natural hour boundary the `·` tick above does not)

#### Scenario: Quarter-hour zone, 12h mode, sub-hour interval
- **WHEN** the active interval is 15 minutes for India in 12h mode
- **THEN** hour-cell sub rows render `½a`/`½p` (unchanged) and intermediate cells render superscript wall-clock minutes (e.g. `⁴⁵ ⁰⁰ ¹⁵`)

#### Scenario: Whole-hour zone, 24h mode, 60-minute interval (legacy)
- **WHEN** the active interval is 60 minutes for UTC in 24h mode
- **THEN** hour-cell sub rows render `"  "` (unchanged)

### Requirement: Selection step matches the active interval
A single Left or Right arrow key press SHALL move the selection by exactly one cell, which equals N minutes when the active interval is N. The selected cell's content (info-column time string, copy text) SHALL reflect the corresponding sub-hour wall-clock time.

#### Scenario: Single right press at 15-minute interval
- **WHEN** the active interval is 15 minutes and `cell_offset = 0`
- **AND** the user presses Right once
- **THEN** `cell_offset` becomes 1 and the displayed time shifts forward by 15 minutes in every timezone

#### Scenario: Single left press at 30-minute interval
- **WHEN** the active interval is 30 minutes and `cell_offset = 0`
- **AND** the user presses Left once
- **THEN** `cell_offset` becomes -1 and the displayed time shifts backward by 30 minutes

#### Scenario: Info-column time reflects sub-hour selection
- **WHEN** the active interval is 15 minutes, the local timezone is UTC, and the user navigates to `cell_offset = 1` while "now" is 10:00
- **THEN** the info-column time string for UTC reads `10:15` (24h) or `10:15 AM` (12h)

### Requirement: Cycle preserves selected absolute time when interval narrows
When the user presses `i` to cycle from interval `A` to interval `B` where `B < A`, the system SHALL multiply `cell_offset` by `A/B` so the absolute selected time is exactly preserved (since `A` is always a multiple of `B` in the supported set).

#### Scenario: 60 → 30 doubles the cell offset
- **WHEN** the active interval is 60 minutes with `cell_offset = 2` (selection is "+2 hours")
- **AND** the user presses `i` to switch to 30 minutes
- **THEN** `cell_offset` becomes 4 (still "+2 hours" in 30-minute cells)

#### Scenario: 30 → 15 doubles the cell offset
- **WHEN** the active interval is 30 minutes with `cell_offset = 3` (selection is "+1h30m")
- **AND** the user presses `i` to switch to 15 minutes
- **THEN** `cell_offset` becomes 6 (still "+1h30m" in 15-minute cells)

### Requirement: Cycle snaps selected time when interval widens
When the user presses `i` to cycle from interval `A` to interval `B` where `B > A`, the system SHALL set `cell_offset` to the nearest integer in the new interval such that the absolute selected time is rounded to the nearest multiple of `B` minutes. Half-cell ties round to the cell with larger absolute offset (away from zero).

#### Scenario: 15 → 60 snaps to nearest hour (down)
- **WHEN** the active interval is 15 minutes with `cell_offset = 1` (selection is "+15 minutes")
- **AND** the user presses `i` (15 → 60)
- **THEN** `cell_offset` becomes 0 (nearest hour is "now")

#### Scenario: 15 → 60 snaps to nearest hour (up)
- **WHEN** the active interval is 15 minutes with `cell_offset = 5` (selection is "+1h15m")
- **AND** the user presses `i` (15 → 60)
- **THEN** `cell_offset` becomes 1 (nearest hour is "+1 hour")

### Requirement: Copy text reflects sub-hour selection
The clipboard text produced by pressing `c` SHALL show the selected sub-hour wall-clock time in each timezone (with the existing time-format rules), not a rounded hour.

#### Scenario: Copy at 15-minute selection
- **WHEN** the active interval is 15 minutes and the user has navigated to a `:30` cell, then presses `c`
- **THEN** the copied text contains times ending in `:30` for whole-hour timezones and the corresponding sub-hour times for fractional zones

### Requirement: Info column time preserves the actual wall-clock minute
The per-zone info column (left side of each timezone block) SHALL display the actual wall-clock minute of the selected datetime, not the floored cell-anchor minute. This means at sub-hour intervals the info column behaves analogously to the 60-minute path: when `cell_offset == 0` it shows the live "now" minute (e.g. `15:37`, not `15:30`), and as the user navigates the original minute is carried through (`15:37 → 16:37 → 17:37` regardless of interval). The timeline cells themselves SHALL stay aligned to the interval grid (floored anchor) so hour digits column-align across zones.

#### Scenario: Live info time at M30 with `cell_offset = 0`
- **WHEN** the wall-clock time is `15:37` and the active interval is 30 minutes
- **THEN** the info column reads `15:37` (not `15:30`), while the timeline hour cell for that hour is still anchored at the `15:30` cell

#### Scenario: Navigation preserves the original minute at M15
- **WHEN** the wall-clock time is `15:37`, the active interval is 15 minutes, and the user navigates `+4` cells
- **THEN** the info column reads `16:37` (not `16:30`)
