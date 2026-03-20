## Why

The timeline currently shows every hour in the same white color, making it hard to visually distinguish working hours
from off-hours at a glance. Adding day/night shading to hour cells lets users instantly see which hours fall within
working time, transition periods, and off-hours across all displayed timezones.

## What Changes

- Hour cells in the timeline are shaded with three tiers of brightness based on the hour-of-day:
    - **Working hours** (default 9–18): current white color (unchanged)
    - **Transition hours** (default 7–8, 19–20): a dimmer foreground color
    - **Night hours** (remaining): an even dimmer foreground color
- The hour ranges for each tier are defined in `config.toml` so users can customise them with a file edit
- A new config flag + keyboard toggle allows turning working-hours shading on/off at runtime

## Capabilities

### New Capabilities

- `working-hours-shading`: Hour-cell foreground colors vary by time-of-day tier, with configurable hour ranges and a
  runtime toggle

### Modified Capabilities

## Impact

- `src/config.rs` – new fields on `AppConfig` for shading ranges + enabled flag
- `src/tui/theme.rs` – new color constants for transition and night hour foregrounds
- `src/tui/render.rs` – `build_hour_spans` (and potentially `build_ampm_spans`) resolve foreground color per hour-of-day
  using the shading config
- `src/tui/app.rs` – new runtime state field for shading enabled
- `src/tui/event.rs` – new keybinding to toggle shading
- `config.toml` – new optional `[working_hours]` section
