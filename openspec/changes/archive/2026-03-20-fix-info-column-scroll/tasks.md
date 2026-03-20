## 1. Compute selected datetime in render_timezone_row

- [x] 1.1 In `render_timezone_row`, after computing `now_tz`, call `compute_datetime_for_hour(tz, now_tz, self.hour_offset)` to get `selected_dt`
- [x] 1.2 Derive `tz_abbr` from `selected_dt.format("%Z")` instead of `now_tz`
- [x] 1.3 Derive `time_str` from `selected_dt` instead of `now_tz`
- [x] 1.4 Derive `date_str` from `selected_dt` instead of `now_tz`
- [x] 1.5 Derive `offset_m` (used in `TimelineParams`) from `selected_dt.offset().fix().local_minus_utc()` instead of `now_tz`

## 2. Update build_info_line call

- [x] 2.1 Pass `selected_dt` instead of `now_tz` as the datetime argument to `build_info_line` so the UTC offset badge is correct
