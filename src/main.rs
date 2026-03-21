mod cli;
mod config;
mod timezone;
mod tui;
mod tz_data;

use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use clap::Parser;

use cli::{Cli, Command};
use config::AppConfig;

fn main() {
    let cli = Cli::parse();

    if cli.command.is_some() && (cli.date.is_some() || cli.time.is_some()) {
        eprintln!("error: --date and --time can only be used when launching the TUI (without a subcommand)");
        std::process::exit(2);
    }

    match cli.command {
        Some(Command::Add { zone }) => cmd_add(&zone.join(" ")),
        Some(Command::Remove { zone, reset }) => {
            if reset {
                cmd_reset();
            } else {
                cmd_remove(&zone.join(" "));
            }
        }
        Some(Command::Print { date, time }) => {
            let anchor = parse_anchor(date.as_deref(), time.as_deref());
            cmd_print(anchor);
        }
        None => {
            let anchor = parse_anchor(cli.date.as_deref(), cli.time.as_deref());
            cmd_tui(anchor);
        }
    }
}

fn parse_anchor(date: Option<&str>, time: Option<&str>) -> Option<DateTime<Utc>> {
    if date.is_none() && time.is_none() {
        return None;
    }

    let naive_date = match date {
        Some(d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(nd) => nd,
            Err(_) => {
                eprintln!("Invalid --date format: \"{d}\". Expected ISO 8601: YYYY-MM-DD (e.g. 2026-04-15)");
                std::process::exit(1);
            }
        },
        None => Local::now().date_naive(),
    };

    let naive_time = match time {
        Some(t) => match NaiveTime::parse_from_str(t, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(t, "%H:%M"))
        {
            Ok(nt) => nt,
            Err(_) => {
                eprintln!("Invalid --time format: \"{t}\". Expected ISO 8601: HH:MM or HH:MM:SS (e.g. 15:00)");
                std::process::exit(1);
            }
        },
        None => Local::now().time(),
    };

    let naive_dt = naive_date.and_time(naive_time);
    let local_dt = match Local.from_local_datetime(&naive_dt).single() {
        Some(dt) => dt,
        None => {
            eprintln!("Ambiguous or invalid local datetime: {naive_dt}");
            std::process::exit(1);
        }
    };

    Some(local_dt.with_timezone(&Utc))
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

fn cmd_print(anchor: Option<DateTime<Utc>>) {
    let config = AppConfig::load();
    let time_format = config.time_format.unwrap_or(config::TimeFormat::Mixed);
    let reference_utc = anchor.unwrap_or_else(Utc::now);
    let text = tui::build_copy_text(
        &config.timezones,
        reference_utc,
        0,
        &|iana_id| tui::use_24h_for_format(time_format, iana_id),
    );
    println!("{text}");
}

fn cmd_tui(anchor: Option<DateTime<Utc>>) {
    let config = AppConfig::load();
    let mut app = tui::App::new(config, anchor);
    if let Err(e) = app.run() {
        eprintln!("TUI error: {e}");
        std::process::exit(1);
    }
}
