use crate::config::TimeFormat;
use crate::tz_data;

use super::app::App;

impl App {
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
