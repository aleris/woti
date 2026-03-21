## Why

When scripting or piping `woti` output into other tools, there is no way to get the timezone text without launching the
interactive TUI and manually pressing `c` to copy. A `print` subcommand lets users get the same formatted text directly
on stdout, enabling shell pipelines, cron jobs, and quick terminal lookups without clipboard dependency.

## What Changes

- Add a new `print` subcommand that outputs to stdout the same formatted text that the `c` (copy) key produces in the
  TUI
- Support `--date` and `--time` flags on the `print` subcommand (same semantics as the root-level flags) to pin the
  reference time
- The process exits immediately after printing (no TUI, no raw mode, no alternate screen)

## Capabilities

### New Capabilities

- `print-command`: Non-interactive stdout output of timezone times with optional date/time pinning

### Modified Capabilities

## Impact

- `src/cli.rs` — new `Print` variant in the `Command` enum with `--date` and `--time` args
- `src/main.rs` — new `cmd_print` handler, reuse of `parse_anchor` logic
- `src/tui/copy.rs` — `build_copy_text` needs to be callable outside the TUI context (currently `App` method); may need
  extraction or a standalone builder
- No new dependencies required; all formatting logic already exists
