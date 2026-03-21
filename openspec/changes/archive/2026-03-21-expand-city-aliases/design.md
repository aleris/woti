## Context

`src/tz_data.rs` contains a `CITY_ALIASES` static array that maps well-known city names to their IANA timezone
identifiers. The lookup function `lookup_city` checks aliases first, then falls back to matching against IANA zone path
components. Cities like "New York" or "Los Angeles" are findable through the IANA path, but cities like "Nashville" or "
Florence" — which don't appear in any IANA zone name — require an explicit alias entry.

Current coverage is 92 alias entries across all regions. The US has 16, Europe has 22 (6 of which are Romanian cities),
and Asia has 22 (9 of which are Indian cities). Many major global cities are absent.

## Goals / Non-Goals

**Goals:**

- Add ~50 aliases for major cities across the US, Europe, and Asia that users are likely to search for
- Maintain the existing code style and organizational structure (grouped by region, aligned columns)
- Ensure every new alias maps to a correct IANA zone with accurate display region

**Non-Goals:**

- Comprehensive coverage of all world cities (diminishing returns)
- Changes to the lookup algorithm or `CityAlias` struct
- Adding aliases for Africa or Oceania (current coverage there is adequate for now)
- Deduplication or removal of existing entries
- Fuzzy matching or partial name search

## Decisions

**1. Selection criteria: metro population + global name recognition**

Cities are included if they meet either threshold:

- Metro population > 500K and in a country already represented, OR
- Widely recognized globally regardless of size (e.g., Florence, Venice, Kyoto, Mecca)

Alternative considered: strict population cutoff only. Rejected because culturally significant cities like Venice (~
260K) or Chiang Mai (~130K) are searched far more often than their population suggests.

**2. One entry per city, no alternate spellings (with exceptions)**

Unlike the existing Bangalore/Bengaluru dual-entry pattern, new cities get a single entry using the most common English
name. Exceptions only where the alternate name is equally prevalent in English.

Alternative considered: adding both local and English names for all cities. Rejected to keep the table manageable — can
always add more aliases later.

**3. IANA zone assignment verified against timezone boundary**

Each city's zone is verified, with special attention to:

- El Paso, TX → `America/Denver` (Mountain, not Central like most of Texas)
- Chiang Mai → `Asia/Bangkok` (same zone as Bangkok)
- Mecca/Medina → `Asia/Riyadh` (Saudi Arabia single zone)
- Cebu → `Asia/Manila` (Philippines single zone)

**4. Append within existing regional groups, maintain column alignment**

New entries are inserted within their respective regional comment blocks (`// North America`, `// Europe`, `// Asia`),
keeping the existing whitespace-aligned formatting.

## Risks / Trade-offs

- **[Name collisions]** Some city names exist in multiple countries (e.g., "Valencia" exists in Spain and Venezuela). We
  use the most globally recognized one. → Mitigation: the `display_region` field disambiguate for the user.
- **[Stale data]** Timezone rules can change (though very rarely for the cities we're adding). → Mitigation: all entries
  use stable IANA identifiers that are maintained by the tz database.
- **[Binary size]** ~50 new static string entries add roughly 3-4 KB. → Acceptable for a CLI tool.
