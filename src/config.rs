use std::fs;
use std::path::PathBuf;

use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::tz_data;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeFormat {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "ampm")]
    AmPm,
    #[serde(rename = "mixed")]
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimezoneEntry {
    pub iana_id: String,
    pub city: String,
    pub region: String,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WorkingHoursConfig {
    #[serde(default = "WorkingHoursConfig::default_enabled")]
    pub enabled: bool,
    #[serde(default = "WorkingHoursConfig::default_work_start")]
    pub work_start: u8,
    #[serde(default = "WorkingHoursConfig::default_work_end")]
    pub work_end: u8,
    #[serde(default = "WorkingHoursConfig::default_transition_start")]
    pub transition_start: u8,
    #[serde(default = "WorkingHoursConfig::default_transition_end")]
    pub transition_end: u8,
}

impl Default for WorkingHoursConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            work_start: 9,
            work_end: 18,
            transition_start: 7,
            transition_end: 20,
        }
    }
}

impl WorkingHoursConfig {
    fn default_enabled() -> bool { true }
    fn default_work_start() -> u8 { 9 }
    fn default_work_end() -> u8 { 18 }
    fn default_transition_start() -> u8 { 7 }
    fn default_transition_end() -> u8 { 20 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub timezones: Vec<TimezoneEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_format: Option<TimeFormat>,
    #[serde(default)]
    pub working_hours: WorkingHoursConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let local_iana = localtime_iana().unwrap_or_else(|| "UTC".to_string());
        let (city, region) = tz_data::city_and_region(&local_iana);

        let mut timezones = vec![];

        if local_iana != "UTC" {
            timezones.push(TimezoneEntry {
                iana_id: "UTC".to_string(),
                city: "UTC".to_string(),
                region: "Coordinated Universal Time".to_string(),
                is_default: true,
            });
        }

        timezones.push(TimezoneEntry {
            iana_id: local_iana.clone(),
            city,
            region,
            is_default: true,
        });

        Self {
            timezones,
            time_format: None,
            working_hours: WorkingHoursConfig::default(),
        }
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
    fn xdg_config_path() -> Option<PathBuf> {
        home_dir().map(|h| h.join(".config").join("woti").join("config.toml"))
    }

    fn legacy_config_path() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            home_dir().map(|h| {
                h.join("Library")
                    .join("Application Support")
                    .join("woti")
                    .join("config.toml")
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            None
        }
    }

    pub fn config_path() -> Option<PathBuf> {
        let xdg = Self::xdg_config_path();
        if xdg.as_ref().is_some_and(|p| p.exists()) {
            return xdg;
        }

        let legacy = Self::legacy_config_path();
        if legacy.as_ref().is_some_and(|p| p.exists()) {
            return legacy;
        }

        xdg
    }

    fn save_path() -> Option<PathBuf> {
        Self::xdg_config_path()
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
        let Some(path) = Self::save_path() else {
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
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
        config.add(make_entry("America/Los_Angeles", "Los Angeles", false));
        assert!(config.has_iana("America/Los_Angeles"));
    }

    #[test]
    fn has_iana_returns_false_for_missing() {
        let config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
        assert!(!config.has_iana("America/Los_Angeles"));
    }

    // --- Spec: "Remove timezone" ---

    #[test]
    fn remove_existing_entry() {
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
        config.add(make_entry("Asia/Tokyo", "Tokyo", false));
        let removed = config.remove_by_iana("Asia/Tokyo");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().iana_id, "Asia/Tokyo");
        assert!(!config.has_iana("Asia/Tokyo"));
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
        assert!(config.remove_by_iana("Asia/Tokyo").is_none());
    }

    // --- Spec: "Defaults persist after adding custom zones" ---

    #[test]
    fn add_preserves_existing_entries() {
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
        config.add(make_entry("UTC", "UTC", true));
        config.add(make_entry("America/Los_Angeles", "Los Angeles", false));
        assert_eq!(config.timezones.len(), 2);
        assert!(config.has_iana("UTC"));
        assert!(config.has_iana("America/Los_Angeles"));
    }

    // --- Spec: "Reset to defaults" ---

    #[test]
    fn reset_removes_custom_timezones() {
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
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
        let mut config = AppConfig { timezones: vec![], time_format: None, working_hours: WorkingHoursConfig::default() };
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
            time_format: None,
            working_hours: WorkingHoursConfig::default(),
        };
        let toml_str = toml::to_string_pretty(&config).expect("serialize");
        let loaded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(loaded.timezones.len(), 2);
        assert_eq!(loaded.timezones[0].iana_id, "UTC");
        assert!(loaded.timezones[0].is_default);
        assert_eq!(loaded.timezones[1].iana_id, "America/New_York");
        assert!(!loaded.timezones[1].is_default);
    }

    // --- Spec: "Configurable hour ranges" / "Shading enabled flag" ---

    #[test]
    fn working_hours_defaults_when_section_absent() {
        let toml_str = r#"
[[timezones]]
iana_id = "UTC"
city = "UTC"
region = "Coordinated Universal Time"
"#;
        let loaded: AppConfig = toml::from_str(toml_str).expect("deserialize");
        let wh = loaded.working_hours;
        assert!(wh.enabled);
        assert_eq!(wh.work_start, 9);
        assert_eq!(wh.work_end, 18);
        assert_eq!(wh.transition_start, 7);
        assert_eq!(wh.transition_end, 20);
    }

    #[test]
    fn working_hours_round_trip_with_custom_values() {
        let config = AppConfig {
            timezones: vec![make_entry("UTC", "UTC", true)],
            time_format: None,
            working_hours: WorkingHoursConfig {
                enabled: false,
                work_start: 8,
                work_end: 17,
                transition_start: 6,
                transition_end: 19,
            },
        };
        let toml_str = toml::to_string_pretty(&config).expect("serialize");
        let loaded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert!(!loaded.working_hours.enabled);
        assert_eq!(loaded.working_hours.work_start, 8);
        assert_eq!(loaded.working_hours.work_end, 17);
        assert_eq!(loaded.working_hours.transition_start, 6);
        assert_eq!(loaded.working_hours.transition_end, 19);
    }

    #[test]
    fn working_hours_partial_fields_use_defaults() {
        let toml_str = r#"
[[timezones]]
iana_id = "UTC"
city = "UTC"
region = "Coordinated Universal Time"

[working_hours]
enabled = false
"#;
        let loaded: AppConfig = toml::from_str(toml_str).expect("deserialize");
        assert!(!loaded.working_hours.enabled);
        assert_eq!(loaded.working_hours.work_start, 9);
        assert_eq!(loaded.working_hours.work_end, 18);
    }

    // --- Spec: "XDG config path" ---

    #[test]
    fn xdg_config_path_ends_with_expected_suffix() {
        let path = AppConfig::xdg_config_path().expect("home dir should resolve");
        assert!(
            path.ends_with(".config/woti/config.toml"),
            "expected path ending .config/woti/config.toml, got {path:?}"
        );
    }

    #[test]
    fn save_path_equals_xdg_path() {
        assert_eq!(AppConfig::save_path(), AppConfig::xdg_config_path());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn legacy_config_path_is_app_support_on_macos() {
        let path = AppConfig::legacy_config_path().expect("home dir should resolve");
        assert!(
            path.ends_with("Library/Application Support/woti/config.toml"),
            "expected macOS legacy path, got {path:?}"
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn legacy_config_path_is_none_on_non_macos() {
        assert!(AppConfig::legacy_config_path().is_none());
    }
}
