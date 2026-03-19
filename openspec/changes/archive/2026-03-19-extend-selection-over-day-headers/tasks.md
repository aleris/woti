## 1. Add label origin tracking

- [x] 1.1 In `build_day_spans` in `src/tui/render.rs`, add a `day_label_origin: Vec<Option<i32>>` array (same length as `day_chars`) initialized to `None`
- [x] 1.2 In the midnight label-writing loop, set `day_label_origin[pos] = Some(h)` for each character position where a label character is written

## 2. Extend highlight logic in style_for

- [x] 2.1 Capture `day_label_origin` in the `style_for` closure alongside `day_is_label`
- [x] 2.2 Add selection extension: when `day_label_origin[pos] == Some(p.base_hour)` AND `day_is_label[pos]`, apply `selected_style()` regardless of which cell the position is in
- [x] 2.3 Add local-hour extension: when `day_label_origin[pos] == Some(p.current_hour)` AND `p.hour_offset != 0` AND `day_is_label[pos]`, apply the local-hour label style
- [x] 2.4 Preserve existing behavior: non-label positions and labels not originating from the selected/local hour continue using the current per-cell logic

## 3. Tests

- [x] 3.1 Add test: selected hour at midnight produces selection style across the full day label span (all label chars get SELECTED_BG)
- [x] 3.2 Add test: selected hour NOT at midnight keeps existing single-cell highlight behavior in day row
- [x] 3.3 Add test: label with month/year suffix still gets full highlight when selected at its midnight cell
