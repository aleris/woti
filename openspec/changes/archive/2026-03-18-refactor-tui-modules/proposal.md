## Why

`src/tui.rs` is a 652-line monolith mixing terminal lifecycle, event handling, rendering (header, footer, body, timezone blocks), clipboard/copy logic, time-format helpers, and platform-specific 24h detection. This makes it hard to navigate, test individual pieces in isolation, and extend with new features without growing the file further.

## What Changes

- Split `tui.rs` into a `tui/` module directory with focused submodules:
  - `tui/app.rs` — `App` struct, state fields, public `run()` entry point, terminal setup/teardown
  - `tui/event.rs` — event loop, key dispatch, debounce logic
  - `tui/render.rs` — top-level `render()` layout split and per-section rendering (`render_header`, `render_footer`, `render_body`, `render_timezone_block`)
  - `tui/copy.rs` — `copy_selection()`, `build_copy_text()`
  - `tui/time_format.rs` — `cycle_time_format()`, `use_24h_for_header()`, `use_24h_for_tz()`, `detect_use_24h()`, `macos_24h_preference()`
- Shared constants (`CELL_WIDTH`, `INFO_COL_WIDTH`, `TIMELINE_GAP`, `BLOCK_HEIGHT`, `DEBOUNCE_MS`) live in `tui/mod.rs` — the module root, importable by all submodules
- `compute_datetime_for_hour()` moves into `tui/render.rs` as `pub(super)` (primary consumer is rendering; `copy.rs` imports it from there)
- Extract `render_timezone_block` internals into smaller functions (day-marker span building, hour-span building, info-column building)
- Re-export `App` from `tui/mod.rs` so the rest of the crate (`main.rs`) is unaffected

## Capabilities

### New Capabilities

_None — this is a pure refactor with no new user-facing behavior._

### Modified Capabilities

_None — existing behavior for `copy-selection` and `multi-column-day-label` is preserved as-is._

## Impact

- **Code**: `src/tui.rs` is replaced by `src/tui/mod.rs` + 5 submodules (`app`, `event`, `render`, `copy`, `time_format`). All other source files (`main.rs`, `config.rs`, `cli.rs`, `timezone.rs`, `tz_data.rs`) are unchanged except `main.rs` which already uses `tui::App` and needs no modification.
- **APIs**: No public API changes — `tui::App::new()` and `tui::App::run()` remain the only public surface.
- **Dependencies**: No new crate dependencies.
- **Risk**: Low — behavior-preserving restructuring; existing specs continue to hold.
