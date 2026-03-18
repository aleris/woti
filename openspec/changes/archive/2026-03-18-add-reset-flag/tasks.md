## 1. CLI changes

- [x] 1.1 Add `reset: bool` field with `#[arg(long, conflicts_with = "zone")]` to the `Remove` variant in `cli.rs`
- [x] 1.2 Add `#[arg(required_unless_present = "reset")]` to the `zone` field in `Remove`
- [x] 1.3 Update `after_help` on `Remove` to mention the `--reset` flag

## 2. Config changes

- [x] 2.1 Add `AppConfig::reset()` method that removes non-default timezones and returns the count removed

## 3. Command handler

- [x] 3.1 Update `main.rs` dispatch to destructure `reset` from `Remove` and branch accordingly
- [x] 3.2 Implement `cmd_reset()`: load config, call `reset()`, save, print confirmation (or "nothing to remove" message)

## 4. Tests

- [x] 4.1 Add unit test for `AppConfig::reset()` with custom timezones present
- [x] 4.2 Add unit test for `AppConfig::reset()` with only defaults (returns 0)
