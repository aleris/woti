use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use chrono::{Datelike, Local, Offset, Timelike, Utc};
use chrono_tz::Tz;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;

use crate::config::{AppConfig, TimeFormat};
use crate::tz_data;

const CELL_WIDTH: u16 = 3;
const INFO_COL_WIDTH: u16 = 40;
const TIMELINE_GAP: u16 = 2;
const BLOCK_HEIGHT: u16 = 3;
const DEBOUNCE_MS: u64 = 50;

pub struct App {
    config: AppConfig,
    hour_offset: i32,
    scroll_offset: usize,
    time_format: TimeFormat,
    system_use_24h: bool,
    should_quit: bool,
    copied_at: Option<Instant>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let system_use_24h = detect_use_24h();
        let time_format = config.time_format.unwrap_or_else(|| {
            if system_use_24h {
                TimeFormat::H24
            } else {
                TimeFormat::AmPm
            }
        });
        Self {
            config,
            hour_offset: 0,
            scroll_offset: 0,
            time_format,
            system_use_24h,
            should_quit: false,
            copied_at: None,
        }
    }

    fn max_scroll(&self, body_height: u16) -> usize {
        let visible = (body_height / BLOCK_HEIGHT) as usize;
        self.config.timezones.len().saturating_sub(visible)
    }

    fn use_24h_for_header(&self) -> bool {
        match self.time_format {
            TimeFormat::H24 => true,
            TimeFormat::AmPm => false,
            TimeFormat::Mixed => self.system_use_24h,
        }
    }

    fn use_24h_for_tz(&self, iana_id: &str) -> bool {
        match self.time_format {
            TimeFormat::H24 => true,
            TimeFormat::AmPm => false,
            TimeFormat::Mixed => !tz_data::uses_12h_clock(iana_id),
        }
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

    fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let mut last_render = Instant::now();
        let mut pending_h_offset: i32 = 0;

        terminal.draw(|f| self.render(f))?;

        loop {
            let timeout =
                Duration::from_millis(if pending_h_offset != 0 { DEBOUNCE_MS } else { 250 });

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('q'), _)
                            | (KeyCode::Char('x'), _)
                            | (KeyCode::Esc, _)
                            | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                self.should_quit = true;
                            }
                            (KeyCode::Left, _) => {
                                pending_h_offset -= 1;
                            }
                            (KeyCode::Right, _) => {
                                pending_h_offset += 1;
                            }
                            (KeyCode::Up, _) => {
                                if self.scroll_offset > 0 {
                                    self.scroll_offset -= 1;
                                    terminal.draw(|f| self.render(f))?;
                                    last_render = Instant::now();
                                }
                            }
                            (KeyCode::Down, _) => {
                                let body_h = terminal.size()?.height.saturating_sub(2);
                                let max = self.max_scroll(body_h);
                                if self.scroll_offset < max {
                                    self.scroll_offset += 1;
                                    terminal.draw(|f| self.render(f))?;
                                    last_render = Instant::now();
                                }
                            }
                            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                                self.copy_selection();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                                self.cycle_time_format();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            _ => {}
                        }
                    }
                    Event::Resize(_, _) => {
                        let body_h = terminal.size()?.height.saturating_sub(2);
                        let max = self.max_scroll(body_h);
                        if self.scroll_offset > max {
                            self.scroll_offset = max;
                        }
                        terminal.draw(|f| self.render(f))?;
                        last_render = Instant::now();
                    }
                    _ => {}
                }

                if self.should_quit {
                    return Ok(());
                }

                if pending_h_offset != 0
                    && last_render.elapsed() >= Duration::from_millis(DEBOUNCE_MS)
                {
                    self.hour_offset += pending_h_offset;
                    pending_h_offset = 0;
                    terminal.draw(|f| self.render(f))?;
                    last_render = Instant::now();
                }
            } else {
                if pending_h_offset != 0 {
                    self.hour_offset += pending_h_offset;
                    pending_h_offset = 0;
                }
                terminal.draw(|f| self.render(f))?;
                last_render = Instant::now();
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let layout = Layout::vertical([
            Constraint::Length(1), // header — always visible
            Constraint::Min(0),   // body — scrollable
            Constraint::Length(1), // footer — always visible
        ])
        .split(area);

        self.render_header(frame, layout[0]);
        self.render_body(frame, layout[1]);
        self.render_footer(frame, layout[2], layout[1].height);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let now = Local::now();
        let time_str = if self.use_24h_for_header() {
            now.format("%H:%M:%S").to_string()
        } else {
            now.format("%-I:%M:%S %p").to_string()
        };
        let date_str = now.format("%a, %b %d %Y").to_string();
        let tz_abbr = now.format("%Z").to_string();

        let right = format!("{date_str}  {time_str} {tz_abbr}");

        let title = Span::styled(" woti", Style::default().fg(Color::Cyan).bold());
        let spacer_width = area.width.saturating_sub(5 + right.len() as u16 + 1);
        let spacer = Span::raw(" ".repeat(spacer_width as usize));
        let right_span = Span::styled(
            format!("{right} "),
            Style::default().fg(Color::White).dim(),
        );

        let line = Line::from(vec![title, spacer, right_span]);
        let p = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(p, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, body_height: u16) {
        if let Some(t) = self.copied_at {
            if t.elapsed() < Duration::from_secs(2) {
                let line = Line::from(vec![
                    Span::styled(" Copied! ", Style::default().fg(Color::Green).bold()),
                ]);
                let p = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));
                frame.render_widget(p, area);
                return;
            }
        }

        let max = self.max_scroll(body_height);
        let can_up = self.scroll_offset > 0;
        let can_down = self.scroll_offset < max;

        let key_on = Style::default().fg(Color::Black).bg(Color::Gray);
        let key_off = Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Rgb(50, 50, 50));
        let label_on = Style::default().fg(Color::Gray);
        let label_off = Style::default().fg(Color::DarkGray);

        let shortcuts = vec![
            Span::styled(" ↑ ", if can_up { key_on } else { key_off }),
            Span::styled(" Up ", if can_up { label_on } else { label_off }),
            Span::styled(" ↓ ", if can_down { key_on } else { key_off }),
            Span::styled(" Down ", if can_down { label_on } else { label_off }),
            Span::styled(" ← ", key_on),
            Span::styled(" Prev Hour ", label_on),
            Span::styled(" → ", key_on),
            Span::styled(" Next Hour ", label_on),
            Span::styled(" c ", key_on),
            Span::styled(" Copy ", label_on),
            Span::styled(" q ", key_on),
            Span::styled(" Quit ", label_on),
        ];

        let sel = Style::default()
            .fg(Color::Cyan)
            .bg(Color::Rgb(60, 60, 60))
            .bold();
        let dim = Style::default().fg(Color::DarkGray);
        let sep = Style::default().fg(Color::Rgb(80, 80, 80));

        let fmt_switcher: Vec<Span> = vec![
            Span::styled(" f ", key_on),
            Span::raw(" "),
            Span::styled(
                " 24 ",
                if self.time_format == TimeFormat::H24 { sel } else { dim },
            ),
            Span::styled("│", sep),
            Span::styled(
                " am ",
                if self.time_format == TimeFormat::AmPm { sel } else { dim },
            ),
            Span::styled("│", sep),
            Span::styled(
                " mx ",
                if self.time_format == TimeFormat::Mixed { sel } else { dim },
            ),
            Span::raw(" "),
        ];

        let left_w: usize = shortcuts.iter().map(|s| s.width()).sum();
        let right_w: usize = fmt_switcher.iter().map(|s| s.width()).sum();
        let spacer = (area.width as usize).saturating_sub(left_w + right_w);

        let mut spans = shortcuts;
        spans.push(Span::raw(" ".repeat(spacer)));
        spans.extend(fmt_switcher);

        let line = Line::from(spans);
        let p = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(p, area);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect) {
        let total = self.config.timezones.len();
        if total == 0 {
            return;
        }

        let mut y = area.y;
        let y_end = area.y + area.height;

        for i in self.scroll_offset..total {
            if y + BLOCK_HEIGHT > y_end {
                break;
            }
            let rect = Rect::new(area.x, y, area.width, BLOCK_HEIGHT);
            self.render_timezone_block(frame, rect, &self.config.timezones[i]);
            y += BLOCK_HEIGHT;
        }
    }

    fn cycle_time_format(&mut self) {
        self.time_format = match self.time_format {
            TimeFormat::H24 => TimeFormat::AmPm,
            TimeFormat::AmPm => TimeFormat::Mixed,
            TimeFormat::Mixed => TimeFormat::H24,
        };
        self.config.time_format = Some(self.time_format);
        let _ = self.config.save();
    }

    fn copy_selection(&mut self) {
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

            lines.push(format!("{} / {} {}{}", entry.city, tz_abbr, time_str, day_suffix));
        }

        lines.join("\n")
    }

    fn render_timezone_block(
        &self,
        frame: &mut Frame,
        area: Rect,
        entry: &crate::config::TimezoneEntry,
    ) {
        let tz: Tz = entry.iana_id.parse().unwrap_or(chrono_tz::UTC);

        let now_utc = Utc::now();
        let now_tz = now_utc.with_timezone(&tz);
        let current_hour = now_tz.hour() as i32;

        let use_24h = self.use_24h_for_tz(&entry.iana_id);
        let tz_abbr = now_tz.format("%Z").to_string();
        let time_str = if use_24h {
            now_tz.format("%H:%M").to_string()
        } else {
            now_tz.format("%-I:%M %p").to_string()
        };
        let date_str = now_tz.format("%a, %b %d").to_string();

        let block = Block::default().padding(Padding::new(1, 1, 0, 0));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

        let info_w = (INFO_COL_WIDTH.min(inner.width)) as usize;
        let left_pad = info_w + TIMELINE_GAP as usize;
        let timeline_avail = inner.width.saturating_sub(left_pad as u16);
        let num_cells = (timeline_avail / CELL_WIDTH) as i32;

        let base_hour = current_hour + self.hour_offset;
        let start_hour = base_hour - num_cells / 2;

        let cell_w = CELL_WIDTH as usize;
        let total_day_chars = (num_cells as usize) * cell_w;
        let mut day_chars: Vec<char> = vec![' '; total_day_chars];
        let mut day_is_label: Vec<bool> = vec![false; total_day_chars];

        let mut hour_spans: Vec<Span> = Vec::new();
        let mut ampm_spans: Vec<Span> = Vec::new();

        for i in 0..num_cells {
            let h = start_hour + i;
            let hour_in_day = ((h % 24) + 24) % 24;

            let dt = compute_datetime_for_hour(tz, now_tz, h - current_hour);
            let is_selected = h == base_hour;
            let is_local = h == current_hour && self.hour_offset != 0;

            let cell_style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .bold()
            } else if is_local {
                Style::default().bg(Color::Rgb(50, 50, 50))
            } else {
                Style::default()
            };

            if hour_in_day == 0 {
                let day_label = format!(
                    "{} {}",
                    dt.format("%a").to_string().to_uppercase(),
                    dt.format("%-d")
                );
                let start_pos = (i as usize) * cell_w;
                for (j, ch) in day_label.chars().enumerate() {
                    let pos = start_pos + j;
                    if pos < total_day_chars {
                        day_chars[pos] = ch;
                        day_is_label[pos] = true;
                    }
                }
            }

            let h_str = if use_24h {
                format!("{:>width$}", hour_in_day, width = cell_w)
            } else {
                let h12 = hour_in_day % 12;
                let h12 = if h12 == 0 { 12 } else { h12 };
                format!("{:>width$}", h12, width = cell_w)
            };
            hour_spans.push(Span::styled(
                h_str,
                if is_selected {
                    cell_style
                } else if is_local {
                    cell_style.fg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                },
            ));

            let (row2_text, row2_style) = if !use_24h {
                let ampm = if hour_in_day < 12 { "am" } else { "pm" };
                (
                    format!("{:>width$}", ampm, width = cell_w),
                    if is_selected {
                        cell_style
                    } else if is_local {
                        cell_style.fg(Color::DarkGray)
                    } else {
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM)
                    },
                )
            } else {
                (" ".repeat(cell_w), cell_style)
            };
            ampm_spans.push(Span::styled(row2_text, row2_style));
        }

        let day_spans: Vec<Span> = {
            let style_for = |pos: usize| -> Style {
                let cell_idx = pos / cell_w;
                let h = start_hour + cell_idx as i32;
                let is_sel = h == base_hour;
                let is_loc = h == current_hour && self.hour_offset != 0;
                let is_lab = day_is_label[pos];
                if is_sel {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .bold()
                } else if is_loc {
                    if is_lab {
                        Style::default()
                            .bg(Color::Rgb(50, 50, 50))
                            .fg(Color::Magenta)
                            .bold()
                    } else {
                        Style::default().bg(Color::Rgb(50, 50, 50))
                    }
                } else if is_lab {
                    Style::default().fg(Color::Magenta).bold()
                } else {
                    Style::default()
                }
            };

            let mut spans = Vec::new();
            let mut pos = 0;
            while pos < total_day_chars {
                let style = style_for(pos);
                let mut end = pos + 1;
                while end < total_day_chars && style_for(end) == style {
                    end += 1;
                }
                let text: String = day_chars[pos..end].iter().collect();
                spans.push(Span::styled(text, style));
                pos = end;
            }
            spans
        };

        // Row 1: blank info area + day markers
        let mut line1 = vec![Span::raw(" ".repeat(left_pad))];
        line1.extend(day_spans);
        frame.render_widget(Paragraph::new(Line::from(line1)), rows[0]);

        // Row 2: city  [TZ]  +offset   time + hours
        let utc_secs = now_tz.offset().fix().local_minus_utc();
        let offset_h = utc_secs / 3600;
        let offset_m = (utc_secs.abs() % 3600) / 60;
        let offset_str = if offset_m != 0 {
            format!("{:+}:{:02}", offset_h, offset_m)
        } else {
            format!("{:+}", offset_h)
        };
        let tz_badge = format!(" {} ", tz_abbr);
        let parts_len = entry.city.len() + 1 + tz_badge.len() + 1 + offset_str.len() + time_str.len();
        let info_gap = info_w.saturating_sub(parts_len);
        let mut line2 = vec![
            Span::styled(&entry.city, Style::default().fg(Color::White).bold()),
            Span::raw(" "),
            Span::styled(tz_badge, Style::default().fg(Color::White).bg(Color::DarkGray)),
            Span::raw(" "),
            Span::styled(offset_str, Style::default().fg(Color::Cyan)),
            Span::raw(" ".repeat(info_gap)),
            Span::styled(time_str, Style::default().fg(Color::Green).bold()),
            Span::raw(" ".repeat(TIMELINE_GAP as usize)),
        ];
        line2.extend(hour_spans);
        frame.render_widget(Paragraph::new(Line::from(line2)), rows[1]);

        // Row 3: region + date (+ am/pm in 12h mode)
        let region_len = entry.region.len() + date_str.len();
        let info_gap3 = info_w.saturating_sub(region_len);
        let mut line3 = vec![
            Span::styled(&entry.region, Style::default().fg(Color::DarkGray)),
            Span::raw(" ".repeat(info_gap3)),
            Span::styled(date_str, Style::default().fg(Color::DarkGray)),
            Span::raw(" ".repeat(TIMELINE_GAP as usize)),
        ];
        line3.extend(ampm_spans);
        frame.render_widget(Paragraph::new(Line::from(line3)), rows[2]);
    }
}

fn detect_use_24h() -> bool {
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

fn compute_datetime_for_hour(
    _tz: Tz,
    now_tz: chrono::DateTime<Tz>,
    offset_from_current: i32,
) -> chrono::DateTime<Tz> {
    let duration = chrono::Duration::hours(offset_from_current as i64);
    now_tz + duration
}
