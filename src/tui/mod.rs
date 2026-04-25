mod app;
mod copy;
mod event;
mod nav_interval;
mod render;
mod theme;
mod time_format;

pub use app::App;
pub use copy::build_copy_text;
pub use time_format::use_24h_for_format;
#[cfg(test)]
pub(crate) use render::format_offset;

/// Resolve the active navigation interval at TUI launch.
///
/// Resolution order (matches the `--date`/`--time` flag pattern):
/// 1. Explicit `--interval` flag value, if provided.
/// 2. Persisted `config.interval` (if it maps to a valid `NavInterval`).
/// 3. Default `NavInterval::H1` (60 minutes).
pub fn resolve_launch_interval(flag: Option<NavInterval>, config_minutes: u32) -> NavInterval {
    flag.or_else(|| NavInterval::from_minutes(config_minutes))
        .unwrap_or(NavInterval::H1)
}

const CELL_WIDTH: u16 = 3;
const INFO_COL_WIDTH: u16 = 40;
const TIMELINE_GAP: u16 = 2;
const BLOCK_HEIGHT: u16 = 3;
const DEBOUNCE_MS: u64 = 50;

const ACCEL_MAX_MS: u64 = 2000;
const ACCEL_MAX_STEP: i32 = 8;

/// Navigation interval: how many minutes a single timeline cell represents.
/// At `H1` (default) the view is byte-for-byte identical to the legacy hourly view.
/// At `M30` / `M15` intermediate cells are inserted between hour cells (1 / 3 respectively).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavInterval {
    H1,
    M30,
    M15,
}

impl NavInterval {
    pub const fn minutes(self) -> u32 {
        match self {
            NavInterval::H1 => 60,
            NavInterval::M30 => 30,
            NavInterval::M15 => 15,
        }
    }

    pub const fn next(self) -> Self {
        match self {
            NavInterval::H1 => NavInterval::M30,
            NavInterval::M30 => NavInterval::M15,
            NavInterval::M15 => NavInterval::H1,
        }
    }

    pub const fn from_minutes(m: u32) -> Option<Self> {
        match m {
            60 => Some(NavInterval::H1),
            30 => Some(NavInterval::M30),
            15 => Some(NavInterval::M15),
            _ => None,
        }
    }
}

#[cfg(test)]
mod nav_interval_tests {
    use super::NavInterval;

    #[test]
    fn minutes_match_variants() {
        assert_eq!(NavInterval::H1.minutes(), 60);
        assert_eq!(NavInterval::M30.minutes(), 30);
        assert_eq!(NavInterval::M15.minutes(), 15);
    }

    #[test]
    fn next_cycles_in_order() {
        assert_eq!(NavInterval::H1.next(), NavInterval::M30);
        assert_eq!(NavInterval::M30.next(), NavInterval::M15);
        assert_eq!(NavInterval::M15.next(), NavInterval::H1);
    }

    #[test]
    fn from_minutes_accepts_supported_values() {
        assert_eq!(NavInterval::from_minutes(60), Some(NavInterval::H1));
        assert_eq!(NavInterval::from_minutes(30), Some(NavInterval::M30));
        assert_eq!(NavInterval::from_minutes(15), Some(NavInterval::M15));
    }

    #[test]
    fn from_minutes_rejects_unsupported_values() {
        assert_eq!(NavInterval::from_minutes(0), None);
        assert_eq!(NavInterval::from_minutes(7), None);
        assert_eq!(NavInterval::from_minutes(45), None);
        assert_eq!(NavInterval::from_minutes(120), None);
    }
}
