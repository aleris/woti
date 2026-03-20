# 🕜 woti

World time in your terminal - see current times across time zones at a glance.

![screenshot.png](screenshot.png)

## Install

```
curl -fsSL https://raw.githubusercontent.com/aleris/woti/main/scripts/install.sh | sh
```

To install a specific version:

```
curl -fsSL https://raw.githubusercontent.com/aleris/woti/main/scripts/install.sh | sh -s -- --version v0.3.0
```

Supported platforms: macOS (Apple Silicon, Intel) and Linux (x86_64, aarch64).

The binary is installed to `~/.local/bin` (or `/usr/local/bin` when run as root).

## Usage

```
woti                        Launch the TUI

woti add PST                Add by timezone abbreviation
woti add Bucharest          Add by city name
woti add America/New_York   Add by IANA identifier
woti remove PST             Remove a timezone

woti --help                 Show help
```

Local and UTC are preconfigured by default. Configuration is stored in `~/.config/woti/config.toml`.

### TUI controls

- `←` / `→`: scroll the timeline by hour
- `↑` / `↓`: scroll timezone list
- `c`: copy the current selected hours column to clipboard
- `w`: turn workday hours shading on/off
- `f`: cycle time format (mixed -> am/pm -> 24h → …)
- `q` / `x` / `Esc`, `ctrl+c`: exit

In **mixed** mode, 12h/24h is chosen per timezone based on the country's convention
from [Unicode CLDR](https://cldr.unicode.org/) data, last updated on March 2026.
This covers ~320 canonical IANA zones; legacy or uncommon aliases default to 24h. 
Press `f` to override if the automatic choice doesn't match your preference — the setting is saved.

### Pinning date & time

By default, the TUI shows the current live time. Use `--date` and `--time` to pin the display
to a specific point in time (ISO 8601 format, interpreted in your local timezone):

```
woti --date 2026-04-15              Launch pinned to April 15, current time
woti --time 09:30                   Today at 09:30
woti --date 2026-04-15 --time 14:00 April 15 at 2pm
```

When pinned, the clock does not advance — the displayed time stays fixed. 
Left/Right arrow navigation still shifts by hour relative to the pinned time.

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
