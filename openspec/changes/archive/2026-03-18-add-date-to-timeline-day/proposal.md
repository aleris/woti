## Why

The original spec requires day markers to show both the day name and numeric date (e.g., "WED 18"), but the current implementation only renders the short day name ("WED"). This makes it harder to orient in time when viewing the timeline, especially when it crosses multiple days.

## What Changes

- Modify the timeline day marker (row 1) to display "DAY DD" format (e.g., "WED 18") instead of just "DAY"
- Switch from column-by-column rendering for the day row to a row-based approach, since "WED 18" (6 chars) spans more than one 3-char column
- Preserve proper alignment with the hour and AM/PM rows which continue to render per-column

## Capabilities

### New Capabilities
- `multi-column-day-label`: Render day marker labels that span across multiple 3-char timeline columns, with correct alignment and styling

### Modified Capabilities

## Impact

- `src/tui.rs`: Day marker rendering logic in the timeline strip (lines ~306-324 and ~362-365)
- No API or dependency changes
- Visual change only — no configuration or data model impact
