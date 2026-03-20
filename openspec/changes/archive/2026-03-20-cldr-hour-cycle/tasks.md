## 1. Add ZONE_COUNTRY lookup table

- [x] 1.1 Add a sorted `const ZONE_COUNTRY: &[(&str, &str)]` table to `src/tz_data.rs` mapping every IANA zone from `zone1970.tab` to its primary ISO 3166-1 alpha-2 country code (~350 entries). Include a comment noting the source and date (IANA tzdata 2025a / zone1970.tab).

## 2. Add TWELVE_HOUR_COUNTRIES table

- [x] 2.1 Add a sorted `const TWELVE_HOUR_COUNTRIES: &[&str]` table to `src/tz_data.rs` listing all country codes where CLDR `timeData._preferred` is `h` (any variant). Include a comment noting the source (CLDR v48 timeData.json).

## 3. Rewrite uses_12h_clock

- [x] 3.1 Rewrite `uses_12h_clock()` to use `binary_search_by_key` on `ZONE_COUNTRY` to resolve the IANA zone to a country code, then `binary_search` on `TWELVE_HOUR_COUNTRIES` to check if that country prefers 12h. Return `false` for unknown zones.
- [x] 3.2 Remove the `TWELVE_HOUR_REGIONS` constant entirely.
- [x] 3.3 Remove the ad-hoc `America/Indiana/`, `America/Kentucky/`, `America/North_Dakota/`, `Australia/` prefix fallback from `uses_12h_clock()`.

## 4. Update README

- [x] 4.1 Add a short note to the README (under TUI controls or a new section) explaining that Mixed mode uses Unicode CLDR data for 12h/24h detection per timezone country, and that users can override with `f` if their preference differs.

## 5. Update tests

- [x] 5.1 Add tests for `uses_12h_clock` covering: US zone (12h), DE zone (24h), GB zone (24h per CLDR), AU zone (12h), KR zone (12h), Indiana sub-zone (12h via table), unknown zone (defaults 24h).
- [x] 5.2 Verify existing tests in `src/tz_data.rs` still pass — none should reference `TWELVE_HOUR_REGIONS` directly since it was a private const.
