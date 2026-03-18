# woti

World time in your terminal — see current times across time zones at a glance.

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
- `c`: copy current selection to clipboard
- `f`: cycle time format (24h → am/pm → mixed → …)
- `q` / `x` / `Esc`: exit

## Development

Requires Rust 2024 edition (1.85+).

```
cargo build
cargo run
cargo test
```
