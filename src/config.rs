use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::tz_data;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimezoneEntry {
    pub iana_id: String,
    pub city: String,
    pub region: String,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub timezones: Vec<TimezoneEntry>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let local_iana = localtime_iana().unwrap_or_else(|| "UTC".to_string());
        let (city, region) = tz_data::city_and_region(&local_iana);

        let mut timezones = vec![TimezoneEntry {
            iana_id: local_iana.clone(),
            city,
            region,
            is_default: true,
        }];

        if local_iana != "UTC" {
            timezones.push(TimezoneEntry {
                iana_id: "UTC".to_string(),
                city: "UTC".to_string(),
                region: "Coordinated Universal Time".to_string(),
                is_default: true,
            });
        }

        Self { timezones }
    }
}

fn localtime_iana() -> Option<String> {
    // Read the /etc/localtime symlink to determine the system IANA timezone,
    // following the same approach as the time-tz crate.
    #[cfg(unix)]
    {
        let path = std::path::Path::new("/etc/localtime");
        let realpath = std::fs::read_link(path).ok()?;
        realpath
            .to_str()?
            .split("/zoneinfo/")
            .last()
            .map(|s| s.to_string())
    }
    #[cfg(not(unix))]
    {
        None
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "woti").map(|dirs| dirs.config_dir().join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let Some(path) = Self::config_path() else {
            return Err("Could not determine config directory".to_string());
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }

        let contents =
            toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize config: {e}"))?;
        fs::write(&path, contents).map_err(|e| format!("Failed to write config: {e}"))?;
        Ok(())
    }

    pub fn has_iana(&self, iana_id: &str) -> bool {
        self.timezones.iter().any(|e| e.iana_id == iana_id)
    }

    pub fn add(&mut self, entry: TimezoneEntry) {
        self.timezones.push(entry);
    }

    pub fn reset(&mut self) -> usize {
        let removed = self.timezones.iter().filter(|e| !e.is_default).count();
        self.timezones.retain(|e| e.is_default);
        removed
    }

    pub fn remove_by_iana(&mut self, iana_id: &str) -> Option<TimezoneEntry> {
        if let Some(pos) = self.timezones.iter().position(|e| e.iana_id == iana_id) {
            Some(self.timezones.remove(pos))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(iana_id: &str, city: &str, is_default: bool) -> TimezoneEntry {
        TimezoneEntry {
            iana_id: iana_id.to_string(),
            city: city.to_string(),
            region: "Test".to_string(),
            is_default,
        }
    }

    // --- Spec: "Default timezones" ---

    #[test]
    fn default_config_includes_utc() {
        let config = AppConfig::default();
        assert!(config.has_iana("UTC"));
    }

    #[test]
    fn default_config_has_at_least_local_zone() {
        let config = AppConfig::default();
        assert!(!config.timezones.is_empty());
        assert!(config.timezones[0].is_default);
    }

    #[test]
    fn default_config_all_entries_are_default() {
        let config = AppConfig::default();
        for tz in &config.timezones {
            assert!(tz.is_default, "{} should be marked as default", tz.iana_id);
        }
    }

    // --- Spec: "Duplicate prevention" ---

    #[test]
    fn has_iana_detects_existing() {
        let mut config = AppConfig { timezones: vec![] };
        config.add(make_entry("America/Los_Angeles", "Los Angeles", false));
        assert!(config.has_iana("America/Los_Angeles"));
    }

    #[test]
    fn has_iana_returns_false_for_missing() {
        let config = AppConfig { timezones: vec![] };
        assert!(!config.has_iana("America/Los_Angeles"));
    }

    // --- Spec: "Remove timezone" ---

    #[test]
    fn remove_existing_entry() {
        let mut config = AppConfig { timezones: vec![] };
        config.add(make_entry("Asia/Tokyo", "Tokyo", false));
        let removed = config.remove_by_iana("Asia/Tokyo");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().iana_id, "Asia/Tokyo");
        assert!(!config.has_iana("Asia/Tokyo"));
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let mut config = AppConfig { timezones: vec![] };
        assert!(config.remove_by_iana("Asia/Tokyo").is_none());
    }

    // --- Spec: "Defaults persist after adding custom zones" ---

    #[test]
    fn add_preserves_existing_entries() {
        let mut config = AppConfig { timezones: vec![] };
        config.add(make_entry("UTC", "UTC", true));
        config.add(make_entry("America/Los_Angeles", "Los Angeles", false));
        assert_eq!(config.timezones.len(), 2);
        assert!(config.has_iana("UTC"));
        assert!(config.has_iana("America/Los_Angeles"));
    }

    // --- Spec: "Reset to defaults" ---

    #[test]
    fn reset_removes_custom_timezones() {
        let mut config = AppConfig { timezones: vec![] };
        config.add(make_entry("UTC", "UTC", true));
        config.add(make_entry("Europe/Bucharest", "Bucharest", true));
        config.add(make_entry("America/Los_Angeles", "Los Angeles", false));
        config.add(make_entry("Asia/Tokyo", "Tokyo", false));
        config.add(make_entry("Europe/Berlin", "Berlin", false));

        let removed = config.reset();
        assert_eq!(removed, 3);
        assert_eq!(config.timezones.len(), 2);
        assert!(config.has_iana("UTC"));
        assert!(config.has_iana("Europe/Bucharest"));
        assert!(!config.has_iana("America/Los_Angeles"));
    }

    #[test]
    fn reset_with_only_defaults_returns_zero() {
        let mut config = AppConfig { timezones: vec![] };
        config.add(make_entry("UTC", "UTC", true));
        config.add(make_entry("Europe/Bucharest", "Bucharest", true));

        let removed = config.reset();
        assert_eq!(removed, 0);
        assert_eq!(config.timezones.len(), 2);
    }

    // --- Config serialization round-trip ---

    #[test]
    fn config_serializes_and_deserializes() {
        let config = AppConfig {
            timezones: vec![
                make_entry("UTC", "UTC", true),
                make_entry("America/New_York", "New York", false),
            ],
        };
        let toml_str = toml::to_string_pretty(&config).expect("serialize");
        let loaded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(loaded.timezones.len(), 2);
        assert_eq!(loaded.timezones[0].iana_id, "UTC");
        assert!(loaded.timezones[0].is_default);
        assert_eq!(loaded.timezones[1].iana_id, "America/New_York");
        assert!(!loaded.timezones[1].is_default);
    }
}
