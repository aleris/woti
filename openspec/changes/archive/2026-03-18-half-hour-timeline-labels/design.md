## Context

The TUI renders each timezone as a 3-row block in `src/tui/render.rs`:

| Row | Left side (info column) | Right side (timeline) |
|-----|-------------------------|-----------------------|
| 1   | (padding)               | Day labels (`WED 18`) |
| 2   | City · TZ badge · offset · time | Hour numbers (`8  9 10 11`) |
| 3   | Region · date           | AM/PM or blank (`am pm am pm` / `            `) |

Each timeline cell is 3 columns: 1 space separator + 2-char content. `CELL_WIDTH = 3`.

In 24h mode, row 3's timeline cells are blank (`"  "`). In 12h mode they show `"am"` or `"pm"`.

The `TimelineParams` struct feeds all three span builders but currently carries no information about the timezone's minute offset. The offset is only computed in `build_info_line` for the `+5:30` text in the info column.

## Goals / Non-Goals

**Goals:**
- Communicate fractional-hour offsets (`:30`, `:45`) directly in the timeline for each hour cell.
- Fit within the existing 3-column cell layout — no layout or cell-width changes.
- Preserve all existing styling (selected highlight, local marker, dim).

**Non-Goals:**
- Changing how the info-column offset string (`+5:30`) is displayed.
- Supporting arbitrary minute offsets — only the real-world values 0, 30, 45 matter.
- Adding new rows or changing the 3-row block structure.

## Decisions

### 1. Encode minutes in the sub-row (row 3), not the hour row

**Rationale:** The hour row (row 2) has exactly 2 chars per cell for the hour number. There's no room for an extra symbol without widening cells, which would break vertical alignment across zones. Row 3 already has 2 chars of content that can be repurposed.

**Alternative considered:** Widening `CELL_WIDTH` to 4 for half-hour zones — rejected because it misaligns the timeline columns across different timezone rows.

### 2. Mode-specific formatting

| Mode | Current row 3 content | New content (half-hour tz) |
|------|----------------------|---------------------------|
| 24h  | `"  "` (blank)       | `"³⁰"` or `"⁴⁵"` (Unicode superscript digits) |
| 12h  | `"am"` / `"pm"`      | `"½a"` / `"½p"` (or `"¾a"` / `"¾p"` for :45)  |

**Rationale for 24h — superscript:** The sub-row is currently unused in 24h mode. Superscript digits (`³⁰`) are each 1 terminal column wide, fitting perfectly in the 2-char slot. They visually read as "minutes past" at a reduced weight.

**Rationale for 12h — combined glyph:** AM/PM info can't be dropped — it's essential in 12h mode. `½` is a single character (1 terminal column), leaving room for `a` or `p` as a compact meridiem indicator. This preserves both pieces of information in the same 2-char space.

**Alternative considered:** Using `½` in the hour row and full `am`/`pm` in sub-row — rejected because double-digit hours (10, 11, 12) have no room for `½` in the 2-char slot.

### 3. Pass `offset_m` through `TimelineParams`

Add an `offset_m: i32` field to `TimelineParams`, computed from `now_tz.offset().fix().local_minus_utc()` in `render_timezone_block`. The span builder reads it to decide which format to use. Value is 0 for whole-hour zones (no change in behavior).

### 4. Style the fractional indicator with the same logic as existing content

The superscript / `½` text uses the same `is_selected` / `is_local` / default style cascade already in `build_ampm_spans`, just applied to the new text. No new colors or modifiers.

## Risks / Trade-offs

- **[Terminal font support for superscript digits]** → Mitigation: `³` (U+00B3) and `⁰` (U+2070) are in virtually all monospace terminal fonts. Fall back to plain `30` if rendering issues are reported — but `30` already fits in 2 chars so this is a natural fallback.
- **[`½` display width]** → `½` (U+00BD) is consistently 1 column in monospace fonts per Unicode East Asian Width properties (it's Narrow). Verified against common terminal emulators.
- **[12h readability with `½a`]** → Trade-off: `½a` is less immediately obvious than `am`, but users of half-hour timezones already know their offset. The `½` is a visual cue, not primary information.
