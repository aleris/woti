use crate::config::TimeFormat;
use crate::tz_data;

use super::app::App;

pub(super) fn detect_use_24h() -> bool {
    #[cfg(target_os = "macos")]
    {
        if let Some(val) = macos_24h_preference() {
            return val;
        }
    }

    let locale = std::env::var("LC_TIME")
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_default()
        .to_lowercase();

    !(locale.contains("_us") || locale.contains("_ph") || locale.contains("_au"))
}

#[cfg(target_os = "macos")]
fn macos_24h_preference() -> Option<bool> {
    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "NSGlobalDomain", "AppleICUForce24HourTime"])
        .output()
    {
        if output.status.success() {
            match String::from_utf8_lossy(&output.stdout).trim() {
                "1" => return Some(true),
                "0" => return Some(false),
                _ => {}
            }
        }
    }

    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "NSGlobalDomain", "AppleICUForce12HourTime"])
        .output()
    {
        if output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "1" {
            return Some(false);
        }
    }

    None
}

impl App {
    pub(super) fn use_24h_for_header(&self) -> bool {
        match self.time_format {
            TimeFormat::H24 => true,
            TimeFormat::AmPm => false,
            TimeFormat::Mixed => self.system_use_24h,
        }
    }

    pub(super) fn use_24h_for_tz(&self, iana_id: &str) -> bool {
        match self.time_format {
            TimeFormat::H24 => true,
            TimeFormat::AmPm => false,
            TimeFormat::Mixed => !tz_data::uses_12h_clock(iana_id),
        }
    }

    pub(super) fn cycle_time_format(&mut self) {
        self.time_format = match self.time_format {
            TimeFormat::Mixed => TimeFormat::AmPm,
            TimeFormat::AmPm => TimeFormat::H24,
            TimeFormat::H24 => TimeFormat::Mixed,
        };
        self.config.time_format = Some(self.time_format);
        let _ = self.config.save();
    }
}
