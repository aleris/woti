## Context

`woti` currently has two ways to consume timezone information: the interactive TUI (live display) and the clipboard copy
triggered by `c` inside the TUI. There is no non-interactive path to get the formatted timezone text on stdout. The copy
logic lives in `App::build_copy_text()`, which is tightly coupled to the `App` struct (it reads `self.config`,
`self.reference_time()`, `self.hour_offset`, and `self.use_24h_for_tz()`).

The CLI already supports `--date` and `--time` flags at the root level, but only for the TUI path. Subcommands (`add`,
`remove`) cannot use these flags.

## Goals / Non-Goals

**Goals:**

- Add a `print` subcommand that outputs the same text as the TUI copy feature to stdout
- Support `--date` and `--time` flags on the `print` subcommand to pin reference time
- Reuse the existing copy formatting logic with no behavioral divergence
- Exit immediately after printing (zero TUI overhead)

**Non-Goals:**

- Custom output formats (JSON, CSV, etc.) — future enhancement
- Selecting a subset of timezones from the CLI — future enhancement
- Changing the existing copy behavior in the TUI

## Decisions

### Extract `build_copy_text` from `App`

**Decision:** Extract a standalone `build_copy_text` function that takes the required inputs as parameters rather than
reading from `App` fields. The `App` method becomes a thin wrapper calling the extracted function.

**Rationale:** The `print` subcommand should not need to construct a `tui::App` (which carries TUI state like
`scroll_offset`, `copied_at`, `should_quit`). Extracting the function lets `cmd_print` call it directly with config +
anchor + time format, keeping the code DRY without dragging in TUI dependencies.

**Alternative considered:** Constructing a dummy `App` for `print` — rejected because it's wasteful and couples
non-interactive output to TUI internals.

### Place `--date`/`--time` on the `Print` subcommand

**Decision:** The `Print` variant in the `Command` enum gets its own `date: Option<String>` and `time: Option<String>`
fields, using the same parsing logic as the root-level flags.

**Rationale:** Clap's derive API naturally supports per-subcommand args. Reusing `parse_anchor()` from `main.rs` keeps
behavior consistent. When neither flag is given, `print` uses the current time (same as TUI default).

### Use `Mixed` time format as default

**Decision:** The `print` subcommand uses the `time_format` from the user's config file (defaulting to `Mixed` if
unset), matching the TUI default.

**Rationale:** Consistency with what the user sees in the TUI. Adding a `--format` flag is a future enhancement.

## Risks / Trade-offs

- **[Signature change to `build_copy_text`]** → Extracting the function changes how `App` calls it; existing tests
  reference `app.build_copy_text()`. Mitigation: keep the `App` method as a wrapper that delegates to the free function,
  so tests remain unchanged.
- **[No `hour_offset` in `print`]** → The print subcommand always outputs the selected hour at offset 0 (the reference
  time). There's no concept of left/right navigation. This is intentional — the user pins the exact time with `--date`/
  `--time` instead.
