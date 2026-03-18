## 1. Add offset_m to TimelineParams

- [x] 1.1 Add `offset_m: i32` field to the `TimelineParams` struct in `src/tui/render.rs`
- [x] 1.2 Compute `offset_m` from `now_tz.offset().fix().local_minus_utc()` in `render_timezone_block` and pass it when constructing `TimelineParams`

## 2. Update build_ampm_spans for fractional offsets

- [x] 2.1 In `build_ampm_spans`, when `use_24h` is true and `offset_m != 0`, render superscript minute digits (`"³⁰"` for 30, `"⁴⁵"` for 45) instead of blank `"  "`
- [x] 2.2 In `build_ampm_spans`, when `use_24h` is false and `offset_m != 0`, render fraction + single-char meridiem (`"½a"`/`"½p"` for 30, `"¾a"`/`"¾p"` for 45) instead of `"am"`/`"pm"`
- [x] 2.3 Verify the existing style cascade (selected / local / default) is applied to the new text strings without additional changes

## 3. Verify rendering

- [x] 3.1 Test with a half-hour timezone (e.g., Asia/Kolkata +5:30) in 24h mode — sub-row should show `³⁰` under each hour
- [x] 3.2 Test with a half-hour timezone in 12h mode — sub-row should show `½a` / `½p`
- [x] 3.3 Test with a whole-hour timezone — confirm no change in behavior
