## Context

The TUI renders a horizontal timeline where each timezone has three rows: day labels (row 0), hour numbers (row 1), and AM/PM or minute superscripts (row 2). Day labels like "WED 19" are written at midnight boundaries and naturally span multiple 3-character cells.

Currently, `build_day_spans`'s `style_for` closure determines highlight style purely by which cell a character position falls in (`cell_idx = pos / cell_w`). This means when the selected hour is midnight, only the characters within that one 3-char cell get the selection background — the rest of the day label keeps its normal style, creating an abrupt visual cut.

## Goals / Non-Goals

**Goals:**
- When the selected hour is at a day boundary (midnight), extend the selection highlight across the entire day label text so it looks like a continuous, natural selection.
- Apply the same extension for the local-hour highlight when it falls on midnight.
- Preserve the existing highlight behavior for hours that are not day boundaries (no change to normal cells).

**Non-Goals:**
- Changing how selections work in the hour row or AM/PM row (those remain single-cell highlights).
- Adding multi-hour selection or range selection.
- Changing the day label text content or formatting.

## Decisions

### Track label origin hour per character position

**Decision:** Add a `day_label_origin: Vec<Option<i32>>` array parallel to `day_chars` and `day_is_label`. For each character position that belongs to a day label, store the hour `h` (the midnight hour) that generated that label. Non-label positions store `None`.

**Rationale:** This lets `style_for` know not just "is this a label?" but "which midnight hour produced this label?", enabling it to extend the highlight to all positions belonging to the same label as the selected cell.

**Alternative considered:** Tracking label start/end ranges separately — more complex with no benefit since we already iterate positions.

### Extend highlight in `style_for` based on label origin

**Decision:** In `style_for`, when a position has a `day_label_origin` matching `p.base_hour` (selected) or `p.current_hour` (local), apply the highlight style regardless of which cell the position is in.

**Rationale:** This is the minimal change — one additional condition in the existing closure. It naturally handles labels of any length (including month/year suffixes).

### Keep the leading-space gap unhighlighted

**Decision:** The first character of each cell (`pos_in_cell == 0`) remains unhighlighted even when part of an extended label selection. However, label characters that land on `pos_in_cell == 0` of neighboring cells (positions after the origin cell) should still get the highlight since the label text is contiguous.

**Rationale:** Actually, re-examining: `has_bg = pos_in_cell > 0` currently prevents the leading space from being highlighted. For the extended label, the positions that are label characters (`day_is_label[pos] == true`) should get the highlight regardless of `pos_in_cell`, because the label text is what we want to visually emphasize. Only true gap spaces (not label characters) should remain unhighlighted.

**Final approach:** For positions where `day_label_origin` matches the selected/local hour AND `day_is_label[pos]` is true, apply the highlight. For non-label positions, keep the existing `has_bg` logic.

## Risks / Trade-offs

- **[Visual density]** Extending the highlight over the full label (which can be 15+ chars for month/year labels) creates a larger highlight area → Acceptable because it's only on the day row and matches user expectation.
- **[Edge case: label near timeline edge]** If a label is truncated at the right edge, the highlight still applies only to visible positions → No risk, handled naturally by the existing bounds check.
- **[Performance]** One extra `Vec<Option<i32>>` allocation per timezone block per frame → Negligible; already allocating `day_chars` and `day_is_label` of the same size.
