## 1. Fix build_hour_spans

- [x] 1.1 In `build_hour_spans`, replace `let hour_in_day = ((h % 24) + 24) % 24;` with a call to `compute_datetime_for_hour(p.tz, p.now_tz, h - p.current_hour)` and derive `hour_in_day` from `.hour() as i32`
- [x] 1.2 Verify that working-hours shading (`hour_fg_color`) receives the DST-aware hour

## 2. Fix build_ampm_spans

- [x] 2.1 In `build_ampm_spans`, replace `let hour_in_day = ((h % 24) + 24) % 24;` with the same `compute_datetime_for_hour` call and `.hour() as i32`
- [x] 2.2 Verify that AM/PM text and `ampm_fg_color` shading use the DST-aware hour

## 3. Fix build_day_spans midnight detection

- [x] 3.1 In `build_day_spans`, replace `let hour_in_day = ((h % 24) + 24) % 24;` with `compute_datetime_for_hour(...).hour()` and change the midnight check to use the resolved hour instead of arithmetic

## 4. Validate consistency with copy.rs

- [x] 4.1 Build the project (`cargo build`) and confirm no compilation errors
- [ ] 4.2 Manually verify that scrolling across a DST boundary shows correct hours matching copy output
