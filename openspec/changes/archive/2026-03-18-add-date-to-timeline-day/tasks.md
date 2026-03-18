## 1. Replace day_spans with character buffer

- [x] 1.1 Introduce a `day_buf: Vec<(char, Style)>` initialized to spaces with default style, sized to `num_cells * CELL_WIDTH` characters
- [x] 1.2 Remove the existing `day_spans` vector

## 2. Write day labels into the buffer

- [x] 2.1 In the cell loop, when `hour_in_day == 0`, format the day label as `"{} {}"` using `dt.format("%a").to_uppercase()` and `dt.day()`
- [x] 2.2 Write each character of the label into `day_buf` starting at position `i * CELL_WIDTH`, applying the per-character style based on which cell the character falls in (selected, local, or normal magenta bold)
- [x] 2.3 Truncate the label if it would extend past `day_buf.len()`

## 3. Apply per-cell background styles to the buffer

- [x] 3.1 In the cell loop, for positions that are still default spaces (not overwritten by a label), apply the cell's background style (selected highlight or local-hour background)

## 4. Convert buffer to spans

- [x] 4.1 After the cell loop, convert `day_buf` into a `Vec<Span>` by grouping consecutive characters with the same `Style` into single `Span`s
- [x] 4.2 Use the resulting spans in place of `day_spans` when building the day row `Line`

## 5. Verify

- [x] 5.1 Build and run the TUI to confirm day markers display as "WED 18" format, aligned with the hour row, with correct styling across selected/local/normal columns
