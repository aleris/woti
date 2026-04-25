use std::time::Instant;

use chrono::{DateTime, Datelike, Duration, Offset, Timelike, Utc};
use chrono_tz::Tz;

use crate::config::TimezoneEntry;

use super::app::App;

/// Floor a UTC datetime to the most recent boundary of `interval_minutes`.
/// Flooring in UTC (rather than in each timezone's wall clock) preserves the
/// legacy copy semantics where Kolkata renders `:30`, Nepal `:45`, and UTC
/// `:00` — i.e. each timezone's natural minute boundary derived from its
/// UTC offset.
fn floor_utc_to_interval(dt: DateTime<Utc>, interval_minutes: i32) -> DateTime<Utc> {
    let interval = interval_minutes.max(1);
    let minute = dt.minute() as i32;
    let floored_minute = (minute / interval) * interval;
    let drop = minute - floored_minute;
    dt - Duration::minutes(drop as i64)
        - Duration::seconds(dt.second() as i64)
        - Duration::nanoseconds(dt.nanosecond() as i64)
}

/// Build the copy/print text for the configured timezones at
/// `reference_utc + offset_minutes`, snapped to the active `interval_minutes`
/// grid (in UTC). The default H1 path (`interval_minutes = 60`) reproduces
/// the legacy "show wall hour with timezone-natural minute" output
/// byte-for-byte.
pub fn build_copy_text(
    timezones: &[TimezoneEntry],
    reference_utc: DateTime<Utc>,
    offset_minutes: i32,
    interval_minutes: i32,
    use_24h_for_tz: &dyn Fn(&str) -> bool,
) -> String {
    let mut lines = Vec::new();
    let mut ref_date: Option<chrono::NaiveDate> = None;

    let anchor_utc = floor_utc_to_interval(reference_utc, interval_minutes)
        + Duration::minutes(offset_minutes as i64);

    for entry in timezones {
        let tz: Tz = entry.iana_id.parse().unwrap_or(chrono_tz::UTC);
        let selected_dt = anchor_utc.with_timezone(&tz);
        let now_tz = reference_utc.with_timezone(&tz);
        let _ = now_tz;

        let tz_abbr = crate::tz_data::display_abbreviation(&selected_dt);
        let hour_in_day = selected_dt.hour();
        let actual_minute = selected_dt.minute() as i32;
        let _ = (selected_dt.offset().fix().local_minus_utc().abs() % 3600) / 60;

        let use_24h = use_24h_for_tz(&entry.iana_id);
        let time_str = if use_24h {
            format!("{:02}:{:02}", hour_in_day, actual_minute)
        } else {
            let h12 = hour_in_day % 12;
            let h12 = if h12 == 0 { 12 } else { h12 };
            let ampm = if hour_in_day < 12 { "am" } else { "pm" };
            if actual_minute != 0 {
                format!("{}:{:02}{}", h12, actual_minute, ampm)
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
                let mut suffix = format!(
                    " {} {}",
                    selected_dt.format("%a").to_string().to_uppercase(),
                    selected_dt.day()
                );
                if date.month() != ref_d.month() || date.year() != ref_d.year() {
                    suffix.push_str(&format!(", {}", selected_dt.format("%B")));
                    if date.year() != ref_d.year() {
                        suffix.push_str(&format!(", {}", selected_dt.format("%Y")));
                    }
                }
                suffix
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

impl App {
    pub(super) fn copy_selection(&mut self) {
        let text = self.build_copy_text();
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
            Ok(_) => self.copied_at = Some(Instant::now()),
            Err(_) => {}
        }
    }

    pub(super) fn build_copy_text(&self) -> String {
        let reference_utc = self.reference_time();
        let time_format = self.time_format;
        let interval_minutes = self.interval.minutes() as i32;
        let offset_minutes = self.cell_offset * interval_minutes;
        build_copy_text(
            &self.config.timezones,
            reference_utc,
            offset_minutes,
            interval_minutes,
            &|iana_id| Self::use_24h_static(time_format, iana_id),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, DEFAULT_INTERVAL_MINUTES, TimeFormat, TimezoneEntry, WorkingHoursConfig};
    use super::super::app::App;
    use super::super::NavInterval;

    fn app_with(entries: Vec<TimezoneEntry>, format: TimeFormat) -> App {
        app_with_interval(entries, format, NavInterval::H1)
    }

    fn app_with_interval(
        entries: Vec<TimezoneEntry>,
        format: TimeFormat,
        interval: NavInterval,
    ) -> App {
        let config = AppConfig {
            timezones: entries,
            time_format: Some(format),
            working_hours: WorkingHoursConfig::default(),
            interval: DEFAULT_INTERVAL_MINUTES,
        };
        App::new(config, None, interval)
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

    #[test]
    fn copy_includes_month_when_month_differs() {
        use chrono::{Datelike, Timelike, Utc};

        let now_utc = Utc::now();
        let year = now_utc.year();
        let target_year = if now_utc.month() > 3
            || (now_utc.month() == 3 && now_utc.day() == 31 && now_utc.hour() >= 11)
        {
            year + 1
        } else {
            year
        };
        let target = chrono::NaiveDate::from_ymd_opt(target_year, 3, 31)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap();
        let target_utc =
            chrono::DateTime::<Utc>::from_naive_utc_and_offset(target, Utc);
        let hour_offset = target_utc
            .signed_duration_since(now_utc)
            .num_hours() as i32;

        let mut app = app_with(
            vec![
                entry("UTC", "UTC"),
                entry("Pacific/Kiritimati", "Kiritimati"),
            ],
            TimeFormat::H24,
        );
        app.cell_offset = hour_offset;
        let text = app.build_copy_text();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(
            lines[1].contains(", April"),
            "should include month when month differs, got: {}",
            lines[1]
        );
    }

    #[test]
    fn copy_includes_year_when_year_differs() {
        use chrono::{Datelike, Timelike, Utc};

        let now_utc = Utc::now();
        let year = now_utc.year();
        let target_year = if now_utc.month() == 12
            && now_utc.day() == 31
            && now_utc.hour() >= 11
        {
            year + 1
        } else {
            year
        };
        let target = chrono::NaiveDate::from_ymd_opt(target_year, 12, 31)
            .unwrap()
            .and_hms_opt(11, 0, 0)
            .unwrap();
        let target_utc =
            chrono::DateTime::<Utc>::from_naive_utc_and_offset(target, Utc);
        let hour_offset = target_utc
            .signed_duration_since(now_utc)
            .num_hours() as i32;

        let mut app = app_with(
            vec![
                entry("UTC", "UTC"),
                entry("Pacific/Kiritimati", "Kiritimati"),
            ],
            TimeFormat::H24,
        );
        app.cell_offset = hour_offset;
        let text = app.build_copy_text();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        let next_year = target_year + 1;
        assert!(
            lines[1].contains(&format!(", January, {next_year}")),
            "should include month and year when year differs, got: {}",
            lines[1]
        );
    }

    #[test]
    fn anchor_time_pins_output() {
        use chrono::{NaiveDate, TimeZone, Utc};

        let anchor = NaiveDate::from_ymd_opt(2026, 7, 4)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let anchor_utc = Utc.from_utc_datetime(&anchor);

        let config = AppConfig {
            timezones: vec![entry("UTC", "UTC")],
            time_format: Some(TimeFormat::H24),
            working_hours: WorkingHoursConfig::default(),
            interval: DEFAULT_INTERVAL_MINUTES,
        };
        let app = App::new(config, Some(anchor_utc), NavInterval::H1);

        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains("12:00"),
            "anchored at 12:00 UTC should show 12:00, got: {line}"
        );
    }

    #[test]
    fn standalone_matches_app_wrapper() {
        use chrono::{NaiveDate, TimeZone, Utc};
        use super::build_copy_text;

        let anchor = NaiveDate::from_ymd_opt(2026, 7, 4)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let anchor_utc = Utc.from_utc_datetime(&anchor);

        let entries = vec![
            entry("UTC", "UTC"),
            entry("Asia/Kolkata", "Bangalore"),
            entry("America/Los_Angeles", "San Jose"),
        ];
        let config = AppConfig {
            timezones: entries.clone(),
            time_format: Some(TimeFormat::H24),
            working_hours: WorkingHoursConfig::default(),
            interval: DEFAULT_INTERVAL_MINUTES,
        };
        let app = App::new(config, Some(anchor_utc), NavInterval::H1);

        let from_app = app.build_copy_text();
        let from_standalone = build_copy_text(&entries, anchor_utc, 0, 60, &|_| true);

        assert_eq!(from_app, from_standalone);
    }

    // --- Spec: "Sub-hour selection in copy text" ---

    #[test]
    fn m15_cell_offset_one_yields_15_minute_time() {
        use chrono::{NaiveDate, TimeZone, Utc};

        let anchor = NaiveDate::from_ymd_opt(2026, 4, 25)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let anchor_utc = Utc.from_utc_datetime(&anchor);

        let mut app = app_with_interval(
            vec![entry("UTC", "UTC")],
            TimeFormat::H24,
            NavInterval::M15,
        );
        app.anchor_time = Some(anchor_utc);
        app.cell_offset = 1;

        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains("14:15"),
            "M15 + cell_offset=1 from 14:00 should yield :15, got: {line}"
        );
    }

    #[test]
    fn m30_cell_offset_one_yields_30_minute_time() {
        use chrono::{NaiveDate, TimeZone, Utc};

        let anchor = NaiveDate::from_ymd_opt(2026, 4, 25)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        let anchor_utc = Utc.from_utc_datetime(&anchor);

        let mut app = app_with_interval(
            vec![entry("UTC", "UTC")],
            TimeFormat::AmPm,
            NavInterval::M30,
        );
        app.anchor_time = Some(anchor_utc);
        app.cell_offset = 1;

        let text = app.build_copy_text();
        let line = text.lines().next().unwrap();
        assert!(
            line.contains("9:30am"),
            "M30 + cell_offset=1 from 9:00 should yield 9:30am, got: {line}"
        );
    }

    #[test]
    fn standalone_with_pinned_time() {
        use chrono::{NaiveDate, TimeZone, Utc};
        use super::build_copy_text;

        let anchor = NaiveDate::from_ymd_opt(2026, 4, 15)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let anchor_utc = Utc.from_utc_datetime(&anchor);

        let entries = vec![entry("UTC", "UTC")];
        let text = build_copy_text(&entries, anchor_utc, 0, 60, &|_| true);
        assert!(
            text.contains("14:00"),
            "pinned at 14:00 UTC should show 14:00, got: {text}"
        );
    }
}
