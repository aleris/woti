use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "woti",
    version,
    about = "World time in your terminal",
    long_about = "World time in your terminal — see current times across time zones at a glance.\n\n\
        Run with no arguments to launch the interactive TUI.\n\
        Use subcommands to manage your timezone list.",
    after_help = "\x1b[1mExamples:\x1b[0m\n  \
        woti                        Launch the TUI\n  \
        woti --date 2026-04-15      Launch pinned to a date\n  \
        woti --time 15:00           Launch pinned to a time\n  \
        woti print                  Print times to stdout\n  \
        woti print --time 15:00     Print times pinned to a time\n  \
        woti add PST                Add by timezone abbreviation\n  \
        woti add Bucharest          Add by city name\n  \
        woti add America/New_York   Add by IANA identifier\n  \
        woti remove PST             Remove a timezone"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Pin the TUI to a specific date (ISO 8601: YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,

    /// Pin the TUI to a specific time (ISO 8601: HH:MM or HH:MM:SS)
    #[arg(long)]
    pub time: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a timezone by abbreviation, city name, or IANA identifier
    #[command(
        after_help = "\x1b[1mAccepted formats:\x1b[0m\n  \
            Abbreviation:  PST, EET, CET, EST, JST, ...\n  \
            City name:     Bucharest, \"San Jose\", Tokyo, ...\n  \
            IANA zone:     America/New_York, Europe/London, ...\n\n\
            \x1b[1mExamples:\x1b[0m\n  \
            woti add PST\n  \
            woti add Bucharest\n  \
            woti add America/New_York"
    )]
    Add {
        /// Timezone abbreviation, city name, or IANA identifier
        zone: Vec<String>,
    },
    /// Print times for all configured timezones to stdout
    #[command(
        after_help = "\x1b[1mExamples:\x1b[0m\n  \
            woti print\n  \
            woti print --date 2026-04-15\n  \
            woti print --time 15:00\n  \
            woti print --date 2026-04-15 --time 14:00"
    )]
    Print {
        /// Pin output to a specific date (ISO 8601: YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Pin output to a specific time (ISO 8601: HH:MM or HH:MM:SS)
        #[arg(long)]
        time: Option<String>,
    },
    /// Remove a previously added timezone
    #[command(
        after_help = "\x1b[1mExamples:\x1b[0m\n  \
            woti remove PST\n  \
            woti remove Bucharest\n  \
            woti remove America/New_York\n  \
            woti remove --reset           Remove all custom timezones"
    )]
    Remove {
        /// Remove all custom timezones and restore defaults (Local + UTC)
        #[arg(long, conflicts_with = "zone")]
        reset: bool,

        /// Timezone abbreviation, city name, or IANA identifier
        #[arg(required_unless_present = "reset")]
        zone: Vec<String>,
    },
}
