use std::time::Instant;

use chrono::{Datelike, Timelike, Utc};
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

    fn build_copy_text(&self) -> String {
        let now_utc = Utc::now();
        let mut lines = Vec::new();
        let mut ref_date = None;

        for entry in &self.config.timezones {
            let tz: Tz = entry.iana_id.parse().unwrap_or(chrono_tz::UTC);
            let now_tz = now_utc.with_timezone(&tz);
            let selected_dt = compute_datetime_for_hour(tz, now_tz, self.hour_offset);

            let tz_abbr = selected_dt.format("%Z").to_string();
            let hour_in_day = selected_dt.hour();

            let use_24h = self.use_24h_for_tz(&entry.iana_id);
            let time_str = if use_24h {
                format!("{:02}:00", hour_in_day)
            } else {
                let h12 = hour_in_day % 12;
                let h12 = if h12 == 0 { 12 } else { h12 };
                let ampm = if hour_in_day < 12 { "am" } else { "pm" };
                format!("{}{}", h12, ampm)
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

            lines.push(format!(
                "{} / {} {}{}",
                entry.city, tz_abbr, time_str, day_suffix
            ));
        }

        lines.join("\n")
    }
}
