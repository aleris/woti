## MODIFIED Requirements

### Requirement: Sub-row shows superscript minutes in 24h mode for fractional-offset timezones
When `use_24h` is true and `offset_m` is non-zero, the sub-row (built by `build_ampm_spans`) SHALL display the minute offset as Unicode superscript digits in each hour cell's 2-char content slot. With a sub-hour active interval (see `sub-hour-nav-interval`), intermediate (non-hour) cells SHALL display the wall-clock minute of the cell's datetime as superscript digits, independent of `offset_m`.

#### Scenario: 24h mode with 30-minute offset, 60-minute interval
- **WHEN** `use_24h` is true, `offset_m` is 30, and the active interval is 60 minutes
- **THEN** each sub-row cell displays `"³⁰"` (U+00B3 U+2070) instead of `"  "`

#### Scenario: 24h mode with 45-minute offset, 60-minute interval
- **WHEN** `use_24h` is true, `offset_m` is 45, and the active interval is 60 minutes
- **THEN** each sub-row cell displays `"⁴⁵"` (U+2074 U+2075) instead of `"  "`

#### Scenario: 24h mode with whole-hour offset, 60-minute interval
- **WHEN** `use_24h` is true, `offset_m` is 0, and the active interval is 60 minutes
- **THEN** each sub-row cell displays `"  "` (unchanged behavior)

#### Scenario: 24h mode with 45-minute offset, sub-hour interval
- **WHEN** `use_24h` is true, `offset_m` is 45, and the active interval is 15 minutes
- **THEN** hour-cell sub-row content remains `"⁴⁵"` and intermediate cells display the wall-clock minute as superscript (e.g. `"⁰⁰"`, `"¹⁵"`, `"³⁰"`). The `"⁰⁰"` on the intermediate `:00` cell marks the natural hour boundary the `·` tick on the row above does not.

#### Scenario: 24h mode with whole-hour offset, sub-hour interval (blank hour cell)
- **WHEN** `use_24h` is true, `offset_m` is 0, and the active interval is 15 minutes
- **THEN** hour-cell sub-row content stays `"  "` (unchanged from the 60-minute interval — the row-above hour digit marks the slot for stronger visual contrast) and intermediate cells display `"¹⁵"`, `"³⁰"`, `"⁴⁵"`

### Requirement: Sub-row shows combined fraction + meridiem in 12h mode for fractional-offset timezones
When `use_24h` is false and `offset_m` is non-zero, the sub-row SHALL display a fraction symbol followed by a single-character meridiem indicator in each hour cell's 2-char content slot. With a sub-hour active interval, intermediate (non-hour) cells SHALL display the wall-clock minute of the cell's datetime as superscript digits in 12h mode as well.

#### Scenario: 12h mode with 30-minute offset, AM hour, 60-minute interval
- **WHEN** `use_24h` is false, `offset_m` is 30, the hour is before noon, and the active interval is 60 minutes
- **THEN** the sub-row cell displays `"½a"`

#### Scenario: 12h mode with 30-minute offset, PM hour, 60-minute interval
- **WHEN** `use_24h` is false, `offset_m` is 30, the hour is noon or after, and the active interval is 60 minutes
- **THEN** the sub-row cell displays `"½p"`

#### Scenario: 12h mode with 45-minute offset, AM hour, 60-minute interval
- **WHEN** `use_24h` is false, `offset_m` is 45, the hour is before noon, and the active interval is 60 minutes
- **THEN** the sub-row cell displays `"¾a"`

#### Scenario: 12h mode with 45-minute offset, PM hour, 60-minute interval
- **WHEN** `use_24h` is false, `offset_m` is 45, the hour is noon or after, and the active interval is 60 minutes
- **THEN** the sub-row cell displays `"¾p"`

#### Scenario: 12h mode with whole-hour offset, 60-minute interval
- **WHEN** `use_24h` is false, `offset_m` is 0, and the active interval is 60 minutes
- **THEN** the sub-row cell displays `"am"` or `"pm"` (unchanged behavior)

#### Scenario: 12h mode with whole-hour offset, sub-hour interval
- **WHEN** `use_24h` is false, `offset_m` is 0, and the active interval is 15 minutes
- **THEN** hour-cell sub-row content remains `"am"`/`"pm"` and intermediate cells display superscript wall-clock minutes (`"¹⁵"`, `"³⁰"`, `"⁴⁵"`)

#### Scenario: 12h mode with 30-minute offset, sub-hour interval
- **WHEN** `use_24h` is false, `offset_m` is 30, and the active interval is 15 minutes
- **THEN** hour-cell sub-row content remains `"½a"`/`"½p"` and intermediate cells display superscript wall-clock minutes (e.g. `"⁴⁵"`, `"⁰⁰"`, `"¹⁵"`)
