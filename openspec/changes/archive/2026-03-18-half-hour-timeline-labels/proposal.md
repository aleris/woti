## Why

Timezones with half-hour offsets (e.g., India +5:30, Newfoundland -3:30, Nepal +5:45) currently show rounded whole-hour labels on the timeline, losing the :30/:45 minute component. Users comparing times across zones can't tell that Bangalore's "9" really means 9:30, making cross-zone comparisons misleading.

## What Changes

- The sub-row beneath the hour labels (currently used for am/pm in 12h mode, blank in 24h mode) will indicate the fractional-hour offset for half-hour/quarter-hour timezones.
- **24h mode**: The blank sub-row shows Unicode superscript minutes (e.g., `³⁰` or `⁴⁵`) for each hour cell when the timezone has a non-zero minute offset.
- **12h mode**: The 2-char am/pm cell becomes a combined indicator — `½a`/`½p` (or `¾a`/`¾p` for :45 offsets) — packing the fractional marker and the am/pm meridiem into the existing cell width.
- `TimelineParams` gains the timezone's minute offset so downstream span builders can detect half-hour zones.
- Whole-hour timezones are completely unaffected.

## Capabilities

### New Capabilities
- `half-hour-timeline-indicator`: Display fractional-hour markers in the timeline sub-row for timezones with non-zero minute offsets.

### Modified Capabilities

(none)

## Impact

- `src/tui/render.rs`: `TimelineParams` struct, `build_ampm_spans`, and `render_timezone_block` (to pass offset minutes).
- No new dependencies — uses only Unicode literals already supported by ratatui's `Span`.
- No breaking changes to existing whole-hour timezone rendering.
