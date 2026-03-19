## 1. Fix day label offset

- [x] 1.1 In `build_day_spans` in `src/tui/render.rs`, change `start_pos` from `(i as usize) * p.cell_w` to `(i as usize) * p.cell_w + 1` so the label starts at the hour-digit column instead of the gap-space column
- [x] 1.2 Verify the existing `pos < total_day_chars` bounds check still prevents overflow for labels shifted by +1 near the right edge

## 2. Update spec

- [x] 2.1 Update the "Day label alignment with hour row" requirement in `openspec/specs/multi-column-day-label/spec.md` to reflect that the label starts at `cell_start + 1`

## 3. Verify

- [x] 3.1 Run `cargo build` to confirm compilation
- [x] 3.2 Run `cargo test` to confirm existing tests pass
