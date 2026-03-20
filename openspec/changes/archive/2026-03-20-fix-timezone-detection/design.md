## Context

The `localtime_iana()` function in `src/config.rs` (lines 102-119) detects the user's local IANA timezone to use as a default when no explicit timezone is configured. Currently it only reads the `/etc/localtime` symlink on Unix systems, which fails in three common scenarios: `TZ` env var overrides, container environments with copied timezone files, and non-Unix platforms.

The function is called during config loading to populate a default timezone. Returning `None` causes a silent fallback to UTC, which can confuse users who expect their local time.

## Goals / Non-Goals

**Goals:**
- Respect the `TZ` environment variable as the highest-priority timezone source
- Handle `/etc/localtime` as both a symlink and a regular file
- Detect the local timezone on Windows and other non-Unix platforms
- Maintain the existing return type (`Option<String>`) and call sites unchanged

**Non-Goals:**
- Supporting exotic `TZ` formats (e.g., POSIX TZ strings like `EST5EDT`) — only IANA names (e.g., `America/New_York`)
- Removing the existing Unix filesystem detection — it remains as a fallback layer
- Changing how the detected timezone is used downstream in the config system

## Decisions

### Use a layered detection strategy with clear priority

Detection order:
1. `TZ` environment variable (if set and non-empty)
2. `iana-time-zone` crate (`iana_time_zone::get_timezone()`)
3. Unix filesystem heuristic (existing `/etc/localtime` logic, improved)
4. Return `None`

**Rationale**: `TZ` is the POSIX-standard override and must take precedence. The `iana-time-zone` crate uses platform-native APIs (Core Foundation on macOS, Windows registry, `/etc/localtime` + `/etc/timezone` on Linux) and covers the common cases robustly. The filesystem heuristic is kept as a last-resort fallback.

**Alternatives considered**:
- *Only use `iana-time-zone` crate*: Doesn't respect `TZ` env var, which is important for containers and testing
- *Only fix the filesystem logic*: Doesn't solve Windows support and reimplements what `iana-time-zone` already does well

### Add `iana-time-zone` crate as a dependency

The `iana-time-zone` crate is lightweight (no transitive deps beyond platform system libs), widely used (46M+ downloads), and maintained. It calls platform-native APIs:
- macOS: `CFTimeZoneCopySystem()`
- Linux: reads `/etc/timezone`, `/etc/localtime`, `timedatectl`
- Windows: `GetDynamicTimeZoneInformation()` + CLDR mapping

**Rationale**: Reusing a well-tested crate is preferable to reimplementing platform-specific detection for each OS.

### Validate TZ values against `chrono-tz`

When reading `TZ`, parse the value with `chrono_tz::Tz::from_str` to confirm it's a valid IANA timezone name before returning it. Invalid or POSIX-style values are silently skipped, falling through to the next detection layer.

**Rationale**: The rest of the codebase expects IANA timezone names. Accepting arbitrary strings could cause panics or incorrect behavior downstream.

### Improve the Unix filesystem fallback with `canonicalize`

Replace `std::fs::read_link` with `std::fs::canonicalize` so the path resolution works for both symlinks and regular files that have been symlinked at a higher directory level. Still extract the IANA name by splitting on `/zoneinfo/`.

**Rationale**: `read_link` only works for direct symlinks. `canonicalize` resolves the full path chain, handling cases where `/etc/localtime` is a hardlink or where an intermediate directory is symlinked.

## Risks / Trade-offs

- **New dependency**: Adding `iana-time-zone` increases the dependency tree slightly. → Mitigated by the crate being small with no heavy transitive deps.
- **`TZ` validation rejects valid POSIX strings**: A user setting `TZ=EST5EDT` will have it silently ignored. → Acceptable because woti requires IANA names throughout; POSIX strings would break timezone lookups anyway.
- **`iana-time-zone` crate maintenance**: If the crate becomes unmaintained, the fallback layers still work. → Low risk given the crate's adoption and maintenance track record.
