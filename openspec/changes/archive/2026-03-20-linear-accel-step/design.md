## Context

The TUI's horizontal navigation uses time-based acceleration: holding an arrow key increases the step size over time. Currently, `compute_step` maps elapsed time into four discrete tiers (`1 → 2 → 4 → 8`), controlled by three threshold constants (`ACCEL_TIER1_MS`, `ACCEL_TIER2_MS`, `ACCEL_TIER3_MS`) and a max (`ACCEL_MAX_STEP`). The jumps between tiers are perceptible and the four constants are more knobs than necessary.

## Goals / Non-Goals

**Goals:**
- Replace discrete tiers with a smooth linear ramp from 1 to `ACCEL_MAX_STEP`.
- Reduce acceleration constants from four to two (`ACCEL_MAX_MS`, `ACCEL_MAX_STEP`).
- Preserve all non-acceleration navigation behaviour (direction reset, key-release reset, debounce).

**Non-Goals:**
- Changing the acceleration curve to something non-linear (easing, quadratic, etc.).
- Modifying the debounce interval or key-repeat handling.
- Exposing acceleration parameters in user configuration.

## Decisions

**Linear interpolation formula**

```
t = clamp(elapsed_ms / ACCEL_MAX_MS, 0.0, 1.0)
step = 1 + ((ACCEL_MAX_STEP - 1) as f64 * t) as i32
```

Integer truncation is used rather than rounding so that `step` stays exactly 1 when `elapsed == 0` and reaches `ACCEL_MAX_STEP` only at `elapsed >= ACCEL_MAX_MS`. This keeps the first press reliably at step 1.

*Alternative considered*: `f64::round()` — rejected because rounding would reach the max step slightly before `ACCEL_MAX_MS`, which is unexpected.

**Constant naming**

`ACCEL_TIER3_MS` is renamed to `ACCEL_MAX_MS` to convey that it now represents the duration at which acceleration is fully saturated, not a tier boundary. `ACCEL_TIER1_MS` and `ACCEL_TIER2_MS` are removed entirely.

## Risks / Trade-offs

- **Slight behavioural change**: Users who relied on the "plateau" at step 2 or 4 will now experience a continuous ramp. Mitigation: the ramp covers the same time range, so the feel is similar.
- **Integer truncation granularity**: With `ACCEL_MAX_STEP = 8` and `ACCEL_MAX_MS = 2000`, there are only 8 distinct integer steps over a 2-second window. This is fine for hour-offset scrolling but would be a concern if the max step were much larger. No mitigation needed at current values.
