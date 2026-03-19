## 1. TUI timeline day labels

- [x] 1.1 In `build_day_spans` (`src/tui/render.rs`), pass today's date (`now_tz.date_naive()`) into the label construction logic so it can compare month/year
- [x] 1.2 Update the `day_label` format string: append `, {Month}` when `dt.month() != today.month()`, and further append `, {Year}` when `dt.year() != today.year()`
- [x] 1.3 Verify existing truncation logic handles longer labels (no code change expected, just confirm)

## 2. Copy-to-clipboard day suffix

- [x] 2.1 In `build_copy_text` (`src/tui/copy.rs`), capture `ref_month` and `ref_year` from the first timezone's selected datetime
- [x] 2.2 Update the `day_suffix` format: append `, {Month}` when the timezone date's month differs from `ref_month`, and `, {Year}` when the year differs from `ref_year`

## 3. Tests

- [x] 3.1 Add a test in `render.rs` for `build_day_spans` with a month-boundary crossing (label includes month name)
- [x] 3.2 Add a test in `render.rs` for `build_day_spans` with a year-boundary crossing (label includes month and year)
- [x] 3.3 Add a test in `copy.rs` for `build_copy_text` where a timezone's date is in a different month (day suffix includes month)
- [x] 3.4 Add a test in `copy.rs` for `build_copy_text` where a timezone's date is in a different year (day suffix includes month and year)
- [x] 3.5 Verify existing same-month tests still pass unchanged
