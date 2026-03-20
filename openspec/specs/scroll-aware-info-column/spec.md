### Requirement: Info column reflects selected datetime when scrolled

When the user has scrolled the timeline (`hour_offset != 0`), the info column for each timezone row SHALL derive its display values from the selected datetime rather than the current wall-clock time. The selected datetime is computed by applying `hour_offset` to the current time in that timezone.

The affected display values are:
- Timezone abbreviation (e.g., "EST" vs "EDT")
- Displayed time
- Displayed date
- UTC offset badge (e.g., "-5" vs "-4")
- Superscript minutes indicator in `TimelineParams.offset_m`

#### Scenario: Scrolling across a DST boundary updates the abbreviation

- **WHEN** the user is viewing a timezone that observes DST (e.g., America/New_York) and scrolls the timeline past a DST transition
- **THEN** the timezone abbreviation in the info column SHALL change to reflect the abbreviation at the selected hour (e.g., from "EST" to "EDT" or vice versa)

#### Scenario: Scrolling across a DST boundary updates the UTC offset badge

- **WHEN** the user scrolls past a DST transition for a given timezone
- **THEN** the UTC offset badge SHALL display the offset at the selected hour (e.g., "-4" instead of "-5" for EDT vs EST)

#### Scenario: Scrolling across a date boundary updates the date

- **WHEN** the user scrolls the timeline such that the selected hour falls on a different calendar day
- **THEN** the date shown in the info column SHALL reflect the selected day, not the current day

#### Scenario: Sub-hour DST zones show correct superscript minutes

- **WHEN** the user scrolls past a DST transition for a timezone with sub-hour offset changes (e.g., Australia/Lord_Howe: +10:30 standard, +11:00 DST)
- **THEN** `TimelineParams.offset_m` SHALL reflect the minutes component of the UTC offset at the selected hour

### Requirement: Info column shows current time when not scrolled

When `hour_offset == 0`, the info column SHALL continue to derive all display values from the current wall-clock time (`now_tz`), preserving existing behavior.

#### Scenario: No scroll shows real-time values

- **WHEN** the user has not scrolled (`hour_offset == 0`)
- **THEN** the info column SHALL display the current timezone abbreviation, current time, current date, and current UTC offset
