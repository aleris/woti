## MODIFIED Requirements

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
