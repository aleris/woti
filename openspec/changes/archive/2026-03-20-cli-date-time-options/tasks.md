## 1. CLI Parsing

- [x] 1.1 Add `--date` and `--time` optional fields to the `Cli` struct in `src/cli.rs` with `conflicts_with = "command"`, using `String` type with custom validation
- [x] 1.2 In `src/main.rs`, parse the `--date` and `--time` strings into `NaiveDate` / `NaiveTime`, combine with local timezone to produce `Option<DateTime<Utc>>`, and pass it to `cmd_tui()`
- [x] 1.3 Print clear error messages and exit non-zero when date/time values fail to parse

## 2. App Anchor Plumbing

- [x] 2.1 Add `anchor_time: Option<DateTime<Utc>>` field to `App` in `src/tui/app.rs` and update `App::new()` to accept it
- [x] 2.2 Add `App::reference_time(&self) -> DateTime<Utc>` helper that returns `anchor_time.unwrap_or_else(Utc::now)`

## 3. Replace `Utc::now()` Call Sites

- [x] 3.1 Replace `Utc::now()` in `render_timezone_block` (`src/tui/render.rs`) with `self.reference_time()`
- [x] 3.2 Replace `Utc::now()` in `build_copy_text` (`src/tui/copy.rs`) with `self.reference_time()`

## 4. Update Tests

- [x] 4.1 Update the `app_with` test helper in `src/tui/copy.rs` to supply `anchor_time: None` so existing tests compile
- [x] 4.2 Add a test that constructs an `App` with a known `anchor_time` and verifies `build_copy_text` produces time values matching that anchor instead of the live clock

## 5. Documentation

- [x] 5.1 Update `README.md` to document the `--date` and `--time` flags with usage examples
