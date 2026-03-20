## Why

`uses_12h_clock()` in `tz_data.rs` relies on a hand-curated `TWELVE_HOUR_REGIONS` list of 16 country names matched against the `REGION_NAMES` display-string table. This has two problems: it only covers the ~90 zones in `REGION_NAMES` (falling through to ad-hoc prefix checks for others), and the list itself is incomplete — CLDR identifies 70+ territories that prefer 12-hour time while the current list has 16. Some entries also disagree with the authoritative Unicode CLDR `timeData` (e.g. GB and IE are listed as 12h but CLDR says `_preferred: "H"`).

## What Changes

- Replace `TWELVE_HOUR_REGIONS` (country-name strings) and the `REGION_NAMES`-based lookup in `uses_12h_clock()` with two static `const` tables derived from authoritative sources:
  1. **IANA zone → ISO 3166-1 country code** (from the IANA `zone1970.tab`, ~350 entries)
  2. **12-hour-preferring country codes** (from Unicode CLDR `timeData`, territories where `_preferred` is `h`)
- Remove the ad-hoc `America/Indiana/`, `America/Kentucky/`, `America/North_Dakota/`, `Australia/` prefix fallback — these are covered by the zone→country table.
- `REGION_NAMES` itself is untouched — it serves display purposes and is unrelated to clock detection.

## Capabilities

### New Capabilities

- `cldr-hour-cycle`: Determine 12h vs 24h clock preference for any IANA timezone using authoritative CLDR data via static lookup tables

### Modified Capabilities

_None — the existing `uses_12h_clock()` public API signature is unchanged; only the implementation and accuracy improve._

## Impact

- **Code**: `src/tz_data.rs` — replace `TWELVE_HOUR_REGIONS`, `uses_12h_clock()`, add `ZONE_COUNTRY` and `TWELVE_HOUR_COUNTRIES` tables
- **Behavior**: Some timezones will flip 12h↔24h in Mixed mode to match CLDR data (e.g. GB→24h, added KR/AR/PE→12h)
- **Dependencies**: None added
- **Binary size**: ~4-5 KB additional const data for the zone→country table
