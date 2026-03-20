## 1. Dependencies

- [x] 1.1 Add `iana-time-zone` crate to `Cargo.toml` dependencies

## 2. Core Implementation

- [x] 2.1 Add a `tz_from_env()` helper that reads the `TZ` environment variable, validates it as a valid IANA name via `chrono_tz::Tz::from_str`, and returns `Option<String>`
- [x] 2.2 Rewrite `localtime_iana()` to implement the layered detection chain: `TZ` env var -> `iana_time_zone::get_timezone()` -> Unix filesystem heuristic -> `None`
- [x] 2.3 Replace `std::fs::read_link` with `std::fs::canonicalize` in the Unix filesystem fallback path
- [x] 2.4 Remove the `#[cfg(not(unix))]` block that unconditionally returns `None` — the `iana-time-zone` crate handles non-Unix platforms

## 3. Verification

- [x] 3.1 Test locally with `TZ=America/Chicago cargo run` to confirm env var detection
- [x] 3.2 Test locally without `TZ` set to confirm platform-native detection via `iana-time-zone`
- [x] 3.3 Verify the project compiles with `cargo build`
