## 1. Extract copy text builder

- [x] 1.1 Extract a standalone `build_copy_text` function in `src/tui/copy.rs` that accepts `(&[TimezoneEntry], DateTime<Utc>, i32, &dyn Fn(&str) -> bool)` parameters instead of reading from `App` fields
- [x] 1.2 Update `App::build_copy_text` to be a thin wrapper calling the extracted function
- [x] 1.3 Verify existing copy tests still pass

## 2. Add `Print` CLI subcommand

- [x] 2.1 Add `Print` variant to `Command` enum in `src/cli.rs` with `--date` and `--time` optional args
- [x] 2.2 Update the CLI help text/examples to include the `print` subcommand

## 3. Wire up `cmd_print` in main

- [x] 3.1 Add `cmd_print` function in `src/main.rs` that loads config, parses anchor, calls `build_copy_text`, and prints to stdout with trailing newline
- [x] 3.2 Add the `Print` match arm in `main()` dispatch
- [x] 3.3 Make `build_copy_text` and necessary types `pub` so `main.rs` can call them

## 4. Tests

- [x] 4.1 Add a unit test that the extracted `build_copy_text` produces the same output when called standalone vs through `App`
- [x] 4.2 Add a test for `cmd_print` with `--date` and `--time` flags (integration or manual verification)
