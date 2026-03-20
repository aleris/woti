## ADDED Requirements

### Requirement: Abbreviation map uses current year for DST sampling
The abbreviation map SHALL derive its winter and summer sample dates from the current UTC year at initialization time, rather than a hardcoded year.

#### Scenario: Map built in 2026
- **WHEN** the process starts and the abbreviation map is initialized in the year 2026
- **THEN** the winter sample date SHALL be January 15, 2026 and the summer sample date SHALL be July 15, 2026

#### Scenario: Map built in any future year
- **WHEN** the process starts in year Y
- **THEN** the abbreviation map SHALL use Y as the year for both winter and summer sample dates, reflecting that year's DST rules as defined by the compiled IANA database
