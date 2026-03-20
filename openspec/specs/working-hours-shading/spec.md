### Requirement: Three-tier hour shading
The system SHALL render hour-cell foreground colors based on the hour-of-day in the target timezone, using three tiers:
- **Working**: hours in `[work_start, work_end)` use the standard bright foreground (`HOUR_FG`)
- **Transition**: hours in `[transition_start, work_start)` or `[work_end, transition_end)` use a dimmer foreground (`HOUR_FG_TRANSITION`)
- **Night**: all remaining hours use the dimmest foreground (`HOUR_FG_NIGHT`)

The same tier logic SHALL apply to the am/pm sub-line using `AMPM_FG`, `AMPM_FG_TRANSITION`, and `AMPM_FG_NIGHT` respectively.

#### Scenario: Working hour displays bright
- **WHEN** the hour-of-day for a cell is 10 (within default work range 9–18)
- **THEN** the hour digit foreground color SHALL be `HOUR_FG` (white)

#### Scenario: Transition hour displays dimmer
- **WHEN** the hour-of-day for a cell is 7 (within default transition range 7–9)
- **THEN** the hour digit foreground color SHALL be `HOUR_FG_TRANSITION`

#### Scenario: Night hour displays dimmest
- **WHEN** the hour-of-day for a cell is 2 (outside both work and transition ranges)
- **THEN** the hour digit foreground color SHALL be `HOUR_FG_NIGHT`

#### Scenario: Shading applies per-timezone local hour
- **WHEN** two timezone rows show the same UTC instant but different local hours (e.g., one at 10am, the other at 2am)
- **THEN** the first row's cell SHALL use working-tier color and the second SHALL use night-tier color

### Requirement: Configurable hour ranges
The system SHALL read shading hour ranges from `config.toml` under a `[working_hours]` section with the following fields:
- `work_start` (integer 0–23, default 9)
- `work_end` (integer 0–23, default 18)
- `transition_start` (integer 0–23, default 7)
- `transition_end` (integer 0–23, default 20)

All fields are optional and fall back to their defaults when absent.

#### Scenario: Default ranges when section is missing
- **WHEN** `config.toml` has no `[working_hours]` section
- **THEN** the system SHALL use work_start=9, work_end=18, transition_start=7, transition_end=20

#### Scenario: Custom ranges are respected
- **WHEN** `config.toml` contains `work_start = 8` and `work_end = 17`
- **THEN** hour 8 SHALL render as working tier and hour 17 SHALL render as transition tier

### Requirement: Shading enabled flag
The system SHALL support an `enabled` boolean field in the `[working_hours]` config section (default `true`).

#### Scenario: Shading enabled by default
- **WHEN** `config.toml` has no `[working_hours]` section or no `enabled` field
- **THEN** hour shading SHALL be active

#### Scenario: Shading disabled in config
- **WHEN** `config.toml` contains `enabled = false` under `[working_hours]`
- **THEN** all hours SHALL render with the standard `HOUR_FG` color regardless of hour-of-day

### Requirement: Runtime toggle
The system SHALL provide a keyboard shortcut (`w`) to toggle working-hours shading on and off at runtime.

#### Scenario: Toggle off
- **WHEN** shading is enabled and user presses `w`
- **THEN** shading SHALL be disabled and all hours SHALL render with `HOUR_FG`

#### Scenario: Toggle on
- **WHEN** shading is disabled and user presses `w`
- **THEN** shading SHALL be re-enabled using the configured hour ranges

#### Scenario: Toggle persists to config
- **WHEN** user presses `w` to toggle shading
- **THEN** the new `enabled` value SHALL be saved to `config.toml`

### Requirement: Selected and local cell styles take precedence
When a cell is selected (highlighted) or marked as local-hour, its existing highlight/background style SHALL take precedence over the shading foreground color.

#### Scenario: Selected cell ignores shading
- **WHEN** the selected cell falls on a night hour
- **THEN** the cell SHALL use the selected style (`SELECTED_FG`/`SELECTED_BG`), not the night foreground

#### Scenario: Local-hour cell ignores shading
- **WHEN** the local-hour cell falls on a transition hour and hour_offset != 0
- **THEN** the cell SHALL use the local style (`LOCAL_BG` + `HOUR_FG`), not the transition foreground
