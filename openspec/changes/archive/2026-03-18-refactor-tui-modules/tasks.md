## 1. Create module directory and scaffold

- [x] 1.1 Create `src/tui/` directory and `src/tui/mod.rs` with submodule declarations (`mod app; mod event; mod render; mod copy; mod time_format;`), shared constants (`CELL_WIDTH`, `INFO_COL_WIDTH`, `TIMELINE_GAP`, `BLOCK_HEIGHT`, `DEBOUNCE_MS`), and `pub use app::App;` re-export
- [x] 1.2 Delete `src/tui.rs` (replaced by the module directory)

## 2. Extract time_format module

- [x] 2.1 Create `src/tui/time_format.rs` — move `detect_use_24h()`, `macos_24h_preference()`, `use_24h_for_header()`, `use_24h_for_tz()`, and `cycle_time_format()` into this module. `use_24h_for_header`, `use_24h_for_tz`, and `cycle_time_format` take `&App` / `&mut App` so expose them as `pub(super)` free functions or methods on App via an extension approach; simplest is keeping them as `pub(super)` methods on App by implementing them in this file using `impl App` block

## 3. Extract copy module

- [x] 3.1 Create `src/tui/copy.rs` — move `copy_selection()` and `build_copy_text()` as `pub(super)` methods on App (using an `impl App` block in this file)

## 4. Extract render module and break down render_timezone_block

- [x] 4.1 Create `src/tui/render.rs` — move `render()`, `render_header()`, `render_footer()`, `render_body()`, `render_timezone_block()`, and `compute_datetime_for_hour()` into this module
- [x] 4.2 Extract `build_hour_spans()` from the hour-number loop inside `render_timezone_block` — takes timeline params, returns `Vec<Span>`
- [x] 4.3 Extract `build_ampm_spans()` from the am/pm loop inside `render_timezone_block` — takes timeline params, returns `Vec<Span>`
- [x] 4.4 Extract `build_day_spans()` from the day-marker character buffer logic inside `render_timezone_block` — takes timeline params, returns `Vec<Span>`
- [x] 4.5 Extract `build_info_line()` from the info-column assembly in `render_timezone_block` — takes entry + computed tz values, returns the city/badge/offset/time `Line`

## 5. Extract event module

- [x] 5.1 Create `src/tui/event.rs` — move `event_loop()` as a `pub(super)` method on App (using an `impl App` block)

## 6. Finalize app module

- [x] 6.1 Create `src/tui/app.rs` — define `App` struct, `App::new()`, `App::run()` (terminal lifecycle), and `max_scroll()`. Import and call into event, render, copy, and time_format submodules

## 7. Verify

- [x] 7.1 Run `cargo build` — confirm zero compilation errors
- [x] 7.2 Run `cargo test` — confirm all existing tests pass
- [x] 7.3 Run the TUI manually and verify visual output matches pre-refactor behavior
