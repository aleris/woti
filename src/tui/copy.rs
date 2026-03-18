use std::time::Instant;

use chrono::{Datelike, Offset, Timelike, Utc};
use chrono_tz::Tz;

use super::app::App;
use super::render::compute_datetime_for_hour;

impl App {
    pub(super) fn copy_selection(&mut self) {
        let text = self.build_copy_text();
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
            Ok(_) => self.copied_at = Some(Instant::now()),
            Err(_) => {}
        }
    }

    pub(super) fn build_copy_text(&self) -> String {
        let now_utc = Utc::now();
        let mut lines = Vec::new();
        let mut ref_date = None;

        for entry in &self.config.timezones {
            let tz: Tz = entry.iana_id.parse().unwrap_or(chrono_tz::UTC);
            let now_tz = now_utc.with_timezone(&tz);
            let selected_dt = compute_datetime_for_hour(tz, now_tz, self.hour_offset);

            let tz_abbr = selected_dt.format("%Z").to_string();
            let hour_in_day = selected_dt.hour();
            let offset_m = (selected_dt.offset().fix().local_minus_utc().abs() % 3600) / 60;

            let use_24h = self.use_24h_for_tz(&entry.iana_id);
            let time_str = if use_24h {
                format!("{:02}:{:02}", hour_in_day, offset_m)
            } else {
                let h12 = hour_in_day % 12;
                let h12 = if h12 == 0 { 12 } else { h12 };
                let ampm = if hour_in_day < 12 { "am" } else { "pm" };
                if offset_m != 0 {
                    format!("{}:{:02}{}", h12, offset_m, ampm)
                } else {
                    format!("{}{}", h12, ampm)
                }
            };

            let date = selected_dt.date_naive();
            let day_suffix = match ref_date {
                None => {
                    ref_date = Some(date);
                    String::new()
                }
                Some(ref_d) if date != ref_d => {
                    format!(
                        " {} {}",
                        selected_dt.format("%a").to_string().to_uppercase(),
                        selected_dt.day()
                    )
                }
                _ => String::new(),
            };

            let label = if entry.city == tz_abbr {
                entry.city.clone()
            } else {
                format!("{} / {}", entry.city, tz_abbr)
            };
            lines.push(format!("{} {}{}", label, time_str, day_suffix));
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, TimeFormat, TimezoneEntry};
    use super::super::app::App;

    fn app_with(entries: Vec<TimezoneEntry>, format: TimeFormat) -> App {
        let config = AppConfig {
            timezones: entries,
            time_format: Some(format),
        };
        App {
            config,
            hour_offset: 0,
            scroll_offset: 0,
            time_format: format,
            system_use_24h: false,
            should_quit: false,
            copied_at: None,
        }
    }

    fn entry(iana_id: &str, city: &str) -> TimezoneEntry {
        TimezoneEntry {
            iana_id: iana_id.to_string(),
            city: city.to_string(),
            region: "Test".to_string(),
            is_default: false,
        }
    }

    #[test]
    fn whole_hour_24h_shows_colon_00() {
        let app = app_with(vec![entry("UTC", "UTC")], TimeFormat::H24);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains(":00"),
            "whole-hour 24h should contain :00, got: {line}"
        );
    }

    #[test]
    fn half_hour_24h_shows_colon_30() {
        let app = app_with(vec![entry("Asia/Kolkata", "Bangalore")], TimeFormat::H24);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains(":30"),
            "half-hour 24h should contain :30, got: {line}"
        );
    }

    #[test]
    fn whole_hour_12h_omits_minutes() {
        let app = app_with(vec![entry("UTC", "UTC")], TimeFormat::AmPm);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        // Time part looks like "5pm" or "12am" — no colon
        let time_part = line.split_whitespace().last().unwrap();
        assert!(
            !time_part.contains(':'),
            "whole-hour 12h should have no colon in time, got: {line}"
        );
    }

    #[test]
    fn half_hour_12h_shows_colon_30() {
        let app = app_with(vec![entry("Asia/Kolkata", "Bangalore")], TimeFormat::AmPm);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains(":30"),
            "half-hour 12h should contain :30, got: {line}"
        );
        assert!(
            line.contains("am") || line.contains("pm"),
            "half-hour 12h should contain am/pm, got: {line}"
        );
    }

    #[test]
    fn quarter_hour_24h_shows_colon_45() {
        let app = app_with(vec![entry("Asia/Kathmandu", "Kathmandu")], TimeFormat::H24);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains(":45"),
            "quarter-hour 24h should contain :45, got: {line}"
        );
    }

    #[test]
    fn quarter_hour_12h_shows_colon_45() {
        let app = app_with(vec![entry("Asia/Kathmandu", "Kathmandu")], TimeFormat::AmPm);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains(":45"),
            "quarter-hour 12h should contain :45, got: {line}"
        );
    }

    #[test]
    fn copy_includes_city_and_tz_abbr() {
        let app = app_with(vec![entry("Asia/Kolkata", "Bangalore")], TimeFormat::H24);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(line.starts_with("Bangalore / IST"), "got: {line}");
    }

    #[test]
    fn utc_does_not_duplicate_city_and_abbr() {
        let app = app_with(vec![entry("UTC", "UTC")], TimeFormat::H24);
        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.starts_with("UTC ") && !line.contains("/ UTC"),
            "UTC should not duplicate city/abbr, got: {line}"
        );
    }

    #[test]
    fn multi_zone_produces_multiple_lines() {
        let app = app_with(
            vec![
                entry("UTC", "UTC"),
                entry("Asia/Kolkata", "Bangalore"),
            ],
            TimeFormat::H24,
        );
        let text = app.build_copy_text();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains(":00"), "UTC line: {}", lines[0]);
        assert!(lines[1].contains(":30"), "Kolkata line: {}", lines[1]);
    }
}
