## Why

There's no way to share the timezone comparison from the TUI. When coordinating meetings across timezones, users need to copy the selected hour for all timezones and paste it into chat or email. Currently they must manually type out each timezone's time.

## What Changes

- Add a `c` keyboard shortcut that copies the currently selected (highlighted) hour across all configured timezones to the system clipboard
- Each line includes: `City / Abbreviation Time` (e.g., `Bucharest / EET 4pm`)
- If a timezone's selected hour falls on a different calendar day than the reference (first) timezone, append the day and date (e.g., `San Jose / PDT 8pm WED 19`)
- Show a brief visual confirmation in the footer when copy succeeds
- Add the `c` shortcut to the footer help bar

## Capabilities

### New Capabilities
- `copy-selection`: Copy the highlighted hour column across all timezones to the system clipboard in a human-readable multi-line format

### Modified Capabilities

## Impact

- `src/tui.rs`: New key handler for `c`, clipboard write logic, footer update for the shortcut hint and copy confirmation
- New dependency: clipboard crate (e.g., `arboard` or `cli-clipboard`) for cross-platform clipboard access
- `Cargo.toml`: Add clipboard dependency
