## 1. Struct and REGION_NAMES updates

- [x] 1.1 Rename `display_region` to `region_override` on `CityAlias` and change its type to `Option<&'static str>`. Add doc comment explaining the fallback semantics.
- [x] 1.2 Add missing IANA zones to `REGION_NAMES`: `("Europe/Tirane", "Albania")`, `("America/Guayaquil", "Ecuador")`, `("Asia/Makassar", "Indonesia")`.

## 2. Update CITY_ALIASES table

- [x] 2.1 Convert all alias entries whose `display_region` matches `REGION_NAMES[iana_id]` to `region_override: None`. Add section comments distinguishing fallback entries from override entries.
- [x] 2.2 Convert alias entries that need a custom region to `region_override: Some("...")` (US state/province overrides, Canadian province overrides, Edinburgh/Glasgow Scotland, Canberra ACT).

## 3. Update lookup_city

- [x] 3.1 Update `lookup_city` to resolve region via `region_override.map(…).unwrap_or_else(|| city_and_region(alias.iana_id).1)`.

## 4. Verify

- [x] 4.1 Run `cargo test` and confirm all existing tests pass with no assertion changes.
