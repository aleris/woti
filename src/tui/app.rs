use std::io;
use std::time::Instant;

use chrono::{DateTime, Utc};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::{AppConfig, TimeFormat};

use super::{BLOCK_HEIGHT, NavInterval};

pub struct App {
    pub(super) config: AppConfig,
    pub(super) anchor_time: Option<DateTime<Utc>>,
    /// Selected cell offset relative to the "now" cell. Each cell represents
    /// `interval.minutes()` minutes, so the selected absolute time offset in
    /// minutes is `cell_offset * interval.minutes()`.
    pub(super) cell_offset: i32,
    pub(super) scroll_offset: usize,
    pub(super) time_format: TimeFormat,
    pub(super) shading_enabled: bool,
    pub(super) interval: NavInterval,
    pub(super) should_quit: bool,
    pub(super) copied_at: Option<Instant>,
}

impl App {
    pub fn new(
        config: AppConfig,
        anchor_time: Option<DateTime<Utc>>,
        interval: NavInterval,
    ) -> Self {
        let time_format = config.time_format.unwrap_or(TimeFormat::Mixed);
        let shading_enabled = config.working_hours.enabled;
        Self {
            config,
            anchor_time,
            cell_offset: 0,
            scroll_offset: 0,
            time_format,
            shading_enabled,
            interval,
            should_quit: false,
            copied_at: None,
        }
    }

    pub(super) fn reference_time(&self) -> DateTime<Utc> {
        self.anchor_time.unwrap_or_else(Utc::now)
    }

    pub(super) fn max_scroll(&self, body_height: u16) -> usize {
        let visible = (body_height / BLOCK_HEIGHT) as usize;
        self.config.timezones.len().saturating_sub(visible)
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;

        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = io::stdout().execute(LeaveAlternateScreen);
            hook(info);
        }));

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.event_loop(&mut terminal);

        disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }
}

