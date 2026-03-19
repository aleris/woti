## Context

Each timezone block renders three rows. Row 0 shows day labels (e.g., "MON 5") at midnight boundaries. Row 1 shows hour numbers. Both rows share a character grid where each cell is `CELL_WIDTH` (3) characters wide: 1 gap space + 2 hour-digit characters.

Currently `build_day_spans` places the day label at `start_pos = i * cell_w` — the cell boundary, which is the gap-space column. The hour digits on row 1 start at `i * cell_w + 1`. This 1-character offset makes the day label appear shifted left relative to the numbers below.

## Goals / Non-Goals

**Goals:**
- Align the day label so its first character sits above the first character of the 2-char hour-digit field (position `cell_start + 1`).
- Ensure both single-digit dates ("MON 5", 5 chars) and double-digit dates ("TUE 12", 6 chars) render correctly at the new offset without overflowing or losing characters.

**Non-Goals:**
- Changing the cell width, hour-number formatting, or overall timeline layout.
- Modifying the copy-text alignment (separate logic in `copy.rs`).

## Decisions

**Shift `start_pos` by +1 in `build_day_spans`**
Change the label start from `(i as usize) * p.cell_w` to `(i as usize) * p.cell_w + 1`. This aligns the label's first character with the first character of `{:>2}` hour number in the row below. Simple, single-line change with no structural impact.

Alternatives considered: adjusting the gap space in `build_hour_spans` instead — rejected because it would affect cell width or overall timeline spacing for all rows.

## Risks / Trade-offs

- [Label truncation at right edge] → The +1 shift pushes the label 1 char further right. The existing bounds check (`if pos < total_day_chars`) already handles truncation at the timeline boundary, so labels near the right edge will simply lose their last character instead of overflowing.
