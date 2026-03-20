## 1. Dependency swap

- [x] 1.1 Replace `directories = "6"` with `dirs = "6"` in `Cargo.toml`
- [x] 1.2 Update the `use` import in `src/config.rs` — remove `directories::ProjectDirs`, add `dirs::home_dir`

## 2. Path resolution

- [x] 2.1 Add a helper `xdg_config_path()` that returns `~/.config/woti/config.toml` using `dirs::home_dir()`
- [x] 2.2 Add a helper `legacy_config_path()` that returns the platform-native path (hard-coded per-platform constant, e.g. `~/Library/Application Support/woti/config.toml` on macOS)
- [x] 2.3 Rewrite `config_path()` to return the XDG path if it exists, else the legacy path if it exists, else the XDG path (for creation)
- [x] 2.4 Add a `save_path()` method that always returns the XDG path (used by `save()`)

## 3. Load / Save updates

- [x] 3.1 Update `load()` to use the new `config_path()` resolution logic
- [x] 3.2 Update `save()` to always write to `save_path()` (XDG location), creating `~/.config/woti/` if needed

## 4. Tests

- [x] 4.1 Add unit tests verifying `xdg_config_path()` and `legacy_config_path()` return expected values
- [x] 4.2 Verify existing config round-trip tests still pass
