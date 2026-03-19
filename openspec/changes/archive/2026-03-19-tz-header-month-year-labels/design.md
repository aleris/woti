## Context

The TUI timeline renders a day label row above the hour numbers. When the timeline crosses midnight, `build_day_spans` in `src/tui/render.rs` inserts a label like `THU 19` at the midnight column. The copy-to-clipboard function in `src/tui/copy.rs` appends a similar day suffix (`WED 1`) when a timezone's selected hour falls on a different date than the reference timezone.

Currently neither surface indicates month or year transitions, which becomes confusing when scrolling far from the current date.

## Goals / Non-Goals

**Goals:**
- Add month name to day labels when the displayed date's month differs from today's month.
- Add year to day labels when the displayed date's year differs from today's year.
- Apply the same formatting rules in both the TUI timeline and copy output.
- Keep labels compact — same-month dates remain `DAY DD` with no extra text.

**Non-Goals:**
- Changing the day label positioning/alignment logic (stays as-is from `multi-column-day-label` spec).
- Internationalisation of month names (English only for now).
- Changing the date string on the info line (row 3 of the timezone block).

## Decisions

**1. Compare against "today in the local timezone" not UTC**

The reference point for "same month" is `now_tz` (today's date in each timezone). This means the comparison is per-timezone — each row uses its own local "today" to decide whether to show the month. This matches user expectations: the label tells you *that timezone* crossed a month boundary.

Alternative considered: compare against a single reference date (first timezone's today). Rejected because it would hide month transitions that only some timezones experience.

**2. Full month name, not abbreviated**

Use `April` instead of `Apr`. The day label row has plenty of horizontal space since labels only appear at midnight boundaries and the next label is 24 cells away. The full name is more readable.

**3. Comma-separated format: `DAY DD, Month` / `DAY DD, Month, YYYY`**

The month appears after a comma following the day number. The year, when shown, follows another comma after the month. This reads naturally and matches the user's examples.

**4. Copy output uses the same comma format**

The copy function currently appends ` WED 1` as a suffix. The new format becomes ` WED 1, April` or ` WED 1, April, 2027`. The reference date for month/year comparison in copy is the first timezone's date (existing behavior).

## Risks / Trade-offs

- **[Label overflow]** → Long labels like `WED 1, September, 2027` (23 chars) could overflow the timeline edge. Existing truncation logic in `build_day_spans` already handles this — labels are clipped at `total_day_chars`.
- **[Visual density]** → Extra text on the day row could feel busy. Mitigated by only showing month/year when it actually changes — most usage shows the compact `DAY DD` format.
