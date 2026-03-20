## Why

The `localtime_iana()` function in `src/config.rs` has three detection gaps: it ignores the `TZ` environment variable (
the standard Unix override mechanism), fails when `/etc/localtime` is a regular file instead of a symlink (common in
Docker containers), and returns `None` on non-Unix platforms (Windows always defaults to UTC). These gaps cause silent
fallback to UTC in common environments like containers and Windows.

## What Changes

- Check the `TZ` environment variable first, before any filesystem-based detection
- On Unix, fall back to resolving `/etc/localtime` via `std::fs::canonicalize` (works for both symlinks and regular
  files) instead of `std::fs::read_link`
- Add the `iana-time-zone` crate as a cross-platform fallback that handles Windows, macOS, and Linux robustly
- Establish a clear priority chain: `TZ` env var -> `iana-time-zone` crate -> filesystem heuristics -> `None`

## Capabilities

### New Capabilities

- `robust-timezone-detection`: Reliable local timezone detection across Unix, containers, and Windows using a layered
  fallback strategy

### Modified Capabilities

## Impact

- **Code**: `src/config.rs` — rewrite of `localtime_iana()` function (lines 102-119)
- **Dependencies**: New dependency on `iana-time-zone` crate in `Cargo.toml`
- **Platforms**: Windows users will get automatic timezone detection instead of silent UTC fallback
- **Containers**: Docker/Podman environments with copied `/etc/localtime` files will be detected correctly
- **Dev environments**: Developers using `TZ=America/New_York ./woti` will see the correct override
