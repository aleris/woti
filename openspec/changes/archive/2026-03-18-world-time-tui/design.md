## Context

This is a greenfield Rust CLI project (`woti`). The crate is initialized with Cargo (edition 2024) and has no existing functionality beyond a hello-world main. The tool targets developers who work across time zones and need a fast, terminal-native way to see current times and scan upcoming hours.

The user invokes `woti` with no arguments to launch the TUI, or uses `woti add`/`woti remove` subcommands to manage their timezone list. Configuration persists between sessions in a TOML file at the platform-standard config directory.

## Goals / Non-Goals

**Goals:**
- Fast startup to TUI (<100ms on warm runs)
- Smooth arrow-key scrolling of the 24-hour timeline with debounce to skip intermediate renders during fast key-repeat
- Correct timezone resolution from abbreviations (PST, EET), IANA names (America/Los_Angeles), and city names (Bucharest, San Jose)
- Visually clear layout with color-coded elements, current-hour highlighting, and day-boundary markers
- Persistent user config with sensible defaults (Local + UTC)

**Non-Goals:**
- GUI or web interface
- Countdown timers, alarms, or scheduling features
- Network-based time sync (relies on system clock)
- Interactive timezone search/picker in the TUI (add/remove are separate subcommands)
- Support for non-standard terminal emulators or Windows Console (targets modern terminals with ANSI support)

## Decisions

### 1. CLI Framework: clap (derive)

**Choice**: Use `clap` with derive macros for argument parsing.
**Rationale**: Industry standard for Rust CLIs. Derive API provides clean subcommand definitions with minimal boilerplate. Alternatives considered: `argh` (lighter but less ecosystem support), manual parsing (too error-prone for subcommands).

### 2. TUI Framework: ratatui + crossterm

**Choice**: Use `ratatui` for UI rendering with `crossterm` as the backend.
**Rationale**: `ratatui` is the actively maintained successor to `tui-rs` with a large ecosystem. `crossterm` provides cross-platform terminal manipulation. Alternative considered: `termion` (Linux-only, less active).

### 3. Timezone Handling: chrono + chrono-tz + city lookup table

**Choice**: Use `chrono` for date/time, `chrono-tz` for IANA timezone database, and a built-in city-to-timezone mapping for city name resolution.
**Rationale**: `chrono-tz` embeds the full IANA tz database at compile time—no runtime downloads needed. For city resolution, embed a curated mapping of major cities to IANA zones (covering the user's expected inputs like "San Jose", "Bucharest"). Abbreviation resolution (PST→America/Los_Angeles) uses a hardcoded mapping since abbreviations are ambiguous (CST could be US Central or China Standard)—we pick the most common interpretation. Alternative considered: `jiff` (newer, but less mature ecosystem).

### 4. Config Storage: TOML file via `directories` crate

**Choice**: Store config at `~/.config/woti/config.toml` (Linux/macOS) or equivalent via the `directories` crate.
**Rationale**: TOML is human-readable and Rust-native (via `serde` + `toml`). The `directories` crate resolves platform-appropriate config paths. Config stores the list of timezone entries, each with a display name, IANA zone ID, city, and region.

### 5. Time Format: Follow locale conventions

**Choice**: Detect whether to show 12h or 24h format based on the timezone's common convention, with 24h as default.
**Rationale**: The user spec says "show time in default format for local (ie. 18:43 or 6:43a)." We default to 24h but allow the config to override. The hour strip always shows 0-23 numeric hours with optional am/pm row for 12h mode.

### 6. Rendering Architecture: Immediate-mode with debounced input

**Choice**: Use ratatui's immediate-mode rendering loop. On each tick or input event, re-render the entire frame. For arrow-key scrolling, accumulate offset changes and batch-render at most every 50ms to avoid jank during fast key-repeat.
**Rationale**: Immediate-mode is simpler and ratatui is optimized for full-frame redraws via diffing. Debouncing input prevents wasted renders when the user holds an arrow key. Alternative considered: partial/widget-level updates (adds complexity with minimal gain for this UI size).

### 7. Layout Structure

The TUI layout is divided vertically:
- **Header** (1 line): Title left-aligned, live clock right-aligned
- **Body** (remaining space): Scrollable list of timezone blocks, each consisting of:
  - Info row: `$CITY / $ZONE   $TIME` and below `$REGION   $DATE`
  - Timeline strip (3 rows): day markers, hour numbers, am/pm indicators
  - Blank separator row
- **Footer** (1 line): Key hints with styled key symbols

The timeline strip is centered on the current hour and shifts left/right with arrow keys. The current-hour column is rendered with an inverted/highlighted style.

## Risks / Trade-offs

- **City name ambiguity** → Multiple cities share names (e.g., "San Jose" in US vs Costa Rica). Mitigation: prefer the most populous match; the stored config uses IANA IDs so resolution only happens at `add` time. Users can always add by IANA code for precision.
- **Timezone abbreviation ambiguity** → CST, IST, etc. map to multiple zones. Mitigation: document the chosen defaults; allow IANA override.
- **Terminal color support** → Some terminals have limited color support. Mitigation: use standard 16/256 ANSI colors rather than true color; degrade gracefully.
- **Compile-time timezone DB size** → `chrono-tz` embeds the full IANA DB (~500KB). Acceptable for a CLI binary.
- **City lookup table maintenance** → A static city list won't cover every city. Mitigation: start with a comprehensive list of major world cities; users can always fall back to IANA zone names.
