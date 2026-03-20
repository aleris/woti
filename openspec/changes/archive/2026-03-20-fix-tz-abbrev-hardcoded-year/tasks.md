## 1. Update sample year derivation

- [x] 1.1 Add `use chrono::Utc;` and `use chrono::Datelike;` imports to `src/tz_data.rs` (if not already present)
- [x] 1.2 Replace hardcoded `2025` in `build_abbreviation_map()` with `Utc::now().year()` for both winter and summer sample dates

## 2. Verify

- [x] 2.1 Run `cargo build` to confirm the project compiles without errors
- [x] 2.2 Run `cargo test` to confirm existing tests pass
