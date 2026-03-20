## 1. Update constants

- [x] 1.1 In `src/tui/mod.rs`, remove `ACCEL_TIER1_MS`, `ACCEL_TIER2_MS`, and `ACCEL_TIER3_MS`; add `ACCEL_MAX_MS: u64 = 2000`
- [x] 1.2 Verify `ACCEL_MAX_STEP` remains `8` (no change needed, just confirm)

## 2. Rewrite compute_step

- [x] 2.1 In `src/tui/event.rs`, update the import line to use `ACCEL_MAX_MS` instead of the three tier constants
- [x] 2.2 Replace the tiered if/else body of `compute_step` with the linear interpolation formula: `1 + ((ACCEL_MAX_STEP - 1) as f64 * (elapsed_ms as f64 / ACCEL_MAX_MS as f64).clamp(0.0, 1.0)) as i32`

## 3. Update tests

- [x] 3.1 Replace the four tier-based tests with tests that verify the linear curve at key points: 0 ms → 1, 400 ms → 2, 1000 ms → 4, 2000 ms → 8, 5000 ms → 8 (clamped)
