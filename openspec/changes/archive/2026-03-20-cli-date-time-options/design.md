## Context

The TUI calls `Utc::now()` on every render cycle (~250 ms) to drive the displayed time. There is no indirection — `render_timezone_block` and `build_copy_text` both call `Utc::now()` directly. The `hour_offset` field on `App` shifts the view relative to that live clock.

Users need to view timezone comparisons at a point in time other than "right now" — e.g. checking what a 3 PM ET meeting looks like in other zones next Tuesday.

## Goals / Non-Goals

**Goals:**

- Let users pin the TUI to an arbitrary date and/or time via `--date` and `--time` CLI flags.
- Preserve the current live-clock behaviour when no flags are given.
- Keep `hour_offset` navigation working relative to the anchor.

**Non-Goals:**

- Accepting non-ISO date/time formats (no natural language like "next Tuesday").
- Adding a TUI-internal prompt to change date/time after launch.
- Changing the refresh rate or tick model.

## Decisions

### D1: Anchor stored as `Option<DateTime<Utc>>` on `App`

The `App` struct gets an `anchor_time: Option<DateTime<Utc>>` field. A helper method `App::reference_time()` returns `self.anchor_time.unwrap_or_else(Utc::now)`. All call sites that currently use `Utc::now()` (render and copy) switch to `self.reference_time()`.

**Why:** A single optional field is the minimal change. The `None` path preserves exact current behaviour — live-updating clock. The `Some` path freezes the reference point, and `hour_offset` arithmetic works identically against it.

### D2: Flags live on `Cli`, not on a subcommand

`--date` and `--time` are top-level optional args on the `Cli` struct, guarded by `conflicts_with = "command"` so they only apply when no subcommand is given (i.e. the TUI path). This mirrors how a bare `woti` invocation launches the TUI.

**Why:** Adding a new subcommand (e.g. `woti view --date ...`) would be a larger surface change and break the zero-arg ergonomics. Top-level flags feel natural: `woti --date 2026-04-01 --time 15:00`.

### D3: Parse in local timezone, convert to UTC

The `--date` value is a `NaiveDate`; `--time` is a `NaiveTime`. When both are provided, combine into a `NaiveDateTime`, interpret in the user's local timezone (`iana-time-zone` crate already in deps), then convert to UTC. When only `--date` is given, use the current local time of day. When only `--time` is given, use today's date in local timezone.

**Why:** Users think in local time. Interpreting `--time 15:00` as 3 PM in whatever zone the system clock reports matches intuition. UTC interpretation would confuse most users.

### D4: ISO 8601 subset only

`--date` accepts `YYYY-MM-DD`. `--time` accepts `HH:MM` or `HH:MM:SS`. No timezone offset suffixes — see D3 for timezone interpretation.

**Why:** ISO 8601 is unambiguous and universally understood. Keeping the parser to `NaiveDate::parse_from_str` and `NaiveTime::parse_from_str` means no new deps and minimal validation code.

## Risks / Trade-offs

- **[Ambiguity when only --time given near midnight]** → Acceptable: interpret as today's date in local tz; user can add `--date` to disambiguate.
- **[No live-update when anchor is set]** → By design; the whole point is a frozen reference. The display still redraws on each tick (for cursor/scroll responsiveness) but the time value is stable.
- **[Local timezone assumption]** → Could surprise users in CI/containers with UTC locale, but matches the existing `iana-time-zone` default-zone detection already used by the app.
