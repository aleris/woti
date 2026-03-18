## Why

A developer-friendly CLI tool for quickly checking the current time across multiple time zones. Working with distributed teams requires constant mental timezone math—`woti` eliminates this by providing an always-available, visually rich TUI that shows current times, upcoming hours, and day boundaries at a glance.

## What Changes

- Add `woti add <zone|city>` command to register time zones by IANA code, abbreviation, or city name
- Add `woti remove <zone|city>` command to unregister a time zone
- Local and UTC time zones are preconfigured by default
- Add TUI view (invoked with bare `woti`) showing:
  - Header with title and live-updating local date/time
  - Timezone rows with city/zone, region, current time, and date
  - 24-hour timeline strip per timezone with day-change markers and current-hour highlighting
  - Footer with keyboard shortcut hints
- Arrow key navigation to scroll the timeline left/right with debounced rendering for fast key-repeat
- Persistent timezone configuration stored in a user config file
- Colored output for enhanced readability (dimmed AM/PM indicators, highlighted current hour, styled header/footer)

## Capabilities

### New Capabilities
- `timezone-management`: Add/remove time zones by code, abbreviation, or city name; persist configuration; default Local+UTC
- `tui-display`: Terminal UI with timezone info rows, 24-hour timeline strips, header/footer, coloring, and live clock
- `timeline-navigation`: Arrow-key scrolling of the hour timeline with debounced rendering for smooth interaction

### Modified Capabilities

## Impact

- **New binary**: `woti` CLI entry point with subcommands (`add`, `remove`) and default TUI mode
- **Dependencies**: Rust TUI library (ratatui), timezone handling (chrono, chrono-tz), CLI argument parsing (clap), city-to-timezone resolution, config persistence (directories + serde/toml)
- **Config file**: User-level config at platform-standard location (e.g., `~/.config/woti/config.toml`)
- **No breaking changes**: Greenfield project
