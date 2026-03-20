## Context

`uses_12h_clock()` in `src/tz_data.rs` determines whether a given IANA timezone should display 12-hour time in Mixed mode. It currently uses a hand-curated `TWELVE_HOUR_REGIONS` list of 16 country-name strings, matched via the `REGION_NAMES` display table (~90 zones). Zones not in `REGION_NAMES` fall through to ad-hoc prefix checks (`America/Indiana/`, `Australia/`, etc.). This is incomplete (misses ~55 territories that prefer 12h per CLDR) and inaccurate for some entries.

The authoritative data for this lives in two places:
1. **IANA `zone1970.tab`** — maps every IANA zone to its ISO 3166-1 alpha-2 country code(s)
2. **Unicode CLDR `supplemental/timeData`** — maps country codes to preferred hour cycle (`h` = 12h, `H` = 24h)

## Goals / Non-Goals

**Goals:**
- Cover all ~350 IANA zones in `chrono-tz::TZ_VARIANTS` with correct 12h/24h classification
- Source data from CLDR (the Unicode standard) rather than guesswork
- Zero new runtime dependencies
- Keep the `uses_12h_clock(iana_id: &str) -> bool` public API unchanged

**Non-Goals:**
- Locale-level overrides (e.g. `fr-CA` differs from `CA`) — this is a per-timezone tool, not per-locale
- Automatic CLDR data updates — tables will be refreshed manually as needed
- Replacing `REGION_NAMES` — that table serves display purposes and is unrelated

## Decisions

### 1. Two static `const` lookup tables

Add a `ZONE_COUNTRY` table mapping IANA zone names to ISO country codes, derived from `zone1970.tab`. For zones spanning multiple countries, use the first (most-populous) country code.

Add a `TWELVE_HOUR_COUNTRIES` table listing ISO country codes where CLDR `timeData._preferred` is `h` (any variant: `h`, `hB`, `hb`).

**Why not a `HashMap` or `phf`?** With ~350 entries, a linear scan of a sorted `const` slice with binary search is fast enough (sub-microsecond) and requires zero allocation or build dependencies. We can switch to `phf` later if profiling warrants it.

**Why not ICU4X?** The `icu` crate ecosystem would give automatic CLDR data, but adds 10+ transitive dependencies and megabytes of data blobs for a single boolean question. Massive overkill.

### 2. Trust CLDR as-is for `_preferred`

CLDR says GB and IE prefer 24h (`_preferred: "H"`). This differs from the current code (which lists them as 12h). CLDR's `_preferred` reflects the formal/written convention. We'll trust CLDR — users who disagree can press `f` to cycle to their preference, which persists in config.

### 3. Use binary search on sorted tables

Both `ZONE_COUNTRY` and `TWELVE_HOUR_COUNTRIES` will be sorted alphabetically at compile time. `uses_12h_clock()` will use `binary_search_by_key` for O(log n) lookup — ~9 comparisons for 350 entries.

### 4. Fallback for unknown zones

Zones not found in `ZONE_COUNTRY` (e.g. newly added IANA zones between updates) will default to 24h. This is the safer default since 24h is unambiguous.

## Risks / Trade-offs

- **Data staleness** — CLDR and IANA data evolve. New zones get added ~1-2 per year. Mitigation: the tables are plain `const` arrays in one file, easy to regenerate. A comment will document the CLDR version and date.
- **GB/IE behavior change** — Users with `Europe/London` in Mixed mode will see 24h instead of 12h after this change. Mitigation: Mixed mode was already a best-guess; users can force 12h with `f`. This is a correctness fix, not a regression.
- **Binary size** — ~350 `(&str, &str)` pairs add roughly 4-5 KB of const data. Negligible for a TUI binary.
