mod cli;
mod config;
mod timezone;
mod tui;
mod tz_data;

use clap::Parser;

use cli::{Cli, Command};
use config::AppConfig;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add { zone }) => cmd_add(&zone.join(" ")),
        Some(Command::Remove { zone, reset }) => {
            if reset {
                cmd_reset();
            } else {
                cmd_remove(&zone.join(" "));
            }
        }
        None => cmd_tui(),
    }
}

fn cmd_add(input: &str) {
    let mut config = AppConfig::load();

    match timezone::resolve(input) {
        Ok(entry) => {
            if config.has_iana(&entry.iana_id) {
                eprintln!(
                    "Timezone already configured: {} / {} ({})",
                    entry.city, entry.region, entry.iana_id
                );
                std::process::exit(1);
            }
            let msg = format!(
                "Added: {} / {} ({})",
                entry.city, entry.region, entry.iana_id
            );
            config.add(entry);
            if let Err(e) = config.save() {
                eprintln!("Error saving config: {e}");
                std::process::exit(1);
            }
            println!("{msg}");
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn cmd_remove(input: &str) {
    let mut config = AppConfig::load();

    match timezone::resolve(input) {
        Ok(entry) => {
            if let Some(existing) = config.timezones.iter().find(|e| e.iana_id == entry.iana_id) {
                if existing.is_default {
                    eprintln!(
                        "Cannot remove default timezone: {} ({})",
                        existing.city, existing.iana_id
                    );
                    std::process::exit(1);
                }
            }

            match config.remove_by_iana(&entry.iana_id) {
                Some(removed) => {
                    if let Err(e) = config.save() {
                        eprintln!("Error saving config: {e}");
                        std::process::exit(1);
                    }
                    println!(
                        "Removed: {} / {} ({})",
                        removed.city, removed.region, removed.iana_id
                    );
                }
                None => {
                    eprintln!(
                        "Timezone not in your list: {} ({})",
                        entry.city, entry.iana_id
                    );
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn cmd_reset() {
    let mut config = AppConfig::load();
    let removed = config.reset();

    if removed == 0 {
        println!("No custom timezones to remove. Config already at defaults.");
        return;
    }

    if let Err(e) = config.save() {
        eprintln!("Error saving config: {e}");
        std::process::exit(1);
    }
    println!("Reset: removed {removed} custom timezone(s). Defaults restored (Local + UTC).");
}

fn cmd_tui() {
    let config = AppConfig::load();
    let mut app = tui::App::new(config);
    if let Err(e) = app.run() {
        eprintln!("TUI error: {e}");
        std::process::exit(1);
    }
}
