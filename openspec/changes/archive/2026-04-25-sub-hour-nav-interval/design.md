## Context

The TUI lays out each timezone in a 3-row block (`BLOCK_HEIGHT = 3`) with the timeline using a fixed `CELL_WIDTH = 3` columns per cell (1 separator space + 2 content chars). The hour row prints zero-padded hour numbers (`%2d` / `%I`), and the sub-row prints `am`/`pm` (12h), blank `"  "` (24h), or fractional glyphs (`³⁰`, `⁴⁵`, `½a`, etc.) for half-hour timezones (see `half-hour-timeline-indicator`).

Navigation is purely hour-based today: `App.hour_offset: i32` is incremented/decremented by Left/Right; each render computes `start_hour = base_hour - num_cells / 2` and walks one **hour** per cell. Acceleration (`accelerated-nav`) multiplies the per-press step but the unit is always hours.

The footer renders a left-aligned shortcut list and a right-aligned switcher group: `Shade` (`w`) and the format switcher (`f mx│am│24`). Adding a third right-aligned switcher needs to fit in the same line and follow the same active/dim/separator styling.

The user's reference example in `view.md` makes the desired pixel-level rendering unambiguous, so the design here focuses on the model changes that make that rendering fall out naturally.

## Goals / Non-Goals

**Goals:**
- One mechanism (interval-in-minutes) drives both the time-per-cell, the rendered cell density, and the navigation step. Selection always lands on a multiple of the interval.
- Cell width (3 cols / 2-char content) and row count are unchanged; the change is content + cell-count, not layout.
- Default behavior (60 min) is **byte-for-byte identical to today**: same number of cells visible per row, no `·` glyphs, no superscript minutes, no `⁰⁰` filler — every renderer code path with `interval == 60` MUST produce the exact same output as before this change.
- Sub-hour modes insert intermediate cells between hour cells: 1 intermediate per hour at 30-min, 3 intermediates per hour at 15-min, matching `view.md`.
- The CLI flag and TUI shortcut converge on the same `App` field.
- Sub-hour intermediate cells are visually obvious (`·` + small minute labels) but never compete with hour cells for attention.
- The hour-offset indicator scales gracefully (e.g. "in 30 minutes", "1 hour 15 minutes ago").

