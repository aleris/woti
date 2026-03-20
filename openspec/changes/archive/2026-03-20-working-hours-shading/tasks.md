## 1. Config

- [x] 1.1 Add `WorkingHoursConfig` struct to `src/config.rs` with fields: `enabled` (bool, default true), `work_start` (u8, default 9), `work_end` (u8, default 18), `transition_start` (u8, default 7), `transition_end` (u8, default 20). Derive Serialize/Deserialize with `serde(default)`.
- [x] 1.2 Add `working_hours: WorkingHoursConfig` field (with `serde(default)`) to `AppConfig` so missing `[working_hours]` section in TOML deserializes to defaults.
- [x] 1.3 Add unit tests: round-trip serialization with and without `[working_hours]` section; verify defaults when section is absent.

## 2. Theme colors

- [x] 2.1 Add `HOUR_FG_TRANSITION` and `HOUR_FG_NIGHT` color constants to `src/tui/theme.rs`.
- [x] 2.2 Add `AMPM_FG_TRANSITION` and `AMPM_FG_NIGHT` color constants to `src/tui/theme.rs`.

## 3. Render logic

- [x] 3.1 Add helper function `hour_fg_color(hour_in_day: i32, shading: &WorkingHoursConfig) -> Color` in `src/tui/render.rs` that returns `HOUR_FG`, `HOUR_FG_TRANSITION`, or `HOUR_FG_NIGHT` based on the hour tier.
- [x] 3.2 Add helper function `ampm_fg_color(hour_in_day: i32, shading: &WorkingHoursConfig) -> Color` that returns the corresponding ampm foreground color.
- [x] 3.3 Update `build_hour_spans` to call `hour_fg_color` for non-selected, non-local cells when shading is enabled; pass `WorkingHoursConfig` through `TimelineParams`.
- [x] 3.4 Update `build_ampm_spans` to call `ampm_fg_color` for non-selected, non-local cells when shading is enabled.
- [x] 3.5 Add `shading` field (`WorkingHoursConfig` + enabled flag) to `TimelineParams` and populate it from `App` state in `render_timezone_block`.
- [x] 3.6 Add unit tests for `hour_fg_color`: verify working, transition, and night hours return correct colors; verify boundary hours (e.g., hour 9 is working, hour 7 is transition, hour 6 is night).

## 4. App state and toggle

- [x] 4.1 Add `shading_enabled: bool` field to `App` in `src/tui/app.rs`, initialized from `config.working_hours.enabled`.
- [x] 4.2 Handle `w` key in `src/tui/event.rs`: toggle `shading_enabled`, update `config.working_hours.enabled`, and call `config.save()`.
- [x] 4.3 Add `w Shade` shortcut hint to the footer bar in `render_footer`.

## 5. Integration verification

- [x] 5.1 Manual smoke test: run the app, verify three-tier coloring appears, press `w` to toggle on/off, check config.toml persists the change.
