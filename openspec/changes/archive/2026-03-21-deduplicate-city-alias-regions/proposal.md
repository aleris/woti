## Why

`CITY_ALIASES` and `REGION_NAMES` in `src/tz_data.rs` both store region display strings. About 65% of alias entries
repeat the exact value that `REGION_NAMES` already provides for their `iana_id`. This duplication is a maintenance
burden — adding a new alias for a city that shares an existing zone requires copying the region string, and changes to
region formatting must be updated in two places.

## What Changes

- Rename `display_region` to `region_override` on `CityAlias` and change its type to `Option<&'static str>`. `None`
  means "look up from `REGION_NAMES`"; `Some(...)` means "this alias needs a region different from what the IANA zone's
  entry says" (e.g. Seattle is in Washington, not California).
- Add doc comments to `CityAlias` fields and section comments in the `CITY_ALIASES` table to clarify why some entries
  carry an override and others don't.
- Add 3 missing IANA zones to `REGION_NAMES` (`Europe/Tirane`, `America/Guayaquil`, `Asia/Makassar`) so their aliases
  can also use `None`.
- Update `lookup_city` to resolve the region through fallback: `region_override` → `REGION_NAMES` lookup → IANA
  continent prefix.

## Capabilities

### New Capabilities

- `city-alias-region-fallback`: Make `CityAlias` region optional with automatic fallback to `REGION_NAMES`, reducing
  duplication and adding clarity via renamed field and comments.

### Modified Capabilities

_(none — no spec-level behavior changes, only internal data representation)_

## Impact

- **Code**: `src/tz_data.rs` — `CityAlias` struct, `CITY_ALIASES` table, `REGION_NAMES` table, `lookup_city` function.
- **Behavior**: No user-visible change. Every city lookup returns the same `(city, region)` pair as before.
- **Tests**: Existing tests in `mod tests` validate the output and should continue to pass without modification.
