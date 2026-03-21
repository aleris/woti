## Context

`src/tz_data.rs` has two static tables that both carry region display strings:

1. **`REGION_NAMES`** — maps IANA zone IDs to human-readable region strings (e.g. `"America/Los_Angeles"` →
   `"United States, California"`). Used by `city_and_region()` as the canonical region source for any IANA zone.

2. **`CITY_ALIASES`** — maps common city names that don't appear in IANA paths to their matching zone. Each entry
   carries a `display_region` field.

~65% of alias entries duplicate the exact string that `REGION_NAMES` already provides for the same `iana_id`. The
remaining ~35% genuinely need an override because the alias city is in a different state/province than the IANA
reference city (e.g. Seattle shares `America/Los_Angeles` but is in Washington, not California).

## Goals / Non-Goals

**Goals:**

- Eliminate redundant region strings from `CITY_ALIASES` by falling back to `REGION_NAMES`.
- Rename `display_region` → `region_override` to make the intent self-documenting.
- Add doc comments to `CityAlias` fields and section comments in the table to explain the override vs. fallback
  distinction.
- Fill gaps in `REGION_NAMES` so aliases pointing to unlisted zones can also use fallback.
- Preserve identical output for every existing lookup — zero user-visible change.

**Non-Goals:**

- Changing `REGION_NAMES` from a linear-scan slice to a `HashMap`. The table is ~100 entries and scanned at most once
  per alias lookup; optimisation is not needed.
- Restructuring the overall lookup pipeline (`lookup_city`, `lookup_abbreviation`, `city_and_region`).
- Adding new city aliases or changing any existing display text.

## Decisions

### 1. Use `Option<&'static str>` for the override field

**Choice:** Change `display_region: &'static str` to `region_override: Option<&'static str>`.

**Alternatives considered:**

- *Separate "override" and "no-override" struct variants* — adds complexity for no gain; `Option` is idiomatic Rust.
- *Empty-string sentinel (`""`)* — works but is less explicit than `Option::None` and loses type-level documentation of
  intent.

**Rationale:** `Option` is zero-cost at the type level, makes the "override vs. fallback" semantics visible in the type
signature, and compiles to the same memory layout (nullable pointer).

### 2. Resolve region inside `lookup_city` via existing `city_and_region()`

When `region_override` is `None`, call `city_and_region(alias.iana_id)` which already consults `REGION_NAMES` with a
continent-prefix fallback. This reuses the existing resolution chain rather than duplicating the lookup logic.

### 3. Add three missing zones to `REGION_NAMES`

`Europe/Tirane` (Albania), `America/Guayaquil` (Ecuador), and `Asia/Makassar` (Indonesia) are referenced by aliases but
absent from `REGION_NAMES`. Adding them lets those aliases use `None` and avoids the continent-only fallback.

### 4. Rename field and add comments for clarity

Rename `display_region` → `region_override`. Add a doc comment on the struct field explaining the fallback behaviour.
Add inline comments in the `CITY_ALIASES` table to separate "same-region" groups (where `None` is used) from "override"
groups (where `Some(...)` is needed) so future editors know the pattern.

## Risks / Trade-offs

- **[Subtle data error]** An alias silently gets the wrong region if its `iana_id` is absent from `REGION_NAMES` and no
  override is provided. → Mitigated by auditing every alias during implementation and adding the 3 missing zones.
  Existing tests for `lookup_city` (San Jose, Mumbai, etc.) act as regression guards.
- **[Larger diff for a data-only change]** Touching ~92 alias entries makes the diff noisy. → Acceptable as a one-time
  cleanup; the table will be smaller and clearer afterwards.
