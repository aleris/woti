## Context

woti is a terminal timezone viewer built with Rust/ratatui. The timeline renders hour cells using a single foreground
color (`theme::HOUR_FG`, white) for every hour. Users scanning multiple timezones cannot instantly tell which hours fall
in working time vs. off-hours.

The config file (`config.toml`) already supports `timezones` and `time_format`. It is loaded at startup into `AppConfig`
and serialized with serde/toml.

Hour cells are rendered in `build_hour_spans()` in `src/tui/render.rs`, which resolves the style per cell. The `am/pm`
sub-line follows the same pattern in `build_ampm_spans()`.

## Goals / Non-Goals

**Goals:**

- Three-tier hour shading: working → transition → night, each with a distinct foreground color
- Hour ranges for each tier are user-configurable in `config.toml`
- Runtime toggle (keyboard shortcut) to enable/disable shading
- The toggle state persists in config

**Non-Goals:**

- Per-timezone working hours (all timezones use the same hour ranges, applied to their local hour)
- Background color changes for hour cells (only foreground brightness changes)
- Gradual/smooth color gradients between tiers

## Decisions

### 1. Config shape — flat ranges on `AppConfig`

Add an optional `[working_hours]` section to `AppConfig`:

```toml
[working_hours]
enabled = true
work_start = 9
work_end = 18
transition_start = 7
transition_end = 20
```

The three tiers are derived: hours in `[work_start, work_end)` → working, hours in `[transition_start, work_start)` or
`[work_end, transition_end)` → transition, everything else → night.

**Why flat fields over an array of ranges**: The three-tier model is simple and maps directly to the user's mental
model ("working hours", "buffer hours", "night"). An array of arbitrary ranges would complicate validation and the
config file for no added value.

**Why on `AppConfig`**: It keeps one config struct, one file, and one load/save path. No new files or subsystems needed.

### 2. Theme colors — three new constants

Add `HOUR_FG_TRANSITION` and `HOUR_FG_NIGHT` alongside the existing `HOUR_FG` (which becomes the "work" color).
Similarly `AMPM_FG_TRANSITION` and `AMPM_FG_NIGHT` for the am/pm row.

**Why constants, not computed**: Matches the existing theme pattern (all colors are `pub const`). Keeps the theme
centralized and the render code simple.

### 3. Color resolution — helper function in render.rs

A small function `hour_fg_color(hour_in_day: i32, shading: &WorkingHoursConfig) -> Color` returns the correct foreground
color. Called from `build_hour_spans` and `build_ampm_spans`.

**Why a function, not a lookup table**: The logic is three range checks — a function is clear, testable, and avoids
allocating a 24-element array on every render tick.

### 4. Toggle keybinding — `w` key

`w` toggles `shading_enabled` on `App`. When disabled, all hours render with `HOUR_FG` as they do today. The toggle also
persists the new value to config via the existing `save()` path.

**Why `w`**: Mnemonic for "working hours". The key is not currently bound.

### 5. Runtime state on `App`

Add `shading_enabled: bool` to `App`, initialized from `config.working_hours.enabled`. The render path reads this flag;
the event handler toggles it and calls `config.save()`.

## Risks / Trade-offs

- [Config migration] Existing `config.toml` files won't have `[working_hours]`. → Mitigation: all new fields use
  `serde(default)` so missing section defaults to enabled with the standard 9–18 / 7–20 ranges.
- [Visual subtlety] If terminal color support is limited, the three shades may look identical. → Mitigation: the chosen
  RGB values are spaced far enough apart (White → Gray → DarkGray equivalent) to be distinguishable on most modern
  terminals.
- [Footer crowding] Adding the `w` shortcut to the footer bar takes horizontal space. → Mitigation: use compact label
  format like other shortcuts (`w` + `Shade`).
