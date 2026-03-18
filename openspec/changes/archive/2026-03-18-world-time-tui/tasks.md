## 1. Project Setup and Dependencies

- [x] 1.1 Add dependencies to Cargo.toml: clap (derive), ratatui, crossterm, chrono, chrono-tz, serde (derive), toml, directories
- [x] 1.2 Set up module structure: main.rs, cli.rs, config.rs, timezone.rs, tui.rs, city_db.rs

## 2. Configuration and Persistence

- [x] 2.1 Define config data model: TimezoneEntry (iana_id, city, region, display_zone) and AppConfig (list of entries) with serde derives
- [x] 2.2 Implement config load/save using directories crate for platform-standard path (~/.config/woti/config.toml)
- [x] 2.3 Implement default config initialization with Local and UTC entries, creating config file on first access

## 3. Timezone Resolution

- [x] 3.1 Build city-to-timezone lookup table mapping major city names to IANA zone IDs with city and region metadata
- [x] 3.2 Build abbreviation-to-IANA mapping (PST, EET, CET, CST, EST, etc.) with most-common-interpretation defaults
- [x] 3.3 Implement unified resolve function: try IANA direct match, then abbreviation, then city name; return TimezoneEntry or error
- [x] 3.4 Implement duplicate detection by resolved IANA identifier

## 4. CLI Subcommands

- [x] 4.1 Define clap CLI structure with optional subcommands: add <zone>, remove <zone>, and default (no subcommand) for TUI. Include program description, version (from Cargo.toml), and detailed about text with usage examples. Add per-subcommand help with accepted input formats (abbreviation, city, IANA) and examples
- [x] 4.2 Implement `woti add` handler: resolve input, check duplicates, append to config, print confirmation
- [x] 4.3 Implement `woti remove` handler: resolve input, find matching entry, prevent removing Local/UTC defaults, remove from config, print confirmation
- [x] 4.4 Wire main.rs to dispatch subcommands or launch TUI when no subcommand given

## 5. TUI Core Setup

- [x] 5.1 Implement terminal setup/teardown (crossterm raw mode, alternate screen, mouse capture disable) with panic-safe cleanup
- [x] 5.2 Implement main event loop: tick timer (1s) for clock updates, keyboard input polling, quit on q/x
- [x] 5.3 Define TUI application state: loaded config, current hour offset, terminal size

## 6. TUI Layout and Rendering

- [x] 6.1 Implement header widget: "woti" title left-aligned, live local date/time/zone right-aligned, styled background
- [x] 6.2 Implement footer widget: keyboard shortcut hints (← Previous Hour | → Next Hour | q Exit) with styled key symbols
- [x] 6.3 Implement timezone info row rendering: line 1 ($CITY / $ZONE + 3 spaces + $TIME), line 2 ($REGION + 3 spaces + $DATE)
- [x] 6.4 Implement 24-hour timeline strip rendering: row 1 (day markers at midnight crossings), row 2 (two-digit hours 0-23), row 3 (am/pm dimmed or empty)
- [x] 6.5 Implement current-hour column highlighting across all timeline strip rows
- [x] 6.6 Apply color scheme: header/footer styling, city/zone emphasis, day marker colors, dimmed am/pm, highlighted current hour

## 7. Timeline Navigation

- [x] 7.1 Handle left/right arrow key input to adjust hour offset (+1 / -1)
- [x] 7.2 Implement rendering debounce: accumulate offset changes during fast key-repeat, render at most every 50ms
- [x] 7.3 Ensure offset resets to zero on TUI launch (no persistence of scroll state)

## 8. Integration and Polish

- [x] 8.1 Wire all components together in main: CLI parsing → subcommand dispatch or TUI launch with loaded config
- [x] 8.2 Test end-to-end: add/remove timezones, launch TUI, verify display, arrow key scrolling, quit
- [x] 8.3 Handle edge cases: empty config (only defaults), terminal resize, very narrow terminals
