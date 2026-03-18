# Proposal: Add --reset flag to remove command

## Problem

There is no quick way to clear all user-added timezones and return to the default configuration (Local + UTC). Users who have added many timezones must remove them one by one with `woti remove <zone>`.

## Proposed Change

Add a `--reset` flag to the `remove` subcommand that removes all non-default timezones in a single operation, restoring the config to its initial state (Local + UTC only).

Usage: `woti remove --reset`

The flag is mutually exclusive with the positional `zone` argument — you either remove a specific timezone or reset all of them.

## Impact

- **cli.rs**: Add `--reset` boolean flag to the `Remove` variant
- **main.rs**: Branch on the flag in `cmd_remove` — if set, replace config with defaults instead of resolving a zone
- **config.rs**: Add a `reset()` method that replaces the timezone list with defaults

No changes to the TUI, timezone resolution, or tz_data modules.
