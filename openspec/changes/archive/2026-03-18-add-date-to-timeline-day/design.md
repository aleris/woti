## Context

The timeline in `src/tui.rs` renders three rows (day markers, hours, am/pm) by iterating over cells column-by-column. Each column is `CELL_WIDTH = 3` characters wide. Day markers currently show only the short weekday name (e.g., "WED"), which fits in a single 3-char column. The spec requires "WED 18" format (6 chars), which spans at least 2 columns.

The current rendering builds `day_spans`, `hour_spans`, and `ampm_spans` vectors in a single loop over `num_cells`, pushing one `Span` per cell. The day row then composes these spans into a `Line` and renders it as a `Paragraph`.

## Goals / Non-Goals

**Goals:**
- Display day markers as "DAY DD" (e.g., "WED 18") on the timeline's first row
- Maintain correct alignment between the day row and the hour/ampm rows
- Preserve all existing styling (highlight for selected hour, magenta for day labels, local hour styling)

**Non-Goals:**
- Changing `CELL_WIDTH` — it stays at 3; only the day label rendering changes
- Modifying hour or AM/PM row rendering

## Decisions

### Decision 1: Build day row as a flat character buffer, then convert to styled spans

**Approach**: Instead of building day spans column-by-column (one `Span` per cell), build the day row using a parallel data structure that tracks the character content and style for each character position across the full timeline width. After the cell loop, convert this buffer into `Span`s.

**Rationale**: The day label "WED 18" is 6 characters and must overwrite positions belonging to 2+ cells. Building a character buffer lets us write the label starting at the column where `hour_in_day == 0` and extending as far as needed, without worrying about cell boundaries. The hour and ampm rows remain column-by-column since their content always fits within `CELL_WIDTH`.

**Alternative considered**: Increasing `CELL_WIDTH` to 6+ — rejected because it would halve the number of visible hours, degrading the timeline's information density for all rows.

**Alternative considered**: Pre-scanning to compute day label positions before the loop — adds complexity with no real benefit over the buffer approach.

### Decision 2: Character buffer uses `(char, Style)` tuples

Each position in the buffer stores a character and its style. Day labels write their characters with the appropriate style (magenta/bold for normal, cell_style for selected/local). Non-label positions default to space with the cell's background style. After the loop, consecutive characters with the same style are grouped into `Span`s.

### Decision 3: Day label format is `"%a"` uppercased + space + `"%e"` trimmed

Use `dt.format("%a")` uppercased for the day name and `dt.day()` for the numeric date. This produces "WED 18", "THU 1", etc. The label is written starting at the cell's character offset with no padding — it simply overwrites as many positions as the string occupies.

## Risks / Trade-offs

- **[Overlap at boundaries]** If two day changes occur within 2 cells of each other (impossible in practice since days are 24h apart and we show ~40 cells), labels could overlap. No mitigation needed — the minimum gap between day changes is 24 cells.
- **[Label truncation at edges]** A day label near the right edge of the timeline could extend beyond the available width. Mitigation: truncate the label to fit the remaining character positions.
