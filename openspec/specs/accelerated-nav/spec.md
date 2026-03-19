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
When the user holds a left or right arrow key, the step size SHALL increase based on elapsed time since the first press in the current direction.

The acceleration tiers SHALL be:
- Less than 400 ms: step size 1
- 400 ms to 1000 ms: step size 2
- 1000 ms to 2000 ms: step size 4
- Greater than 2000 ms: step size 8

#### Scenario: Initial presses stay at step 1
- **WHEN** the user begins holding a left/right key
- **AND** less than 400 ms has elapsed since the first press
- **THEN** each event moves the timeline by 1 hour

#### Scenario: Medium hold accelerates to step 2
- **WHEN** the user has been holding a left/right key for 400–1000 ms
- **THEN** each event moves the timeline by 2 hours

#### Scenario: Long hold accelerates to step 4
- **WHEN** the user has been holding a left/right key for 1000–2000 ms
- **THEN** each event moves the timeline by 4 hours

#### Scenario: Extended hold reaches max step 8
- **WHEN** the user has been holding a left/right key for more than 2000 ms
- **THEN** each event moves the timeline by 8 hours

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
