## Why

The `directories` crate places the config file in platform-specific locations (e.g.
`~/Library/Application Support/woti/` on macOS). This is unintuitive for terminal-centric users who expect dotfile-style
configuration at `~/.config/woti/`. Adopting the XDG-style path as the primary location makes the config easy to find,
edit, and version-control across platforms.

## What Changes

- Config resolution checks `~/.config/woti/config.toml` first; if it exists, use it.
- Falls back to the existing `directories::ProjectDirs` path (platform-native) when the XDG path does not exist.
- New config files are always created at `~/.config/woti/config.toml`.
- The `directories` crate dependency is removed since we no longer rely on it for path resolution.

## Capabilities

### New Capabilities

- `xdg-config-path`: Resolve and create config files using `~/.config/woti/` as the preferred path, with a
  platform-native fallback for existing installs.

### Modified Capabilities

_(none — no existing spec-level requirements change)_

## Impact

- **Code**: `src/config.rs` — `config_path()`, `load()`, and `save()` methods change their path resolution logic.
- **Dependencies**: `directories` crate can be removed from `Cargo.toml`; replaced with `dirs::home_dir()` (from the
  lighter `dirs` crate) or manual `$HOME` lookup.
- **Existing users**: Users with a config at the old platform-native path will continue to have it loaded via fallback.
  No data loss or migration required.
