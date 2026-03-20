## Why

The timezone abbreviation map in `src/tz_data.rs` is built by sampling winter and summer dates hardcoded to 2025. If
IANA publishes rule changes for future years (e.g., a country abolishes or adopts DST), the map will not reflect those
changes because it is pinned to 2025. Using the current year keeps the map accurate as long as the binary is rebuilt.

## What Changes

- Replace the hardcoded `2025` year in `build_abbreviation_map()` with `Utc::now().year()` so the winter/summer sample
  dates always use the current year's DST rules.

## Capabilities

### New Capabilities

- `dynamic-tz-abbrev-year`: Timezone abbreviation map uses the current year instead of a hardcoded year for DST sampling
  dates.

### Modified Capabilities

(none)

## Impact

- **Code**: `src/tz_data.rs` — `build_abbreviation_map()` function (lines 48-58).
- **Dependencies**: Adds a `chrono::Utc` import (already available via the `chrono` crate in scope).
- **Behavior**: The abbreviation map will reflect the current year's DST rules on each build of the `LazyLock` static,
  which happens once per process startup.
