## Context

`build_abbreviation_map()` in `src/tz_data.rs` constructs a `HashMap<String, Tz>` by iterating every IANA timezone and sampling two fixed dates — January 15 and July 15 of **2025** — to discover each zone's abbreviations. The map is stored in a `LazyLock` static and built once per process.

Because the sample year is hardcoded, the map cannot pick up DST rule changes published by IANA for years after 2025 (e.g., a country abolishing DST).

## Goals / Non-Goals

**Goals:**

- The abbreviation map SHALL reflect the current year's DST rules at process startup.
- The change is minimal: only the year derivation changes; map-building logic stays identical.

**Non-Goals:**

- Rebuilding the map at runtime after process start (it is initialized once via `LazyLock`; that is sufficient).
- Handling mid-year rule changes (IANA updates published after the binary started are out of scope).

## Decisions

**Use `Utc::now().year()` for the sample year.**

`chrono::Utc::now()` is already available through the `chrono` crate. Calling `.year()` (from the `Datelike` trait) gives the current UTC year at map-initialization time.

Alternatives considered:
- `Local::now().year()` — adds a local-time dependency that is unnecessary; UTC year is sufficient for sampling DST offsets.
- Making the year configurable — over-engineering for a one-line fix.

## Risks / Trade-offs

- **[Negligible runtime cost]** → `Utc::now()` is a single syscall, called once at process startup inside `LazyLock`. No measurable impact.
- **[Year boundary edge case]** → If the process starts on Dec 31 at 23:59 UTC, the map uses that year's rules. This is acceptable; the map is rebuilt on next launch.
