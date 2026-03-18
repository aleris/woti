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
        woti add PST                Add by timezone abbreviation\n  \
        woti add Bucharest          Add by city name\n  \
        woti add America/New_York   Add by IANA identifier\n  \
        woti remove PST             Remove a timezone"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
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
    /// Remove a previously added timezone
    #[command(
        after_help = "\x1b[1mExamples:\x1b[0m\n  \
            woti remove PST\n  \
            woti remove Bucharest\n  \
            woti remove America/New_York"
    )]
    Remove {
        /// Timezone abbreviation, city name, or IANA identifier
        zone: Vec<String>,
    },
}
