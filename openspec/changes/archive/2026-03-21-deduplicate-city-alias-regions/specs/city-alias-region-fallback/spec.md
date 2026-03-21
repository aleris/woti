## ADDED Requirements

### Requirement: CityAlias region resolved via optional override with REGION_NAMES fallback

The `CityAlias` struct SHALL have a `region_override` field of type `Option<&'static str>`. When `region_override` is
`Some(value)`, `lookup_city` SHALL use that value as the resolved region. When `region_override` is `None`,
`lookup_city` SHALL resolve the region by calling `city_and_region(alias.iana_id)`, which consults `REGION_NAMES` and
falls back to the IANA continent prefix.

#### Scenario: Alias with no override uses REGION_NAMES

- **WHEN** `lookup_city("Mumbai")` is called and the alias for Mumbai has `region_override: None`
- **THEN** the returned `ResolvedTz.region` SHALL equal `"India"` (from `REGION_NAMES` entry for `Asia/Kolkata`)

#### Scenario: Alias with override uses the override value

- **WHEN** `lookup_city("Seattle")` is called and the alias for Seattle has
  `region_override: Some("United States, Washington")`
- **THEN** the returned `ResolvedTz.region` SHALL equal `"United States, Washington"` (not `"United States, California"`
  from `REGION_NAMES`)

#### Scenario: Alias whose IANA zone was previously missing from REGION_NAMES

- **WHEN** `lookup_city("Tirana")` is called and `Europe/Tirane` has been added to `REGION_NAMES` as `"Albania"`
- **THEN** the returned `ResolvedTz.region` SHALL equal `"Albania"` with `region_override: None`

### Requirement: REGION_NAMES covers all IANA zones referenced by aliases

Every `iana_id` referenced in `CITY_ALIASES` that uses `region_override: None` SHALL have a corresponding entry in
`REGION_NAMES`. This ensures no alias silently falls back to a bare continent prefix.

#### Scenario: No alias falls through to continent-only fallback

- **WHEN** every alias entry with `region_override: None` is checked
- **THEN** `REGION_NAMES` SHALL contain an entry for that alias's `iana_id`

### Requirement: Field rename and documentation

The struct field SHALL be named `region_override` (not `display_region`). The field SHALL have a doc comment explaining
the fallback semantics. The `CITY_ALIASES` table SHALL include section comments distinguishing entries that use `None`
from entries that carry an override.

#### Scenario: Struct field is named region_override

- **WHEN** the `CityAlias` struct is inspected
- **THEN** it SHALL have a field `region_override: Option<&'static str>` and no field named `display_region`

### Requirement: Output parity with previous implementation

All existing `lookup_city` calls SHALL return identical `(city, region)` pairs as before this change. No user-visible
behaviour changes.

#### Scenario: Existing test assertions remain valid

- **WHEN** the existing test suite (`mod tests` in `tz_data.rs`) is executed
- **THEN** all tests SHALL pass without modification to assertions
