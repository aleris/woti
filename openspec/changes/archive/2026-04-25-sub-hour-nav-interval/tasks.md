## 1. Config plumbing

- [x] 1.1 Add a `NavInterval` enum (variants `H1`, `M30`, `M15`) in `src/tui/mod.rs` with `pub fn minutes(self) -> u32` returning `60 / 30 / 15` and `pub fn next(self) -> Self` cycling `H1 → M30 → M15 → H1`. Re-export from `mod.rs`.
- [x] 1.2 Add an `interval: u32` field to `AppConfig` in `src/config.rs` with a serde default of `60` and a custom deserializer that falls back to `60` when the file value is not in `{60, 30, 15}` (so corrupt configs do not abort startup).
- [x] 1.3 Add a unit test in `src/config.rs` covering: default-when-absent, accepted values round-trip, invalid value falls back to 60, persisted value round-trips.

## 2. CLI flag

- [x] 2.1 Add `--interval <60|30|15>` as `Option<String>` on `Cli` in `src/cli.rs`, using `clap`'s `value_parser = ["60", "30", "15"]` so invalid values are rejected with a clean clap error.
- [x] 2.2 Update the post-parse guard in `src/main.rs` so `--interval` combined with any subcommand exits 2 with a clear message (extend the existing `--date` / `--time` check).
- [x] 2.3 In `cmd_tui` (or its caller), resolve the active interval as: flag value → `config.interval` → `60`. Pass the resolved `NavInterval` into `App::new`.
- [x] 2.4 Update `src/cli.rs` `after_help` examples to mention `woti --interval 30` and `woti --interval 15`.

## 3. App state and `i` shortcut

- [x] 3.1 Add `interval: NavInterval` to `App` in `src/tui/app.rs`; update `App::new` to accept it.
- [x] 3.2 Rename `App.hour_offset: i32` → `App.cell_offset: i32` (or keep the name but document it as cells). Update all internal references in `src/tui/{app,event,render,copy}.rs`.
- [x] 3.3 Implement `App::cycle_interval(&mut self)` in `src/tui/nav_interval.rs` (mirrors the `time_format.rs` pattern): rescale `cell_offset` to preserve the absolute selected time across the new interval (multiply when narrowing, round-half-away-from-zero when widening), update `self.interval`, mirror the new value into `self.config.interval`, and call `self.config.save()` (ignore the `Result`, matching `cycle_time_format`).
- [x] 3.4 Wire `(KeyCode::Char('i'), KeyModifiers::NONE)` in `src/tui/event.rs` to call `cycle_interval`, reset `nav_start`/`nav_dir`, redraw, and update `last_render` (mirror the `f`/`w` handlers exactly).
- [x] 3.5 Add unit tests for `cycle_interval` covering: 60→30 doubles `cell_offset`, 30→15 doubles, 15→60 rounds-to-nearest-hour (down for `+15`, up for `+45`), wrap order is `H1→M30→M15→H1`.

## 4. Rendering — hour and sub rows

- [x] 4.1 Add `interval_minutes: u32` to `TimelineParams` in `src/tui/render.rs` and populate it from `App.interval.minutes()` in `render_timezone_block`.
- [x] 4.2 In `render_timezone_block`, compute the per-cell stride in minutes (`interval_minutes`), the visible-window start as `base_dt - (num_cells / 2) * interval_minutes`, and step `dt` per cell by `interval_minutes` instead of by `1 hour`. The legacy "every cell is an hour" arithmetic is the special case where `interval_minutes == 60`.
- [x] 4.3 Add a helper `is_hour_cell(dt, offset_m) -> bool` returning `dt.minute() == offset_m % 60`.
- [x] 4.4 In `build_hour_spans`, render the hour digit for hour cells (existing path) and `" ·"` (right-aligned in the 2-char content slot) for intermediate cells. The selected/local/default style cascade applies to both branches unchanged.
- [x] 4.5 Add a helper `minutes_superscript_full(m: u32) -> &'static str` returning `"⁰⁰" / "¹⁵" / "³⁰" / "⁴⁵"` for `0/15/30/45` (and `"  "` fallback). The `⁰⁰` only ever fires on intermediate cells of fractional zones (whole-hour zones reach `:00` via the hour-cell branch which is blank in 24h / `am`/`pm` in 12h). Keep the existing `minutes_superscript` for the half-hour-indicator hour-cell path or unify them — call out which.
- [x] 4.6 In `build_ampm_spans`, branch per cell:
  - intermediate cell → render `minutes_superscript_full(dt.minute())`.
  - hour cell + 24h + whole-hour zone (any interval) → render `"  "` (legacy/blank).
  - hour cell + 24h + half/quarter zone → render `"³⁰"` / `"⁴⁵"` (legacy).
  - hour cell + 12h + whole-hour zone → render `"am"` / `"pm"` (legacy).
  - hour cell + 12h + half/quarter zone → render `"½a"` / `"½p"` / `"¾a"` / `"¾p"` (legacy).
