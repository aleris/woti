### Requirement: Zone-to-country lookup table

The system SHALL maintain a static `ZONE_COUNTRY` table mapping IANA timezone identifiers to ISO 3166-1 alpha-2 country
codes, sourced from the IANA `zone1970.tab` file. The table MUST be sorted alphabetically by IANA zone name. For zones
that span multiple countries, the first (most-populous) country code from `zone1970.tab` SHALL be used.

#### Scenario: Known zone resolves to country code

- **WHEN** `uses_12h_clock` is called with `"America/New_York"`
- **THEN** the zone resolves to country code `"US"` and the hour cycle for `US` is used

#### Scenario: Known zone in multi-country entry

- **WHEN** `uses_12h_clock` is called with `"Asia/Dubai"` (which covers AE, OM, RE, SC, TF in zone1970.tab)
- **THEN** the zone resolves to the first-listed country code `"AE"`

#### Scenario: Unknown zone defaults to 24h

- **WHEN** `uses_12h_clock` is called with an IANA zone not present in `ZONE_COUNTRY`
- **THEN** the function returns `false` (24h preference)

### Requirement: CLDR-based 12-hour country classification

The system SHALL maintain a static `TWELVE_HOUR_COUNTRIES` table listing ISO 3166-1 alpha-2 country codes whose
preferred hour cycle is 12-hour, sourced from Unicode CLDR `supplemental/timeData` where `_preferred` is `h` (any
variant). The table MUST be sorted alphabetically.

#### Scenario: US timezone detected as 12h

- **WHEN** `uses_12h_clock` is called with `"America/Chicago"`
- **THEN** it returns `true` because `US` is in `TWELVE_HOUR_COUNTRIES`

#### Scenario: German timezone detected as 24h

- **WHEN** `uses_12h_clock` is called with `"Europe/Berlin"`
- **THEN** it returns `false` because `DE` is not in `TWELVE_HOUR_COUNTRIES`

#### Scenario: GB timezone detected as 24h per CLDR

- **WHEN** `uses_12h_clock` is called with `"Europe/London"`
- **THEN** it returns `false` because CLDR classifies `GB` as `_preferred: "H"` (24h)

#### Scenario: Australian timezone detected as 12h

- **WHEN** `uses_12h_clock` is called with `"Australia/Sydney"`
- **THEN** it returns `true` because `AU` is in `TWELVE_HOUR_COUNTRIES`

#### Scenario: South Korean timezone detected as 12h

- **WHEN** `uses_12h_clock` is called with `"Asia/Seoul"`
- **THEN** it returns `true` because `KR` is in `TWELVE_HOUR_COUNTRIES`

### Requirement: Replaces legacy hardcoded data

The `TWELVE_HOUR_REGIONS` constant and the ad-hoc `America/Indiana/`, `America/Kentucky/`, `America/North_Dakota/`,
`Australia/` prefix fallback logic SHALL be removed. The `REGION_NAMES` table SHALL remain unchanged (it serves
display-name purposes only).

#### Scenario: Indiana zones covered by zone table

- **WHEN** `uses_12h_clock` is called with `"America/Indiana/Indianapolis"`
- **THEN** it returns `true` via the `ZONE_COUNTRY` table (maps to `US`) without any prefix-based fallback

### Requirement: Public API unchanged

The function signature `pub fn uses_12h_clock(iana_id: &str) -> bool` SHALL remain unchanged. Callers (`time_format.rs`)
MUST NOT require any modifications.

#### Scenario: Mixed mode continues to work

- **WHEN** `TimeFormat::Mixed` is active and `use_24h_for_tz` calls `tz_data::uses_12h_clock`
- **THEN** the call succeeds with the same signature and return type as before
