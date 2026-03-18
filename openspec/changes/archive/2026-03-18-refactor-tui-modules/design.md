## Context

`src/tui.rs` is currently a single 652-line file that owns all TUI concerns: terminal setup/teardown, the event loop with debounced key handling, the full rendering pipeline (header bar, scrollable timezone body, footer shortcut bar), clipboard copy logic, time-format cycling, and platform-specific 24h detection. The rest of the crate is already well-factored (`config.rs`, `cli.rs`, `timezone.rs`, `tz_data.rs`), so `tui.rs` is the only file that needs attention.

## Goals / Non-Goals

**Goals:**
- Break `tui.rs` into a `tui/` module directory with focused submodules so each file has a single responsibility
- Extract large nested closures and multi-concern functions into smaller, named functions
- Keep the public API (`tui::App::new`, `tui::App::run`) unchanged so `main.rs` needs zero modifications
- Keep all existing behavior and rendering pixel-identical

**Non-Goals:**
- Adding tests (can be done in a follow-up once the modules are in place)
- Changing any visual output, keybindings, or user-facing behavior
- Introducing new abstractions like traits or generics — keep it simple
- Refactoring other modules (`config.rs`, `main.rs`, etc.)

## Decisions

### 1. Module directory layout: `src/tui/` with 6 files

```
src/tui/
  mod.rs           — re-export App; declare submodules
  app.rs           — App struct + state + run() + terminal lifecycle
  event.rs         — event_loop() + key dispatch
  render.rs        — render(), render_header, render_footer, render_body, render_timezone_block
  copy.rs          — copy_selection(), build_copy_text()
  time_format.rs   — time format helpers + detect_use_24h() + macos_24h_preference()
```

**Rationale**: Each file maps to a distinct concern. `render.rs` remains the largest (~250 lines) but it's cohesive — all rendering. A further split of render into header/footer/body was considered but adds module overhead for tightly coupled code that shares the same `&self` methods.

**Alternative considered**: Keeping a flat `tui_render.rs`, `tui_event.rs` etc. at the `src/` level. Rejected because a module directory is idiomatic Rust and keeps the top-level `src/` clean.

### 2. Move free functions into the submodule that owns their concern

`detect_use_24h()`, `macos_24h_preference()`, and `compute_datetime_for_hour()` are currently free functions at module scope. Each moves to the submodule that owns its domain:

- `detect_use_24h` + `macos_24h_preference` → `time_format.rs` — these are time-format detection, called from `App::new`
- `compute_datetime_for_hour` → `render.rs` as `pub(super)` — its primary consumer is the rendering pipeline; `copy.rs` imports it from there

### 3. Extract sub-functions inside `render_timezone_block`

`render_timezone_block` is ~210 lines with inline logic for:
- Building hour spans (the numbered timeline row)
- Building am/pm spans (the bottom row of the timeline)
- Building day-marker spans (the top row with day labels)
- Assembling the info column (city, TZ badge, offset, time)

Each of these becomes a separate private function in `render.rs`:
- `build_hour_spans(...)` → returns `Vec<Span>`
- `build_ampm_spans(...)` → returns `Vec<Span>`
- `build_day_spans(...)` → returns `Vec<Span>`
- `build_info_line(...)` → returns `Line`

This reduces `render_timezone_block` to orchestration: compute shared values, call builders, assemble rows.

### 4. Visibility: `pub(crate)` for App, `pub(super)` within tui/

- `App` and its `new()`/`run()` methods stay `pub` (re-exported from `tui/mod.rs`)
- Helper methods used across submodules use `pub(super)` — visible within `tui/` but not outside
- Everything else is private to its file

### 5. Constants stay shared via `mod.rs` or a top-level `const` block

`CELL_WIDTH`, `INFO_COL_WIDTH`, `TIMELINE_GAP`, `BLOCK_HEIGHT`, `DEBOUNCE_MS` are used across `render.rs` and `event.rs`. They'll be defined in `mod.rs` (or a shared constants section) and imported by submodules.

## Risks / Trade-offs

- **[Risk] Rendering regression** → Mitigation: This is a move-only + extract-function refactor. Manual visual testing before/after confirms pixel-identical output. Existing specs (`copy-selection`, `multi-column-day-label`) serve as behavioral contracts.
- **[Risk] Increased file count** → Mitigation: 6 small focused files (each 40–250 lines) is easier to navigate than 1 large file. Standard Rust module conventions.
- **[Trade-off] `render.rs` is still ~250 lines** → Acceptable: it's all rendering and cohesive. Splitting further into `render_header.rs` etc. adds indirection without meaningful benefit at this scale.
