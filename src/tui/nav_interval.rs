use super::app::App;

impl App {
    /// Cycle the navigation interval to the next value (`H1 → M30 → M15 → H1`).
    ///
    /// Rescales `cell_offset` so the absolute selected time is preserved when
    /// the interval narrows, and snaps to the nearest cell (round-half-away-
    /// from-zero) when the interval widens. Persists the new interval to
    /// `config.toml` (errors are intentionally ignored, matching
    /// [`App::cycle_time_format`]).
    pub(super) fn cycle_interval(&mut self) {
        self.cycle_interval_inner();
        let _ = self.config.save();
    }

    /// Pure-state portion of [`App::cycle_interval`] (no I/O). Exposed for tests.
    pub(super) fn cycle_interval_inner(&mut self) {
        let old_minutes = self.interval.minutes() as i32;
        let new_interval = self.interval.next();
        let new_minutes = new_interval.minutes() as i32;

        let total_minutes = self.cell_offset * old_minutes;
        let half = new_minutes / 2;
        let rounded = if total_minutes >= 0 {
            (total_minutes + half) / new_minutes
        } else {
            (total_minutes - half) / new_minutes
        };

        self.cell_offset = rounded;
        self.interval = new_interval;
        self.config.interval = new_interval.minutes();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, DEFAULT_INTERVAL_MINUTES, TimeFormat, WorkingHoursConfig};
    use crate::tui::NavInterval;

    fn app_with(interval: NavInterval, cell_offset: i32) -> App {
        let config = AppConfig {
            timezones: vec![],
            time_format: Some(TimeFormat::H24),
            working_hours: WorkingHoursConfig::default(),
            interval: DEFAULT_INTERVAL_MINUTES,
        };
        let mut app = App::new(config, None, interval);
        app.cell_offset = cell_offset;
        app
    }

    #[test]
    fn cycle_h1_to_m30_doubles_cell_offset() {
        let mut app = app_with(NavInterval::H1, 3);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::M30);
        assert_eq!(app.cell_offset, 6);
    }

    #[test]
    fn cycle_m30_to_m15_doubles_cell_offset() {
        let mut app = app_with(NavInterval::M30, -4);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::M15);
        assert_eq!(app.cell_offset, -8);
    }

    #[test]
    fn cycle_m15_to_h1_rounds_to_nearest_hour_down_for_15() {
        // +15 min absolute -> nearest hour is 0 (round half-away-from-zero on +30 only).
        let mut app = app_with(NavInterval::M15, 1);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::H1);
        assert_eq!(app.cell_offset, 0);
    }

    #[test]
    fn cycle_m15_to_h1_rounds_to_nearest_hour_up_for_45() {
        // +45 min absolute -> nearest hour is +1.
        let mut app = app_with(NavInterval::M15, 3);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::H1);
        assert_eq!(app.cell_offset, 1);
    }

    #[test]
    fn cycle_m15_to_h1_rounds_negative_correctly() {
        let mut app = app_with(NavInterval::M15, -3);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::H1);
        assert_eq!(app.cell_offset, -1);

        let mut app2 = app_with(NavInterval::M15, -1);
        app2.cycle_interval_inner();
        assert_eq!(app2.cell_offset, 0);
    }

    #[test]
    fn cycle_wrap_order_h1_m30_m15_h1() {
        let mut app = app_with(NavInterval::H1, 0);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::M30);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::M15);
        app.cycle_interval_inner();
        assert_eq!(app.interval, NavInterval::H1);
    }

    #[test]
    fn cycle_persists_to_config() {
        let mut app = app_with(NavInterval::H1, 0);
        app.cycle_interval_inner();
        assert_eq!(app.config.interval, 30);
        app.cycle_interval_inner();
        assert_eq!(app.config.interval, 15);
        app.cycle_interval_inner();
        assert_eq!(app.config.interval, 60);
    }

    // --- Spec 8.5: format_offset reflects sub-hour selection ---

    #[test]
    fn m15_cell_offset_two_yields_in_30_minutes_indicator() {
        use crate::tui::format_offset;
        let app = app_with(NavInterval::M15, 2);
        let minutes = app.cell_offset * app.interval.minutes() as i32;
        assert_eq!(minutes, 30);
        assert_eq!(format_offset(minutes), "in 30 minutes");
    }

    // --- Spec 9.1: persistence round-trip via TOML serialization ---
    //
    // We exercise the TOML serializer/deserializer directly rather than
    // touching `~/.config/woti/config.toml` so the test is hermetic.

    #[test]
    fn cycle_interval_value_round_trips_through_toml() {
        let mut app = app_with(NavInterval::H1, 0);

        app.cycle_interval_inner();
        assert_eq!(app.config.interval, 30);
        let toml_str = toml::to_string_pretty(&app.config).expect("serialize");
        let reloaded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(reloaded.interval, 30);

        app.cycle_interval_inner();
        let toml_str = toml::to_string_pretty(&app.config).expect("serialize");
        let reloaded: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(reloaded.interval, 15);
    }

    // --- Spec 9.2: launch resolution order (flag > config > default) ---

    #[test]
    fn launch_resolution_flag_wins_over_config() {
        let chosen = crate::tui::resolve_launch_interval(Some(NavInterval::M15), 30);
        assert_eq!(chosen, NavInterval::M15);
    }

    #[test]
    fn launch_resolution_config_wins_over_default_when_no_flag() {
        let chosen = crate::tui::resolve_launch_interval(None, 30);
        assert_eq!(chosen, NavInterval::M30);
    }

    #[test]
    fn launch_resolution_falls_back_to_h1_for_default_or_invalid() {
        // Default config value (60) → H1
        assert_eq!(
            crate::tui::resolve_launch_interval(None, 60),
            NavInterval::H1
        );
        // Out-of-range config value → H1 fallback
        assert_eq!(
            crate::tui::resolve_launch_interval(None, 7),
            NavInterval::H1
        );
        assert_eq!(
            crate::tui::resolve_launch_interval(None, 0),
            NavInterval::H1
        );
    }
}
