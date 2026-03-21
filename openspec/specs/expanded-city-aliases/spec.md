### Requirement: US city aliases resolve to correct timezone

The system SHALL include city aliases for the following US cities, each mapping to the correct IANA timezone identifier:

**America/Los_Angeles (Pacific):** Sacramento (California), Oakland (California)
**America/Chicago (Central):** San Antonio (Texas), Nashville (Tennessee), Kansas City (Missouri), St. Louis (Missouri), New Orleans (Louisiana), Milwaukee (Wisconsin), Oklahoma City (Oklahoma), Memphis (Tennessee), Fort Worth (Texas), Omaha (Nebraska)
**America/New_York (Eastern):** Charlotte (North Carolina), Tampa (Florida), Orlando (Florida), Columbus (Ohio), Pittsburgh (Pennsylvania), Baltimore (Maryland), Raleigh (North Carolina), Jacksonville (Florida), Cleveland (Ohio)
**America/Denver (Mountain):** Albuquerque (New Mexico), Colorado Springs (Colorado), El Paso (Texas)
**America/Phoenix (no DST):** Tucson (Arizona)

#### Scenario: Central time US city lookup

- **WHEN** a user searches for "nashville"
- **THEN** the system SHALL return a resolved timezone with tz=America/Chicago, city="Nashville", region="United States, Tennessee"

#### Scenario: Mountain time Texas city lookup

- **WHEN** a user searches for "el paso"
- **THEN** the system SHALL return a resolved timezone with tz=America/Denver, city="El Paso", region="United States, Texas"

#### Scenario: Arizona city lookup (no DST)

- **WHEN** a user searches for "tucson"
- **THEN** the system SHALL return a resolved timezone with tz=America/Phoenix, city="Tucson", region="United States, Arizona"

### Requirement: European city aliases resolve to correct timezone

The system SHALL include city aliases for the following European cities:

**Europe/London:** Birmingham (United Kingdom), Liverpool (United Kingdom), Leeds (United Kingdom), Bristol (United Kingdom)
**Europe/Paris:** Nice (France), Toulouse (France), Bordeaux (France)
**Europe/Berlin:** Cologne (Germany), Stuttgart (Germany), Düsseldorf (Germany)
**Europe/Rome:** Naples (Italy), Turin (Italy), Florence (Italy), Venice (Italy), Bologna (Italy)
**Europe/Madrid:** Valencia (Spain), Seville (Spain), Málaga (Spain)
**Europe/Istanbul:** Izmir (Turkey)
**Europe/Brussels:** Antwerp (Belgium)
**Europe/Dublin:** Cork (Ireland)
**Europe/Athens:** Thessaloniki (Greece)
**Europe/Zurich:** Bern (Switzerland), Basel (Switzerland)
**Europe/Vienna:** Salzburg (Austria), Graz (Austria)
**Europe/Stockholm:** Gothenburg (Sweden)
**Europe/Kyiv:** Kharkiv (Ukraine), Lviv (Ukraine), Odesa (Ukraine)

#### Scenario: Italian city lookup

- **WHEN** a user searches for "florence"
- **THEN** the system SHALL return a resolved timezone with tz=Europe/Rome, city="Florence", region="Italy"

#### Scenario: UK city lookup

- **WHEN** a user searches for "birmingham"
- **THEN** the system SHALL return a resolved timezone with tz=Europe/London, city="Birmingham", region="United Kingdom"

#### Scenario: Swiss capital lookup

- **WHEN** a user searches for "bern"
- **THEN** the system SHALL return a resolved timezone with tz=Europe/Zurich, city="Bern", region="Switzerland"

### Requirement: Asian city aliases resolve to correct timezone

The system SHALL include city aliases for the following Asian cities:

**Asia/Tokyo:** Kyoto (Japan), Yokohama (Japan), Nagoya (Japan)
**Asia/Shanghai:** Hangzhou (China), Nanjing (China), Xi'an (China), Wuhan (China)
**Asia/Seoul:** Incheon (South Korea)
**Asia/Riyadh:** Mecca (Saudi Arabia), Medina (Saudi Arabia)
**Asia/Tehran:** Isfahan (Iran)
**Asia/Bangkok:** Chiang Mai (Thailand)
**Asia/Manila:** Cebu (Philippines)
**Asia/Jakarta:** Surabaya (Indonesia)
**Asia/Kuala_Lumpur:** Penang (Malaysia)

#### Scenario: Japanese cultural city lookup

- **WHEN** a user searches for "kyoto"
- **THEN** the system SHALL return a resolved timezone with tz=Asia/Tokyo, city="Kyoto", region="Japan"

#### Scenario: Chinese tech hub lookup

- **WHEN** a user searches for "hangzhou"
- **THEN** the system SHALL return a resolved timezone with tz=Asia/Shanghai, city="Hangzhou", region="China"

#### Scenario: Saudi holy city lookup

- **WHEN** a user searches for "mecca"
- **THEN** the system SHALL return a resolved timezone with tz=Asia/Riyadh, city="Mecca", region="Saudi Arabia"

### Requirement: Alias lookup is case-insensitive

All new city aliases SHALL be matched case-insensitively, consistent with existing behavior (the `name` field is stored lowercase and input is lowercased before comparison).

#### Scenario: Mixed-case input

- **WHEN** a user searches for "Nashville" or "NASHVILLE" or "nashville"
- **THEN** the system SHALL return the same resolved timezone result in all cases

### Requirement: New aliases do not conflict with existing entries

No new alias SHALL duplicate an existing alias `name` or collide with a city name already resolvable through IANA zone path matching.

#### Scenario: No duplicate alias names

- **WHEN** the full CITY_ALIASES array is inspected
- **THEN** every `name` field SHALL be unique across the entire array
