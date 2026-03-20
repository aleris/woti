## Context

The app currently uses the `directories` crate (`ProjectDirs::from("", "", "woti")`) to resolve the config file path.
This yields platform-specific locations:

- **macOS**: `~/Library/Application Support/woti/config.toml`
- **Linux**: `~/.config/woti/config.toml` (XDG default)
- **Windows**: `C:\Users\<user>\AppData\Roaming\woti\config.toml`

Since woti is a terminal tool aimed at developers, users expect `~/.config/woti/` — the de facto standard for CLI tools.
On Linux this already happens to match, but on macOS the current path is buried in `Library/Application Support`.

## Goals / Non-Goals

**Goals:**

- Make `~/.config/woti/config.toml` the preferred config location on all platforms.
- Transparently fall back to the old `directories`-based path so existing users are unaffected.
- Create new config files at `~/.config/woti/config.toml`.
- Remove the `directories` crate dependency.

**Non-Goals:**

- Automatically migrating existing config files to the new location.
- Honoring `$XDG_CONFIG_HOME` overrides (keep it simple — hardcoded `~/.config`).
- Windows support for `~/.config` (Windows has no `$HOME` in the same sense; fall through to `None`).

## Decisions

### 1. Use `dirs::home_dir()` for home directory resolution

**Choice**: Replace `directories` crate with `dirs` (lighter, single-purpose).

**Rationale**: `directories` provides `ProjectDirs`, `BaseDirs`, `UserDirs` — all unnecessary. We only need the user's
home directory to construct `~/.config/woti/`. The `dirs` crate already handles cross-platform home resolution (`$HOME`
on Unix, `USERPROFILE` on Windows) and has zero transitive dependencies.

**Alternative considered**: Manual `std::env::var("HOME")` — fragile, no Windows fallback.

### 2. Two-tier path resolution (XDG-first, legacy-fallback)

**Choice**: `config_path()` returns the first path that exists from:

1. `~/.config/woti/config.toml`
2. Old platform-native path via `directories::ProjectDirs` (compile-time only for the fallback constant)

If neither exists, return the XDG path (so new files are created there).

**Rationale**: Existing macOS users with configs at `~/Library/Application Support/woti/` should not lose their
configuration. Once they edit or re-save, the new path is used.

**Simplification**: Since the `directories` crate is being removed, hard-code the known legacy paths per-platform as
constants rather than keeping the dependency just for fallback. On Linux the legacy and new paths are identical so no
fallback is needed. On macOS the legacy path is `~/Library/Application Support/woti/config.toml`.

### 3. `save()` always writes to the XDG path

**Choice**: `save()` writes to `~/.config/woti/config.toml` regardless of where the file was loaded from.

**Rationale**: This naturally migrates users to the new path when they make any config change. The old file remains but
becomes stale — an acceptable trade-off to avoid explicit migration logic.

## Risks / Trade-offs

- **[Stale legacy file]** → After a save, both old and new files exist. Acceptable: XDG is checked first, so the old
  file is ignored. Users can manually delete it.
- **[No `$XDG_CONFIG_HOME` support]** → Power users who set `$XDG_CONFIG_HOME=/custom/path` won't have that respected.
  Mitigation: can be added in a follow-up if requested; hardcoded `~/.config` covers 99% of use cases.
- **[Windows edge case]** → `dirs::home_dir()` returns `Some` on Windows, so `~/.config/woti/` could technically be
  created on Windows. This is fine — the app already targets Unix primarily.
