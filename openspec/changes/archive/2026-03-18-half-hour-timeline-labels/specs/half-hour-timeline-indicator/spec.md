## ADDED Requirements

### Requirement: TimelineParams carries timezone minute offset
The `TimelineParams` struct SHALL include an `offset_m` field representing the minute component (0, 30, or 45) of the timezone's UTC offset. It SHALL be computed from `now_tz.offset().fix().local_minus_utc()` and passed to all timeline span builders.

#### Scenario: Whole-hour timezone
- **WHEN** a timezone has offset +5:00 (offset_m = 0)
- **THEN** `TimelineParams.offset_m` is 0

#### Scenario: Half-hour timezone
- **WHEN** a timezone has offset +5:30 (offset_m = 30)
- **THEN** `TimelineParams.offset_m` is 30

#### Scenario: Quarter-hour timezone
- **WHEN** a timezone has offset +5:45 (offset_m = 45)
- **THEN** `TimelineParams.offset_m` is 45

### Requirement: Sub-row shows superscript minutes in 24h mode for fractional-offset timezones
When `use_24h` is true and `offset_m` is non-zero, the sub-row (built by `build_ampm_spans`) SHALL display the minute offset as Unicode superscript digits in each cell's 2-char content slot.

#### Scenario: 24h mode with 30-minute offset
- **WHEN** `use_24h` is true and `offset_m` is 30
- **THEN** each sub-row cell displays `"³⁰"` (U+00B3 U+2070) instead of `"  "`

#### Scenario: 24h mode with 45-minute offset
- **WHEN** `use_24h` is true and `offset_m` is 45
- **THEN** each sub-row cell displays `"⁴⁵"` (U+2074 U+2075) instead of `"  "`

#### Scenario: 24h mode with whole-hour offset
- **WHEN** `use_24h` is true and `offset_m` is 0
- **THEN** each sub-row cell displays `"  "` (unchanged behavior)

### Requirement: Sub-row shows combined fraction + meridiem in 12h mode for fractional-offset timezones
When `use_24h` is false and `offset_m` is non-zero, the sub-row SHALL display a fraction symbol followed by a single-character meridiem indicator in each cell's 2-char content slot.

#### Scenario: 12h mode with 30-minute offset, AM hour
- **WHEN** `use_24h` is false, `offset_m` is 30, and the hour is before noon
- **THEN** the sub-row cell displays `"½a"`

#### Scenario: 12h mode with 30-minute offset, PM hour
- **WHEN** `use_24h` is false, `offset_m` is 30, and the hour is noon or after
- **THEN** the sub-row cell displays `"½p"`

#### Scenario: 12h mode with 45-minute offset, AM hour
- **WHEN** `use_24h` is false, `offset_m` is 45, and the hour is before noon
- **THEN** the sub-row cell displays `"¾a"`

#### Scenario: 12h mode with 45-minute offset, PM hour
- **WHEN** `use_24h` is false, `offset_m` is 45, and the hour is noon or after
- **THEN** the sub-row cell displays `"¾p"`

#### Scenario: 12h mode with whole-hour offset
- **WHEN** `use_24h` is false and `offset_m` is 0
- **THEN** the sub-row cell displays `"am"` or `"pm"` (unchanged behavior)

### Requirement: Fractional indicator styling matches existing sub-row styling
The fractional-hour indicator text SHALL use the same style cascade as the current am/pm text: selected_style when the cell is selected, local_style with DarkGray foreground when the cell is the local hour, and DarkGray + DIM otherwise.

#### Scenario: Selected cell with fractional indicator
- **WHEN** a half-hour timezone cell is the selected hour
- **THEN** the sub-row indicator uses `selected_style()` (black on yellow, bold)

#### Scenario: Local cell with fractional indicator
- **WHEN** a half-hour timezone cell is the local hour and hour_offset != 0
- **THEN** the sub-row indicator uses `local_style()` with DarkGray foreground
