## Why

The `CITY_ALIASES` table in `src/tz_data.rs` has uneven coverage across regions. Many globally recognizable cities are
missing — users searching for "Nashville", "Florence", "Kyoto", or "Charlotte" get no results, while smaller cities in
some countries are well-represented. Expanding the alias table improves the out-of-the-box experience for city-based
timezone lookup.

## What Changes

- Add ~50 new city aliases to the `CITY_ALIASES` static array in `src/tz_data.rs`, covering major gaps across three
  continents:
    - **United States** (~15 cities): San Antonio, Nashville, Charlotte, Tampa, Orlando, Columbus, Kansas City,
      Sacramento, Pittsburgh, Baltimore, St. Louis, New Orleans, Albuquerque, Tucson, El Paso
    - **Europe** (~20 cities): Birmingham, Liverpool, Naples, Florence, Venice, Turin, Valencia, Seville, Cologne, Nice,
      Toulouse, Thessaloniki, Antwerp, Bern, Cork, Izmir, Gothenburg, Kharkiv, Lviv, Salzburg
    - **Asia** (~15 cities): Kyoto, Yokohama, Nagoya, Hangzhou, Nanjing, Xi'an, Wuhan, Incheon, Mecca, Medina, Isfahan,
      Chiang Mai, Cebu, Surabaya, Penang
- Each alias maps to the correct IANA timezone identifier with appropriate display city and region strings
- No changes to lookup logic, data structures, or external APIs

## Capabilities

### New Capabilities

- `expanded-city-aliases`: Adds ~50 city alias entries for major cities in the US, Europe, and Asia that are currently
  missing from the lookup table

### Modified Capabilities

_None — this is purely additive data, no existing behavior changes._

## Impact

- **Code**: Only `src/tz_data.rs` is modified (the `CITY_ALIASES` static array)
- **Binary size**: Marginal increase from additional static string data (~3-4 KB)
- **Tests**: Existing tests remain valid; may want to add spot-check tests for a few new aliases
- **Dependencies**: None
