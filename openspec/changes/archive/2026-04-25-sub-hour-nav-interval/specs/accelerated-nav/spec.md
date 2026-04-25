## MODIFIED Requirements

### Requirement: Single-tap navigation unchanged
A single left or right arrow key press SHALL move the timeline by exactly 1 cell in the corresponding direction, regardless of acceleration state. A cell represents the active navigation interval (60, 30, or 15 minutes — see `sub-hour-nav-interval`); when the interval is 60 minutes (default) one cell equals 1 hour, preserving the original behavior.

#### Scenario: Single left tap (60-minute interval)
- **WHEN** the active interval is 60 minutes
- **AND** the user presses the left arrow key once and releases
- **THEN** `cell_offset` decreases by 1 (selection moves back 1 hour)

#### Scenario: Single right tap (60-minute interval)
- **WHEN** the active interval is 60 minutes
- **AND** the user presses the right arrow key once and releases
- **THEN** `cell_offset` increases by 1 (selection moves forward 1 hour)

#### Scenario: Single right tap (15-minute interval)
- **WHEN** the active interval is 15 minutes
- **AND** the user presses the right arrow key once and releases
- **THEN** `cell_offset` increases by 1 (selection moves forward 15 minutes)

#### Scenario: Single left tap (30-minute interval)
- **WHEN** the active interval is 30 minutes
- **AND** the user presses the left arrow key once and releases
- **THEN** `cell_offset` decreases by 1 (selection moves back 30 minutes)

### Requirement: Time-based acceleration ramp
When the user holds a left or right arrow key, the step size SHALL increase linearly based on elapsed time since the first press in the current direction. The step size is expressed in cells (each cell representing the active interval), so the per-press wall-clock distance equals `step * interval_minutes`.

The step size SHALL be computed as:
- `t = clamp(elapsed_ms / ACCEL_MAX_MS, 0.0, 1.0)`
- `step = 1 + floor((ACCEL_MAX_STEP - 1) * t)`

Where `ACCEL_MAX_MS` is 2000 and `ACCEL_MAX_STEP` is 8.

This means the cell step grows continuously from 1 (at 0 ms) to `ACCEL_MAX_STEP` (at `ACCEL_MAX_MS` and beyond), independent of the active interval.

#### Scenario: Immediate press is step 1
- **WHEN** the user begins holding a left/right key
- **AND** 0 ms has elapsed
- **THEN** each event moves the selection by 1 cell (= 1 × interval minutes)

#### Scenario: Midpoint of ramp at 60-minute interval
- **WHEN** the user has been holding a left/right key for 1000 ms (half of ACCEL_MAX_MS)
- **AND** the active interval is 60 minutes
- **THEN** each event moves the selection by 4 cells = 4 hours (`1 + floor(7 * 0.5) = 4`)

#### Scenario: Midpoint of ramp at 15-minute interval
- **WHEN** the user has been holding a left/right key for 1000 ms
- **AND** the active interval is 15 minutes
- **THEN** each event moves the selection by 4 cells = 60 minutes

#### Scenario: Near start of ramp
- **WHEN** the user has been holding a left/right key for 400 ms
- **THEN** each event moves the selection by 2 cells (`1 + floor(7 * 0.2) = 2`)

#### Scenario: At maximum at 60-minute interval
- **WHEN** the user has been holding a left/right key for 2000 ms or more at 60-minute interval
- **THEN** each event moves the selection by 8 cells = 8 hours (ACCEL_MAX_STEP)

#### Scenario: At maximum at 15-minute interval
- **WHEN** the user has been holding a left/right key for 2000 ms or more at 15-minute interval
- **THEN** each event moves the selection by 8 cells = 2 hours

#### Scenario: Beyond maximum is clamped
- **WHEN** the user has been holding a left/right key for 5000 ms
- **THEN** each event moves the selection by 8 cells (does not exceed ACCEL_MAX_STEP)

### Requirement: Non-navigation keys reset acceleration
The acceleration state SHALL reset when any non-left/right key event is received. This includes the `i` key (interval cycle), `w` (shading), `f` (format), `c` (copy), and `q`/`x`/`Esc` (quit).

#### Scenario: Copy key interrupts acceleration
- **WHEN** the user is holding the right arrow key with active acceleration
- **AND** the user presses 'c' to copy
- **THEN** acceleration state SHALL reset, and the next right press starts at step size 1

#### Scenario: Interval cycle key interrupts acceleration
- **WHEN** the user is holding the right arrow key with active acceleration
- **AND** the user presses `i` to cycle the active interval
- **THEN** acceleration state SHALL reset, and the next right press starts at step size 1
