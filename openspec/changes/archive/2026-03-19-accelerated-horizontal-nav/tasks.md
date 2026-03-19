## 1. Acceleration constants

- [x] 1.1 Add acceleration threshold constants to `src/tui/mod.rs` (or alongside `DEBOUNCE_MS`): `ACCEL_TIER1_MS = 400`, `ACCEL_TIER2_MS = 1000`, `ACCEL_TIER3_MS = 2000`, `ACCEL_MAX_STEP = 8`

## 2. Event loop acceleration logic

- [x] 2.1 Add local acceleration state variables to the event loop in `src/tui/event.rs`: `nav_start: Option<Instant>` and `nav_dir: i32`
- [x] 2.2 Expand the left/right key match arm to also accept `KeyEventKind::Repeat`
- [x] 2.3 Implement a `compute_step(elapsed: Duration) -> i32` helper (private function or closure) that returns 1/2/4/8 based on the acceleration thresholds
- [x] 2.4 On left/right Press/Repeat: set or update `nav_start`/`nav_dir`, compute the step, and add `±step` to `pending_h_offset` instead of `±1`
- [x] 2.5 Reset `nav_start` to `None` on direction change (left↔right)
- [x] 2.6 Reset `nav_start` to `None` when a non-left/right key event is received
- [x] 2.7 Reset `nav_start` to `None` on poll timeout when `pending_h_offset == 0` (key released)

## 3. Testing

- [x] 3.1 Add unit tests for `compute_step`: verify each tier boundary (0ms→1, 400ms→2, 1000ms→4, 2000ms→8)