- [x] 4.7 Verify `build_day_spans` still aligns midnight day labels. Hour-cell detection inside `build_day_spans` should be the same as `is_hour_cell`; intermediate cells contribute spaces, so day-label placement only happens on hour cells (existing logic should already handle this once cells iterate by `interval_minutes`).

## 5. Footer switcher

- [x] 5.1 In `render_footer`, add a new right-aligned switcher block before `Shade`:
  ```
   i  60│30│15
  ```
  using the same `key_on`/`sel`/`dim`/`sep` styles as the format switcher. Highlight the cell matching `self.interval`.
- [x] 5.2 Recompute `right_w` to include the new spans so the spacer math still works.
- [x] 5.3 Add the `i  Interval` (or similar) shortcut to the left-side shortcut list if it would otherwise be discoverable only via the switcher group — match the precedent of `f` and `w` (which are only in the right-aligned group, not the left list).

## 6. Hour-offset indicator (sub-hour formatter)

- [x] 6.1 Replace `format_hour_offset(offset: i32) -> String` with `format_offset(minutes: i32) -> String` returning:
  - `0` → `""`
  - pure hour (`minutes % 60 == 0`) → `"in N hour(s)"` / `"N hour(s) ago"` (preserve singular/plural).
  - pure sub-hour (`|minutes| < 60`) → `"in N minute(s)"` / `"N minute(s) ago"`.
  - mixed → `"in H hour(s) M minutes"` / `"H hour(s) M minutes ago"`.
- [x] 6.2 Update `render_body` to compute `selected_offset_minutes = self.cell_offset * self.interval.minutes() as i32` and pass that to `format_offset`. Keep the alignment math identical (it uses `selected_cell` index, not minutes).
- [x] 6.3 Update existing `format_hour_offset_*` tests to cover the new function: zero, ±60, ±1, ±15/30/45, ±75 ("in 1 hour 15 minutes"), ±150 ("2 hours 30 minutes ago"), large hour values.

## 7. Copy text

- [x] 7.1 Update `build_copy_text` (and the `cmd_print` caller chain in `src/main.rs`) so the selected datetime is computed from `cell_offset * interval_minutes` instead of `hour_offset` hours.
- [x] 7.2 Update copy-text tests in `src/tui/copy.rs` to construct `App` with an explicit `NavInterval` (default `H1` for legacy tests). Add one test verifying that with `interval = M15` and `cell_offset = 1`, the copied UTC time string ends with `:15`.

## 8. Integration tests / golden checks

- [x] 8.1 Write a render test for a UTC row at 60-min interval that asserts the hour-row text and sub-row text are identical to a pre-change baseline (no `·`, no superscript markers).
- [x] 8.2 Write a render test for a UTC row at 15-min interval that asserts the hour-row text contains `· · ·` between consecutive hour cells and the sub-row text contains `¹⁵ ³⁰ ⁴⁵` markers in the expected order. Whole-hour 24h zones never have an intermediate at `:00`, so `⁰⁰` does not appear in their sub-row.
- [x] 8.3 Write a render test for a Nepal (`+5:45`) row at 15-min interval that asserts hour cells stay `⁴⁵` and intermediate cells show `⁰⁰ ¹⁵ ³⁰` between consecutive hour cells (matches `view.md`).
- [x] 8.4 Write a render test for a San Jose (12h mode) row at 15-min interval that asserts hour cells stay `am`/`pm` and intermediate cells show superscript minutes (matches `view.md`).
- [x] 8.5 Write an integration test that constructs an `App` with `interval = M15` and `cell_offset = 2` and asserts `format_offset` returns `"in 30 minutes"`.

## 9. Persistence behavior

- [x] 9.1 Add a config-round-trip test confirming that pressing `i` writes the new value to `config.toml` (load → mutate → save → reload → assert).
- [x] 9.2 Add a launch-resolution test (or integration covering the `cmd_tui` path) confirming flag-wins-over-config and config-wins-over-default.

## 10. Documentation

- [x] 10.1 Update `README.md` (or equivalent help text) to document the `--interval` flag, the `i` shortcut, and the persistence semantics ("`i` persists; `--interval` is a session override").