**Non-Goals:**
- Persisting the chosen interval to `config.toml` (it is session-only, like the in-app format toggle isn't written by `i`).
- Supporting arbitrary intervals (only 60, 30, 15). 5-min and 1-min are intentionally out of scope.
- Changing the cell width or block height — no "expanded" mode.
- Updating the `print` subcommand output to support intervals (left for a future change; `print` continues to emit hourly rows).
- Sub-hour-aware DST transition rendering beyond what the current code already handles via `chrono`.

## Decisions

### D1: Interval is an enum with explicit minute values

Define `NavInterval { H1, M30, M15 }` in `src/tui/mod.rs` with a `pub fn minutes(self) -> u32` method returning `60 / 30 / 15`, and a `pub fn next(self) -> Self` for the `i`-key cycle (`H1 → M30 → M15 → H1`). The CLI parses the flag value with `clap`'s `value_parser` restricted to `[60, 30, 15]` and maps it to the enum.

**Why an enum over a raw `u32`:** It bounds the valid set at the type level, makes the cycle order explicit, and avoids invalid values (e.g. 7) ever reaching the renderer. The `minutes()` accessor keeps arithmetic readable.

**Alternative considered:** A free `u32` with runtime guards. Rejected because every renderer/event path would re-validate.

### D2: Selection offset is stored as cells, not minutes

Rename `App.hour_offset: i32` → `App.cell_offset: i32`. The minute-offset of the selection from "now" is always `cell_offset * interval.minutes()`. The hour-aligned arithmetic in `render_timezone_block` (`base_hour = current_hour + hour_offset`) becomes a minute-aware computation:

```
base_dt = now_tz + Duration::minutes((cell_offset * interval.minutes() as i32) as i64)
```

Each timeline cell `i` represents `start_dt + i * interval` rather than `start_hour + i` hours. Hour cells are detected as `dt.minute() == offset_m % 60` (i.e. the timezone's own "top of the hour"); intermediate cells are everything else.

**Why store cells, not minutes:** Keeps acceleration simple — `compute_step` continues to return cells, multiplied by the per-cell minute width. Switching interval mid-session does not "lose" the user's position too badly: when the interval narrows from 60 → 15, `cell_offset` is multiplied by 4 so the absolute selected time is preserved. When it widens, `cell_offset` is divided (rounding to the nearest hour). This conversion happens in the `i`-key handler.

**Alternative considered:** Store `minute_offset: i32` directly. Rejected because every render call would have to divide-by-interval to find the cell index, and acceleration math gets messier.

### D3: Cell content is decided per-cell using `dt.minute()`

`build_hour_spans` and `build_ampm_spans` already iterate over `0..num_cells` and compute a `dt` per cell. They keep doing exactly that, just stepping by `interval.minutes()`. The cell count `num_cells` is computed from the available width exactly as today (`timeline_avail / CELL_WIDTH`), so a sub-hour mode in the same terminal width yields the same number of cells but a smaller visible time span.

A cell is an **hour cell** iff `dt.minute() == p.offset_m % 60`. Otherwise it is an **intermediate cell**.

- **Hour row:**
  - hour cell → render the hour digit (existing behavior).
  - intermediate cell → render `" ·"` (right-aligned dot in the 2-char content slot, consistent with how `format!("{:>2}", ...)` aligns hour numbers).
- **Sub row (24h, whole-hour zone):**
  - hour cell: `"  "` (blank, all intervals — the hour digit on the row above marks the slot).
  - intermediate cell: superscript minutes (`"¹⁵"`, `"³⁰"`, `"⁴⁵"`). Note: whole-hour zones never have an
    intermediate at wall `:00` (that minute is always the hour cell), so `⁰⁰` is unreachable here.
- **Sub row (24h, half/quarter zone):**
  - hour cell: keep `"³⁰"` / `"⁴⁵"` (existing).
  - intermediate cell: superscript minutes for the wall-clock minute of `dt`.
- **Sub row (12h, whole-hour zone):**
  - hour cell: `"am"` / `"pm"` (existing).
  - intermediate cell: superscript minutes.
- **Sub row (12h, half/quarter zone):**
  - hour cell: `"½a"` / `"¾p"` (existing).
  - intermediate cell: superscript minutes.

When `interval == 60` the iteration produces only hour cells (because every cell is at the timezone's top-of-hour by construction), so no `·` glyphs and no superscript minutes are ever emitted — the legacy output is reproduced exactly.

This means the existing styling cascade (selected/local/default) applies uniformly because we still emit one styled span per cell — only the text content depends on `dt.minute()`.

**Mapping minutes → superscript:** `00 → "⁰⁰"`, `15 → "¹⁵"`, `30 → "³⁰"`, `45 → "⁴⁵"`, with a fall-through to `"  "` for safety. The `0 → "⁰⁰"` case only ever fires on intermediate cells of fractional-offset zones (Nepal `+5:45`, India `+5:30`, etc.) where wall `:00` lands between two hour cells — the row above only shows a `·` tick there, so the sub-row needs `⁰⁰` to mark the natural hour boundary. Whole-hour zones reach this minute via the hour-cell branch (kept blank in 24h, `am`/`pm` in 12h), so `⁰⁰` is unreachable for them. Two-character superscript glyphs all measure 1 terminal column wide (Unicode East Asian Width = Narrow), so they fit the 2-char content slot exactly.

### D4: Hour detection works for half- and quarter-hour zones

For Nepal (`+5:45`, `offset_m = 45`), the "hour cells" are at wall-clock minutes `:45`. So a cell is an "hour cell" iff `dt.minute() % 60 == p.offset_m`. With this rule, the example in `view.md` falls out: Nepal's Sub row reads `⁴⁵ ⁰⁰ ¹⁵ ³⁰ ⁴⁵` because the first/last cells are hour cells (kept as `⁴⁵`) and the three middle cells are sub-hour ticks at wall-clock `:00`, `:15`, `:30`.

For India (`+5:30`, `offset_m = 30`) in 12h mode the hour cells stay `½a`/`½p` and intermediate cells (`+15` and `+30` from the hour cell, i.e. wall-clock `:45` and `:00`) show `⁴⁵` / `⁰⁰`. This gives `½a ⁴⁵ ⁰⁰ ¹⁵ ½a` for a 4-cells-per-hour run, matching the `view.md` example.

### D4b: Info column time preserves the actual minute (cell anchor vs display dt)

The render pipeline computes two related datetimes per zone block:

```
cell_anchor_dt = floor_to_interval(now_tz, interval) + cell_offset * interval
display_dt     = now_tz                              + cell_offset * interval
```

- `cell_anchor_dt` feeds `TimelineParams.base_dt` so the cells stay snapped
  to the interval grid. This is what makes hour digits column-align across
  zones and what makes `is_hour_cell` work.
- `display_dt` feeds the per-zone info column (`tz_abbr`, `time_str`,
  `date_str`, and `build_info_line`'s `selected_dt`) so the info reads the
  user's actual wall-clock minute rather than the floored cell boundary.

At H1 the two collapse to the same value (because `floor_to_interval` is a
no-op for `interval >= 60`), which is why the legacy 60-minute path already
shows `15:37` correctly today. For sub-hour intervals the split is what
preserves that property: when `cell_offset == 0` the info reads `15:37`
(not `15:30`/`15:15`), and as the user navigates the original minute is
carried through (`15:37 → 16:37 → 17:37` regardless of interval).

### D5: Acceleration unchanged in shape, scaled in unit

`compute_step` still returns "1 + ramp" cells per event. The Left/Right handler increments `pending_h_offset` (rename to `pending_cell_offset`) by `step` cells; on flush it adds to `cell_offset`. The minutes-per-step is implicit in the cell→minute conversion at render time. This means with `--interval 15` and full acceleration, the user moves 8 cells/event = 2 hours/event, comparable to today's 8 hours/event at hourly granularity — which is the expected "same physical key-hold feel, finer step" semantics.

### D6: Footer switcher styling matches existing groups

Add a third right-aligned switcher block before `Shade`:

```
 i  60│30│15  w  Shade  f  mx│am│24
```

Each option uses `sel`/`dim` styles with `│` `sep` separators, identical to the format switcher. Width math (`right_w` in `render_footer`) just sums the new spans. When the terminal is too narrow for all three groups, the existing `spacer = (width - left - right).max(0)` logic naturally collapses gaps; we do not add new responsive trimming.

### D7: `i` key handler converts `cell_offset` to preserve selected absolute time

When the user presses `i`, compute the selected absolute datetime from the *current* (interval, cell_offset) pair, switch to the next interval, then snap `cell_offset` to the nearest cell of the new interval:

```
absolute_minutes = cell_offset * old_interval.minutes() as i32
new_cell_offset  = round(absolute_minutes / new_interval.minutes() as f32)
```

Going 60 → 30 → 15: an exact integer multiplication, never lossy. Going 15 → 60: rounds e.g. `+25 min` (cell_offset = 5/15-mode) to nearest hour `+0`. This is the behavior users expect when widening a viewport.

### D8: Hour-offset indicator gets a minute-aware formatter

`format_hour_offset(offset: i32) -> String` becomes `format_offset(minutes: i32) -> String`:

- `0` → `""`
- pure hour multiples → "in N hour(s)" / "N hour(s) ago" (existing format preserved).
- pure sub-hour → "in N minutes" / "N minutes ago".
- mixed → "in 1 hour 15 minutes" / "2 hours 30 minutes ago".

Singular/plural rules apply to both units. Alignment in `render_body` uses `selected_cell` (cell index of selection in the visible row), unchanged in formula.

### D9: CLI flag and validation

```rust
#[arg(long, value_parser = ["60", "30", "15"])]
pub interval: Option<String>,
```

Optional (no `default_value`) so that "flag absent" is distinguishable from "flag present with the default". Parsed in `main.rs` to `Option<NavInterval>` (or rejected with a clear error before invoking `App::new`). Combining `--interval` with any subcommand is rejected the same way `--date`/`--time` are today (existing post-parse guard in `main`).

### D11: Persistence parity with the other right-aligned switchers

The active interval is persisted to `config.toml` (new field `interval`), matching how `f` (time format) and `w` (shading) already write back via `config.save()`. The `i`-key handler writes after each cycle. The CLI flag is a session override only:

- **Launch resolution** (in `main.rs` / `App::new`):
  1. If `--interval` is provided on the CLI, use it for this session and do **not** write it back to disk.
  2. Otherwise read `config.interval` (defaulting to `60` via serde when the field is absent or unknown).
- **`i`-key handler**: cycle to the next interval, write it to `self.config.interval`, then call `self.config.save()` (ignoring the `Result`, like the existing toggles).

`AppConfig` gains:

```rust
#[serde(default = "default_interval", deserialize_with = "deserialize_interval")]
pub interval: u32,  // always one of 60, 30, 15
```

with a custom deserializer that falls back to `60` if the file contains an unrecognized value (so corrupt configs do not break startup).

**Why parity matters:** Users who switch to a sub-hour view rarely want to "un-do" that on every launch — they reach for `i` because that workflow is now their default. Treating `i` differently from `f`/`w` would surprise users.

**Why the flag does not persist:** `--interval` is for ad-hoc sessions (a meeting check, a quick demo) just like `--date`/`--time`. Writing it back would silently mutate the user's persisted preference. This matches our existing CLI-vs-TUI split.

### D10: Copy text follows the selection naturally

`build_copy_text` already takes the reference UTC and `hour_offset` (whole-hour shift) and adds it to derive the selected timestamp. We change the signature to take `selected_offset_minutes: i32` instead, computed as `cell_offset * interval.minutes()`. No format change inside the copy text — the existing time formatter prints minutes anyway.

## Risks / Trade-offs

- **[Hour-cell detection ambiguity for `:00` half-hour zones]** A timezone with `offset_m = 0` and `interval = 30` correctly treats `dt.minute() == 0` as an hour cell. Verified by the `view.md` example. → No mitigation needed.
- **[Renaming `hour_offset` ripples through copy/test code]** → Mitigation: keep the field name `hour_offset` if churn is high, but document it as "cell_offset"; the spec is what matters. Decision deferred to implementation.
- **[`print` subcommand intentionally out of scope]** Users may expect `woti print --interval 15` to enumerate sub-hour rows. → Document in the proposal as non-goal; revisit in a follow-up change if requested.
- **[Indicator string can grow to ~22 chars]** ("in 23 hours 45 minutes"). Existing alignment math centers the label under the selected cell with no truncation, so it can run off the right edge in narrow terminals. → Acceptable: the user is deliberately exploring a far offset; the cell highlight is the primary anchor.
- **[Footer overflow on narrow terminals]** Adding a third switcher (~12 chars) eats spacer first, then visually crowds shortcuts. → Acceptable for v1; if reports come in we can elide the cycle separators (`60/30/15` instead of `60│30│15`) to save 4 cols.
- **[Glyph rendering for `·` (U+00B7) and superscript digits]** All targeted glyphs are in standard monospace fonts; the existing half-hour indicator has shipped `³⁰`/`⁴⁵`/`½` without reports. → Reuse the same glyph set, no additional risk.

## Migration Plan

No migration needed: the default `--interval 60` and the default `App.interval = NavInterval::H1` reproduce today's exact rendering. Existing config files are untouched — the interval is not persisted. Tests that assert on hourly rendering remain valid because they default to `H1`.

## Open Questions

- Should the `i`-key cycle direction be reversible (e.g. Shift+`i` cycles `H1 ← M30 ← M15`)? Out of scope for this change; can be added later if needed.
- Should the `--interval` flag also affect the `print` subcommand? Out of scope (see Non-Goals); revisit when there's a use case.
