### Requirement: Single-tap navigation unchanged
A single left or right arrow key press SHALL move the timeline by exactly 1 hour in the corresponding direction, regardless of acceleration state.

#### Scenario: Single left tap
- **WHEN** the user presses the left arrow key once and releases
- **THEN** `hour_offset` decreases by 1

#### Scenario: Single right tap
- **WHEN** the user presses the right arrow key once and releases
- **THEN** `hour_offset` increases by 1

### Requirement: Repeat events are handled for left/right keys
The event loop SHALL process `KeyEventKind::Repeat` events for left and right arrow keys in addition to `KeyEventKind::Press` events.

#### Scenario: Held key generates repeat events
- **WHEN** the user holds down the left or right arrow key and the terminal emits `Repeat` events
- **THEN** each `Repeat` event SHALL be treated as a navigation step (with acceleration applied)

#### Scenario: Terminal without repeat support
- **WHEN** the terminal does not emit `KeyEventKind::Repeat` events
- **THEN** navigation SHALL still work via individual `Press` events at step size 1

### Requirement: Time-based acceleration ramp
When the user holds a left or right arrow key, the step size SHALL increase linearly based on elapsed time since the first press in the current direction.

The step size SHALL be computed as:
- `t = clamp(elapsed_ms / ACCEL_MAX_MS, 0.0, 1.0)`
- `step = 1 + floor((ACCEL_MAX_STEP - 1) * t)`

Where `ACCEL_MAX_MS` is 2000 and `ACCEL_MAX_STEP` is 8.

This means the step grows continuously from 1 (at 0 ms) to `ACCEL_MAX_STEP` (at `ACCEL_MAX_MS` and beyond).

#### Scenario: Immediate press is step 1
- **WHEN** the user begins holding a left/right key
- **AND** 0 ms has elapsed
- **THEN** each event moves the timeline by 1 hour

#### Scenario: Midpoint of ramp
- **WHEN** the user has been holding a left/right key for 1000 ms (half of ACCEL_MAX_MS)
- **THEN** each event moves the timeline by 4 hours (`1 + floor(7 * 0.5) = 4`)

#### Scenario: Near start of ramp
- **WHEN** the user has been holding a left/right key for 400 ms
- **THEN** each event moves the timeline by 2 hours (`1 + floor(7 * 0.2) = 2`)

#### Scenario: At maximum
- **WHEN** the user has been holding a left/right key for 2000 ms or more
- **THEN** each event moves the timeline by 8 hours (ACCEL_MAX_STEP)

#### Scenario: Beyond maximum is clamped
- **WHEN** the user has been holding a left/right key for 5000 ms
- **THEN** each event moves the timeline by 8 hours (does not exceed ACCEL_MAX_STEP)

### Requirement: Acceleration resets on direction change
The acceleration state SHALL reset to step size 1 when the user changes navigation direction (e.g., switches from left to right).

#### Scenario: Direction reversal resets acceleration
- **WHEN** the user has been holding the right arrow key for over 1 second (step size 4)
- **AND** the user then presses the left arrow key
- **THEN** the first left step SHALL move 1 hour (acceleration restarts from zero)

### Requirement: Acceleration resets on key release
The acceleration state SHALL reset when the user releases the navigation key, detected by a poll timeout with no pending offset.

#### Scenario: Release and re-press restarts acceleration
- **WHEN** the user holds the right arrow key for 2+ seconds, then releases
- **AND** the user presses right again after a pause
- **THEN** the new sequence SHALL start at step size 1

### Requirement: Non-navigation keys reset acceleration
The acceleration state SHALL reset when any non-left/right key event is received.

#### Scenario: Copy key interrupts acceleration
- **WHEN** the user is holding the right arrow key with active acceleration
- **AND** the user presses 'c' to copy
- **THEN** acceleration state SHALL reset, and the next right press starts at step size 1
