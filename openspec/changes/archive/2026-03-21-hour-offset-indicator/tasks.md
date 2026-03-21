## 1. Format function

- [x] 1.1 Add `format_hour_offset(offset: i32) -> String` in `src/tui/render.rs` that returns "in N hour(s)" for positive offsets, "N hour(s) ago" for negative, and empty string for zero
- [x] 1.2 Add unit tests for `format_hour_offset`: zero returns empty, ±1 singular, ±N plural, large values

## 2. Render the indicator row

- [x] 2.1 In `render_body`, after the timezone block loop, render a 1-row indicator when `hour_offset != 0`: compute left padding to align with the selected column, emit the label with `selected_style()`
- [x] 2.2 Verify the indicator disappears when navigating back to `hour_offset == 0` and reappears on any left/right movement
