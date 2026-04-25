# 🕜 woti

World time in your terminal - see current times across time zones at a glance.

![screenshot.png](screenshot.png)

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/aleris/woti/main/scripts/install.sh | sh
```

To install a specific version:

```sh
curl -fsSL https://raw.githubusercontent.com/aleris/woti/main/scripts/install.sh | sh -s -- --version v0.29.0
```

Or download it from [releases](https://github.com/aleris/woti/releases).

Supported platforms: macOS (Apple Silicon, Intel) and Linux (x86_64, aarch64).

The binary is installed to `~/.local/bin`.

## Usage

```sh
woti                        # Launch the TUI

woti add PST                # Add by timezone abbreviation (PST, EET, CET, CST, EST, etc.) (*)
woti add Bucharest          # Add by city name
woti add America/New_York   # Add by IANA identifier in tz database (**)
woti add San Jose           # Add by city name that is not in IANA (***)
woti remove PST             # Remove a timezone

woti --help                 # Show help
```

<sup>(\*)</sup> [List of time zone abbreviations](https://en.wikipedia.org/wiki/List_of_time_zone_abbreviations)
<sup>(\*\*)</sup> [List of tz database time zones](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
<sup>(\*\*\*)</sup> The list is limited see `tz_data.rs`

Local and UTC are preconfigured by default. Configuration is stored in `~/.config/woti/config.toml`.

### TUI controls

- `←` / `→`: scroll the timeline by one cell (60, 30, or 15 minutes depending on `i`)
- `↑` / `↓`: scroll timezone list
- `c`: copy the current selected column to clipboard
- `w`: turn workday hours shading on/off
- `f`: cycle time format (mixed -> am/pm -> 24h → …)
- `i`: cycle navigation interval (60 → 30 → 15 → …) — the choice is persisted
- `q` / `x` / `Esc`, `ctrl+c`: exit

In **mixed** mode, 12h/24h is chosen per timezone based on the country's convention
from [Unicode CLDR](https://cldr.unicode.org/) data, last updated on March 2026.
This covers ~320 canonical IANA zones; legacy or uncommon aliases default to 24h. 
Press `f` to override if the automatic choice doesn't match your preference - the setting is saved.

### Print to console

To print the time with all your zones to console:

```sh
woti print # prints for current time

woti print --time 19:00 # prints for 7 pm using the current date
woti print --date 2026-03-20 --time 19:00 # prints for 7 pm on March 20, 2026
```

The input format is ISO date and time.

The output is the same as for copy from TUI, just printed to stdout directly. It prints something like:
> UTC 17:00<br/>
> Bucharest / EET 19:00<br/>
> San Jose / PDT 10am<br/>
> Bangalore / IST 10:30pm<br/>

### Pinning date & time in TUI

By default, the TUI shows the current live time. Use `--date` and `--time` to pin the display
to a specific point in time (ISO 8601 format, interpreted in your local timezone):

```sh
woti --date 2026-04-15               # Launch pinned to April 15, current time
woti --time 09:30                    # Today at 09:30
woti --date 2026-04-15 --time 14:00  # April 15 at 2pm
```

When pinned, the clock does not advance - the displayed time stays fixed. 
Left/Right arrow navigation still shifts by one cell (60, 30, or 15 minutes — see below)
relative to the pinned time.

### Navigation interval

By default each timeline cell represents one hour. Switch to finer granularity with the
`i` shortcut in the TUI or with the `--interval` flag at launch:

```sh
woti --interval 30   # Launch with 30-minute cells
woti --interval 15   # Launch with 15-minute cells
```

- The `i` key inside the TUI cycles `60 → 30 → 15 → 60` and **persists** the choice to
  `config.toml` (same as `f` and `w`).
- `--interval` is a **session-only** override that does not modify the saved config
  (same as `--date` / `--time`).
- At 30/15-minute intervals each hour cell gains 1 or 3 intermediate ticks; the sub-row
  shows minute markers as superscript digits (`¹⁵ ³⁰ ⁴⁵`). For whole-hour timezones the
  hour cell sub-row stays blank (the hour digit above marks the slot). For fractional
  zones (Nepal `+5:45`, India `+5:30`, …) the intermediate cell that lands on wall `:00`
  shows `⁰⁰`, since the row above only shows a `·` tick there. At 60 minutes the view
  is byte-for-byte identical to earlier versions.

### Configuration

The configuration is stored in a config file. 
On macOS/Linux the configuration file is located at `~/.config/woti/config.toml`.
The file is updated when changed from the cli. 
You can also update it directly (for example to reorder timezones).

## Development

Requires Rust 2024 edition (1.85+).

Before committing, run dev setup to install the git hook to increment version. 
```sh
make setup-dev
```

Build, test, run:
```sh
cargo build
cargo test
cargo run
cargo run -- --version
```

Release, pushes current version tag which triggers workflow on GitHub:
```sh
make release
```
