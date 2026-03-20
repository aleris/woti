## Why

The current `compute_step` function uses discrete tiers (1 → 2 → 4 → 8) that create noticeable "jumps" in scroll speed
as the user holds a navigation key. A linear ramp from 1 to `ACCEL_MAX_STEP` over the acceleration window produces
smoother, more predictable acceleration and is simpler to configure — only two constants (`ACCEL_MAX_MS`,
`ACCEL_MAX_STEP`) instead of four.

## What Changes

- Replace the tiered `compute_step` logic with a linear interpolation:
  `step = 1 + (ACCEL_MAX_STEP - 1) * clamp(elapsed / ACCEL_MAX_MS, 0, 1)`, truncated to an integer.
- Remove constants `ACCEL_TIER1_MS`, `ACCEL_TIER2_MS`, and `ACCEL_TIER3_MS`.
- Rename / replace them with a single `ACCEL_MAX_MS` constant (same value as the old tier-3 threshold, 2000 ms).
- Update the existing unit tests to verify the linear curve instead of tier boundaries.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `accelerated-nav`: The acceleration ramp requirement changes from discrete tiers to a continuous linear interpolation
  over `0..ACCEL_MAX_MS`.

## Impact

- `src/tui/mod.rs` — constant definitions (remove 3, keep/rename 1).
- `src/tui/event.rs` — `compute_step` function body and its import line.
- `src/tui/event.rs` — unit tests in `mod tests`.
- No public API, dependency, or behavioral changes outside of step-size granularity.
