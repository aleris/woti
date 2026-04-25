use std::time::Duration;

use chrono::{Datelike, Offset, Timelike};
use chrono_tz::Tz;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

use crate::config::{TimeFormat, WorkingHoursConfig};

use super::app::App;
use super::theme;
use super::{BLOCK_HEIGHT, CELL_WIDTH, INFO_COL_WIDTH, NavInterval, TIMELINE_GAP};

/// Compute the datetime for a cell whose offset from "now" is given in minutes.
/// Used by the sub-hour interval rendering pipeline (and equivalent to the
/// legacy hour-based helper when `offset_minutes` is a multiple of 60).
pub fn compute_datetime_for_minutes(
    now_tz: chrono::DateTime<Tz>,
    offset_minutes: i32,
) -> chrono::DateTime<Tz> {
    now_tz + chrono::Duration::minutes(offset_minutes as i64)
}

/// Floor `now_tz` to the most recent boundary of `interval_minutes`. For the
/// 60-minute interval we intentionally return `now_tz` unchanged so the H1
/// rendering path stays byte-for-byte identical to the legacy behavior
/// (which preserved `now_tz`'s wall-clock minutes inside each hour cell).
fn floor_to_interval(now_tz: chrono::DateTime<Tz>, interval_minutes: i32) -> chrono::DateTime<Tz> {
    if interval_minutes >= 60 {
        return now_tz;
    }
    let minute = now_tz.minute() as i32;
    let floored_minute = (minute / interval_minutes) * interval_minutes;
    let drop_minutes = minute - floored_minute;
    now_tz
        - chrono::Duration::minutes(drop_minutes as i64)
        - chrono::Duration::seconds(now_tz.second() as i64)
        - chrono::Duration::nanoseconds(now_tz.nanosecond() as i64)
}

/// `true` when the cell at `dt` is an "hour cell" (rendered with the wall-hour
/// digit + the timezone's existing sub-row glyph). Intermediate cells in
/// sub-hour intervals fail this check and render `·` + a wall-clock minute
/// superscript instead.
///
/// In sub-hour intervals the cell stride may not divide a timezone's
/// `offset_m` evenly (e.g. Nepal `+5:45` at M30: cells land on `:00`/`:30`
/// but the natural hour change is at `:45`). To keep one hour digit per
/// hour aligned visually with other timezones, we pick the cell whose
/// `[start, start + interval)` window *contains* the natural hour boundary,
/// i.e. `dt.minute() == floor(offset_m / interval_minutes) * interval_minutes`.
/// At H1 callers short-circuit; this helper is only meaningful for
/// `interval_minutes < 60`.
fn is_hour_cell(dt: chrono::DateTime<Tz>, offset_m: i32, interval_minutes: i32) -> bool {
    let cell_minute = dt.minute() as i32;
    let target = (offset_m / interval_minutes) * interval_minutes;
    cell_minute == target
}

impl App {
    pub(super) fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

        self.render_header(frame, layout[0]);
        self.render_body(frame, layout[1]);
        self.render_footer(frame, layout[2], layout[1].height);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let icon = Span::styled(" 🕜 ", Style::default().fg(theme::HEADER_ICON));
        let title = Span::styled("woti", Style::default().fg(theme::HEADER_TITLE).bold());

        let line = Line::from(vec![icon, title]);
        let p = Paragraph::new(line).style(Style::default().bg(theme::HEADER_BG));
        frame.render_widget(p, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, body_height: u16) {
        let footer_bg = Style::default().bg(theme::FOOTER_BG);

        if let Some(t) = self.copied_at {
            if t.elapsed() < Duration::from_secs(2) {
                let line = Line::from(vec![
                    Span::styled(" Copied! ", Style::default().fg(theme::COPIED).bold()),
                ]);
                frame.render_widget(Paragraph::new(line).style(footer_bg), area);
                return;
            }
        }

        let max = self.max_scroll(body_height);
        let can_up = self.scroll_offset > 0;
        let can_down = self.scroll_offset < max;

        let key_on = Style::default().fg(theme::KEY_FG).bg(theme::KEY_BG);
        let key_off = Style::default()
            .fg(theme::KEY_DISABLED_FG)
            .bg(theme::KEY_DISABLED_BG);
        let label_on = Style::default().fg(theme::LABEL_FG);
        let label_off = Style::default().fg(theme::LABEL_DISABLED_FG);

        let nav_label = match self.interval {
            NavInterval::H1 => " Hour ",
            NavInterval::M30 => " 30m ",
            NavInterval::M15 => " 15m ",
        };
        let shortcuts = vec![
            Span::styled(" ↑ ", if can_up { key_on } else { key_off }),
            Span::styled(" Up ", if can_up { label_on } else { label_off }),
            Span::styled(" ↓ ", if can_down { key_on } else { key_off }),
            Span::styled(" Down ", if can_down { label_on } else { label_off }),
            Span::styled(" ← ", key_on),
            Span::styled(format!(" Prev{nav_label}"), label_on),
            Span::styled(" → ", key_on),
            Span::styled(format!(" Next{nav_label}"), label_on),
            Span::styled(" c ", key_on),
            Span::styled(" Copy ", label_on),
            Span::styled(" q ", key_on),
            Span::styled(" Quit ", label_on),
        ];

        let sel = Style::default()
            .fg(theme::SWITCHER_ACTIVE_FG)
            .bg(theme::SWITCHER_ACTIVE_BG)
            .bold();
        let dim = Style::default().fg(theme::SWITCHER_DIM_FG);
        let sep = Style::default().fg(theme::SWITCHER_SEP);

        let shade_label_style = if self.shading_enabled {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(theme::HOUR_FG_TRANSITION)
        };

        let interval_switcher: Vec<Span> = vec![
            Span::styled(" i ", key_on),
            Span::raw(" "),
            Span::styled(
                " 60 ",
                if self.interval == NavInterval::H1 { sel } else { dim },
            ),
            Span::styled("│", sep),
            Span::styled(
                " 30 ",
                if self.interval == NavInterval::M30 { sel } else { dim },
            ),
            Span::styled("│", sep),
            Span::styled(
                " 15 ",
                if self.interval == NavInterval::M15 { sel } else { dim },
            ),
            Span::raw(" "),
        ];

        let fmt_switcher: Vec<Span> = vec![
            Span::styled(" w ", key_on),
            Span::styled(" Shade ", shade_label_style),
            Span::styled(" f ", key_on),
            Span::raw(" "),
            Span::styled(
                " mx ",
                if self.time_format == TimeFormat::Mixed {
                    sel
                } else {
                    dim
                },
            ),
            Span::styled("│", sep),
            Span::styled(
                " am ",
                if self.time_format == TimeFormat::AmPm {
                    sel
                } else {
                    dim
                },
            ),
            Span::styled("│", sep),
            Span::styled(
                " 24 ",
                if self.time_format == TimeFormat::H24 {
                    sel
                } else {
                    dim
                },
            ),
            Span::raw(" "),
        ];

        let left_w: usize = shortcuts.iter().map(|s| s.width()).sum();
        let right_w: usize = interval_switcher.iter().map(|s| s.width()).sum::<usize>()
            + fmt_switcher.iter().map(|s| s.width()).sum::<usize>();
        let spacer = (area.width as usize).saturating_sub(left_w + right_w);

        let mut spans = shortcuts;
        spans.push(Span::raw(" ".repeat(spacer)));
        spans.extend(interval_switcher);
        spans.extend(fmt_switcher);

        let line = Line::from(spans);
        frame.render_widget(Paragraph::new(line).style(footer_bg), area);
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

        if self.cell_offset != 0 && y < y_end {
            let selected_minutes = self.cell_offset * self.interval.minutes() as i32;
            let label = format_offset(selected_minutes);
            let inner_width = area.width.saturating_sub(2);
            let info_w = (INFO_COL_WIDTH.min(inner_width)) as usize;
            let left_pad = info_w + TIMELINE_GAP as usize;
            let timeline_avail = inner_width.saturating_sub(left_pad as u16);
            let num_cells = (timeline_avail / CELL_WIDTH) as i32;
            let selected_cell = num_cells / 2;
            let x_offset =
                1 + left_pad + (selected_cell as usize) * (CELL_WIDTH as usize) + 1;

            let line = Line::from(vec![
                Span::raw(" ".repeat(x_offset)),
                Span::styled(label, selected_style()),
            ]);
            let rect = Rect::new(area.x, y, area.width, 1);
            frame.render_widget(Paragraph::new(line), rect);
        }
    }

    fn render_timezone_block(
        &self,
        frame: &mut Frame,
        area: Rect,
        entry: &crate::config::TimezoneEntry,
    ) {
        let tz: Tz = entry.iana_id.parse().unwrap_or(chrono_tz::UTC);

        let now_utc = self.reference_time();
        let now_tz = now_utc.with_timezone(&tz);
        let interval_minutes = self.interval.minutes() as i32;
        let anchor_dt = floor_to_interval(now_tz, interval_minutes);
        let selected_offset_minutes = self.cell_offset * interval_minutes;
        // Cell-aligned anchor for the timeline (snaps to interval grid so
        // hour digits land on the right column and `is_hour_cell` works).
        let cell_anchor_dt = compute_datetime_for_minutes(anchor_dt, selected_offset_minutes);
        // Left-info datetime preserves the actual wall-clock minute (e.g.
        // `:37` instead of the floored `:30`/`:15`). Mirrors the H1 path
        // where `floor_to_interval` is a no-op and the displayed time is
        // simply `now_tz + cell_offset * 60` — i.e. the user's original
        // minute is carried through navigation.
        let display_dt = compute_datetime_for_minutes(now_tz, selected_offset_minutes);

        let use_24h = self.use_24h_for_tz(&entry.iana_id);
        let tz_abbr = crate::tz_data::display_abbreviation(&display_dt);
        let time_str = if use_24h {
            display_dt.format("%H:%M").to_string()
        } else {
            display_dt.format("%-I:%M %p").to_string()
        };
        let date_str = display_dt.format("%a, %b %d").to_string();

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
        let cell_w = CELL_WIDTH as usize;

        let selected_idx = num_cells / 2;
        let current_idx = selected_idx - self.cell_offset;

        let utc_secs = cell_anchor_dt.offset().fix().local_minus_utc();
        let offset_m = (utc_secs.abs() % 3600) / 60;

        let shading = if self.shading_enabled {
            Some(self.config.working_hours)
        } else {
            None
        };

        let tl = TimelineParams {
            base_dt: cell_anchor_dt,
            cell_offset: self.cell_offset,
            num_cells,
            selected_idx,
            current_idx,
            interval_minutes,
            cell_w,
            use_24h,
            offset_m,
            now_tz,
            shading,
        };

        let hour_spans = build_hour_spans(&tl);
        let ampm_spans = build_ampm_spans(&tl);
        let day_spans = build_day_spans(&tl);

        let mut line1 = vec![Span::raw(" ".repeat(left_pad))];
        line1.extend(day_spans);
        frame.render_widget(Paragraph::new(Line::from(line1)), rows[0]);

        let mut line2 = build_info_line(entry, &tz_abbr, &time_str, display_dt, info_w);
        line2.extend(hour_spans);
        frame.render_widget(Paragraph::new(Line::from(line2)), rows[1]);

        let region_len = entry.region.len() + date_str.len();
        let info_gap3 = info_w.saturating_sub(region_len);
        let mut line3 = vec![
            Span::styled(&entry.region, Style::default().fg(theme::SECONDARY_FG)),
            Span::raw(" ".repeat(info_gap3)),
            Span::styled(date_str, Style::default().fg(theme::SECONDARY_FG)),
            Span::raw(" ".repeat(TIMELINE_GAP as usize)),
        ];
        line3.extend(ampm_spans);
        frame.render_widget(Paragraph::new(Line::from(line3)), rows[2]);
    }
}

struct TimelineParams {
    /// Absolute datetime of the selected (center) cell.
    base_dt: chrono::DateTime<Tz>,
    /// Selected cell offset (in cells, signed) relative to the "now" cell.
    cell_offset: i32,
    num_cells: i32,
    /// Index of the selected cell in the visible row (always `num_cells / 2`).
    selected_idx: i32,
    /// Index of the cell holding the floored "now" boundary
    /// (`selected_idx - cell_offset`).
    current_idx: i32,
    /// Minutes per cell. `60` reproduces the legacy hourly view byte-for-byte.
    interval_minutes: i32,
    cell_w: usize,
    use_24h: bool,
    /// Timezone's sub-hour minute offset from UTC (e.g. `30` for IST, `45` for
    /// Nepal). Used to pick which cells are "hour cells" in sub-hour modes.
    offset_m: i32,
    now_tz: chrono::DateTime<Tz>,
    shading: Option<WorkingHoursConfig>,
}

impl TimelineParams {
    /// Datetime for the cell at visible index `i`.
    fn dt_for_cell(&self, i: i32) -> chrono::DateTime<Tz> {
        let delta = (i - self.selected_idx) as i64 * self.interval_minutes as i64;
        self.base_dt + chrono::Duration::minutes(delta)
    }

    /// True at H1 (every cell is conceptually an hour cell) or when the cell
    /// is the sub-hour cell whose window contains the timezone's natural
    /// hour boundary.
    fn is_hour_cell_at(&self, i: i32) -> bool {
        if self.interval_minutes >= 60 {
            return true;
        }
        is_hour_cell(self.dt_for_cell(i), self.offset_m, self.interval_minutes)
    }
}

fn selected_style() -> Style {
    Style::default()
        .fg(theme::SELECTED_FG)
        .bg(theme::SELECTED_BG)
        .bold()
}

fn local_style() -> Style {
    Style::default().bg(theme::LOCAL_BG)
}

fn hour_fg_color(hour_in_day: i32, wh: &WorkingHoursConfig) -> Color {
    let h = hour_in_day as u8;
    if h >= wh.work_start && h < wh.work_end {
        theme::HOUR_FG
    } else if h >= wh.transition_start && h < wh.transition_end {
        theme::HOUR_FG_TRANSITION
    } else {
        theme::HOUR_FG_NIGHT
    }
}

fn ampm_fg_color(hour_in_day: i32, wh: &WorkingHoursConfig) -> Color {
    let h = hour_in_day as u8;
    if h >= wh.work_start && h < wh.work_end {
        theme::AMPM_FG
    } else if h >= wh.transition_start && h < wh.transition_end {
        theme::AMPM_FG_TRANSITION
    } else {
        theme::AMPM_FG_NIGHT
    }
}

fn build_hour_spans(p: &TimelineParams) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    for i in 0..p.num_cells {
        let dt = p.dt_for_cell(i);
        let hour_in_day = dt.hour() as i32;
        let is_selected = i == p.selected_idx;
        let is_local = i == p.current_idx && p.cell_offset != 0;
        let is_hour = p.is_hour_cell_at(i);

        // Shading is computed against the wall hour, even on intermediate
        // cells, so that 14:15 / 14:30 / 14:45 inherit the same band as 14:00.
        let fg = match &p.shading {
            Some(wh) => hour_fg_color(hour_in_day, wh),
            None => theme::HOUR_FG,
        };

        let style = if is_selected {
            selected_style()
        } else if is_local {
            local_style().fg(theme::HOUR_FG)
        } else {
            Style::default().fg(fg)
        };

        let text = if is_hour {
            if p.use_24h {
                format!("{:>2}", hour_in_day)
            } else {
                let h12 = hour_in_day % 12;
                let h12 = if h12 == 0 { 12 } else { h12 };
                format!("{:>2}", h12)
            }
        } else {
            // Intermediate cell in a sub-hour interval: right-aligned tick.
            " ·".to_string()
        };

        spans.push(Span::raw(" "));
        spans.push(Span::styled(text, style));
    }
    spans
}

/// Superscript glyph for a wall-clock minute on intermediate cells, or for a
/// timezone's sub-hour offset on hour cells. Returns two spaces for any value
/// outside `{0, 15, 30, 45}` so width stays stable.
///
/// The `0 → "⁰⁰"` case only ever fires on intermediate cells of fractional
/// zones (Nepal `+5:45`, India `+5:30`, etc.) where the wall-clock `:00`
/// boundary lands on a tick row, not on an hour digit. For whole-hour zones
/// the `:00` cell is the hour cell itself (handled separately and kept blank
/// in 24h mode / `am`/`pm` in 12h), so `⁰⁰` never appears in their sub-row.
fn minutes_superscript_full(m: i32) -> &'static str {
    match m {
        0 => "⁰⁰",
        15 => "¹⁵",
        30 => "³⁰",
        45 => "⁴⁵",
        _ => "  ",
    }
}

fn minutes_fraction(m: i32) -> &'static str {
    match m {
        30 => "½",
        45 => "¾",
        _ => "",
    }
}

fn build_ampm_spans(p: &TimelineParams) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    for i in 0..p.num_cells {
        let dt = p.dt_for_cell(i);
        let hour_in_day = dt.hour() as i32;
        let is_selected = i == p.selected_idx;
        let is_local = i == p.current_idx && p.cell_offset != 0;
        let is_hour = p.is_hour_cell_at(i);

        let (text, style) = if !p.use_24h {
            let fg = match &p.shading {
                Some(wh) => ampm_fg_color(hour_in_day, wh),
                None => theme::AMPM_FG,
            };

            let style = if is_selected {
                selected_style()
            } else if is_local {
                local_style().fg(theme::AMPM_FG)
            } else {
                Style::default()
                    .fg(fg)
                    .add_modifier(Modifier::DIM)
            };

            let text = if is_hour {
                // Hour cell: existing am/pm or ½a/¾p glyph stays unchanged.
                if p.offset_m != 0 {
                    let frac = minutes_fraction(p.offset_m);
                    let meridiem = if hour_in_day < 12 { "a" } else { "p" };
                    format!("{frac}{meridiem}")
                } else {
                    let ampm = if hour_in_day < 12 { "am" } else { "pm" };
                    ampm.to_string()
                }
            } else {
                // Intermediate cell (sub-hour interval): wall-clock minute
                // superscript (¹⁵ / ³⁰ / ⁴⁵).
                minutes_superscript_full(dt.minute() as i32).to_string()
            };
            (text, style)
        } else {
            let style = if is_selected {
                selected_style()
            } else if is_local {
                local_style()
            } else {
                Style::default()
            };

            let text = if is_hour {
                if p.offset_m != 0 {
                    // Half/quarter-hour zone hour cell: keep ³⁰ / ⁴⁵ glyph.
                    minutes_superscript_full(p.offset_m).to_string()
                } else {
                    // 24h whole-hour zone hour cell: stays blank in all
                    // intervals (better visual contrast — the hour digit
                    // on the row above already marks the slot).
                    "  ".to_string()
                }
            } else {
                minutes_superscript_full(dt.minute() as i32).to_string()
            };
            (text, style)
        };

        spans.push(Span::raw(" "));
        spans.push(Span::styled(text, style));
    }
    spans
}

fn build_day_spans(p: &TimelineParams) -> Vec<Span<'static>> {
    let total_day_chars = (p.num_cells as usize) * p.cell_w;
    let mut day_chars: Vec<char> = vec![' '; total_day_chars];
    let mut day_is_label: Vec<bool> = vec![false; total_day_chars];
    let mut day_label_origin: Vec<Option<i32>> = vec![None; total_day_chars];

    for i in 0..p.num_cells {
        let dt = p.dt_for_cell(i);

        // Only place the day label on hour cells: in sub-hour intervals we
        // must not light it up at 00:15 / 00:30 / 00:45.
        if dt.hour() == 0 && p.is_hour_cell_at(i) {
            let today = p.now_tz.date_naive();
            let label_date = dt.date_naive();
            let mut day_label = format!(
                "{} {}",
                dt.format("%a").to_string().to_uppercase(),
                dt.format("%-d")
            );
            if label_date.month() != today.month() || label_date.year() != today.year() {
                day_label.push_str(&format!(", {}", dt.format("%B")));
                if label_date.year() != today.year() {
                    day_label.push_str(&format!(", {}", dt.format("%Y")));
                }
            }
            let midnight_display = if p.use_24h { 0 } else { 12_i32 };
            let digit_offset = if midnight_display >= 10 { 1 } else { 2 };
            let start_pos = (i as usize) * p.cell_w + digit_offset;
            for (j, ch) in day_label.chars().enumerate() {
                let pos = start_pos + j;
                if pos < total_day_chars {
                    day_chars[pos] = ch;
                    day_is_label[pos] = true;
                    day_label_origin[pos] = Some(i);
                }
            }
        }
    }

    let style_for = |pos: usize| -> Style {
        let cell_idx = (pos / p.cell_w) as i32;
        let pos_in_cell = pos % p.cell_w;
        let is_sel = cell_idx == p.selected_idx;
        let is_loc = cell_idx == p.current_idx && p.cell_offset != 0;
        let is_lab = day_is_label[pos];
        let has_bg = pos_in_cell > 0;
        let origin = day_label_origin[pos];
        let label_sel = is_lab && origin == Some(p.selected_idx);
        let label_loc = is_lab && origin == Some(p.current_idx) && p.cell_offset != 0;
        if label_sel || (is_sel && has_bg) {
            Style::default()
                .fg(theme::SELECTED_FG)
                .bg(theme::SELECTED_BG)
                .bold()
        } else if label_loc || (is_loc && has_bg) {
            if is_lab {
                Style::default()
                    .bg(theme::LOCAL_BG)
                    .fg(theme::DAY_LABEL)
                    .bold()
            } else {
                Style::default().bg(theme::LOCAL_BG)
            }
        } else if is_lab {
            Style::default().fg(theme::DAY_LABEL).bold()
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
}

fn build_info_line<'a>(
    entry: &'a crate::config::TimezoneEntry,
    tz_abbr: &str,
    time_str: &str,
    now_tz: chrono::DateTime<Tz>,
    info_w: usize,
) -> Vec<Span<'a>> {
    let utc_secs = now_tz.offset().fix().local_minus_utc();
    let offset_h = utc_secs / 3600;
    let offset_m = (utc_secs.abs() % 3600) / 60;
    let offset_str = if offset_m != 0 {
        format!("{:+}:{:02}", offset_h, offset_m)
    } else {
        format!("{:+}", offset_h)
    };
    let tz_badge = format!(" {} ", tz_abbr);
    let parts_len =
        entry.city.len() + 1 + tz_badge.len() + 1 + offset_str.len() + time_str.len();
    let info_gap = info_w.saturating_sub(parts_len);
    vec![
        Span::styled(&entry.city, Style::default().fg(theme::CITY_FG).bold()),
        Span::raw(" "),
        Span::styled(
            tz_badge,
            Style::default()
                .fg(theme::TZ_BADGE_FG)
                .bg(theme::TZ_BADGE_BG),
        ),
        Span::raw(" "),
        Span::styled(offset_str, Style::default().fg(theme::OFFSET_FG)),
        Span::raw(" ".repeat(info_gap)),
        Span::styled(
            time_str.to_string(),
            Style::default().fg(theme::TIME_FG).bold(),
        ),
        Span::raw(" ".repeat(TIMELINE_GAP as usize)),
    ]
}

/// Render the relative-offset label shown above the selected column.
///
/// Returns "" for zero, "in <X>" for positive offsets, "<X> ago" for negative
/// offsets, where `<X>` may combine hours and minutes (e.g. `"1 hour 15
/// minutes"`). For hour-only or minute-only offsets only the relevant unit is
/// shown. Singular vs plural is respected for both units.
pub(crate) fn format_offset(minutes: i32) -> String {
    if minutes == 0 {
        return String::new();
    }
    let sign_positive = minutes > 0;
    let abs = minutes.abs();
    let h = abs / 60;
    let m = abs % 60;

    fn unit(n: i32, singular: &str, plural: &str) -> String {
        if n == 1 {
            format!("1 {singular}")
        } else {
            format!("{n} {plural}")
        }
    }

    let body = match (h, m) {
        (0, m) => unit(m, "minute", "minutes"),
        (h, 0) => unit(h, "hour", "hours"),
        (h, m) => format!(
            "{} {}",
            unit(h, "hour", "hours"),
            unit(m, "minute", "minutes")
        ),
    };

    if sign_positive {
        format!("in {body}")
    } else {
        format!("{body} ago")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Build a `TimelineParams` for cell-based tests.
    /// `selected_idx_in_window` says where the selected cell sits in the visible row;
    /// `current_idx_in_window` is computed as `selected_idx - cell_offset`.
    fn params_h1(
        now_tz: chrono::DateTime<Tz>,
        selected_dt: chrono::DateTime<Tz>,
        cell_offset: i32,
        num_cells: i32,
        use_24h: bool,
        offset_m: i32,
    ) -> TimelineParams {
        let selected_idx = num_cells / 2;
        TimelineParams {
            base_dt: selected_dt,
            cell_offset,
            num_cells,
            selected_idx,
            current_idx: selected_idx - cell_offset,
            interval_minutes: 60,
            cell_w: CELL_WIDTH as usize,
            use_24h,
            offset_m,
            now_tz,
            shading: None,
        }
    }

    fn make_params(use_24h: bool) -> TimelineParams {
        // 2026-03-19 22:00 UTC, select +2h → midnight 2026-03-20 (Friday) at the
        // center cell so day-label rendering fits inside a 10-cell window.
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 3, 19, 22, 0, 0).unwrap();
        let selected_dt = tz.with_ymd_and_hms(2026, 3, 20, 0, 0, 0).unwrap();
        params_h1(now_tz, selected_dt, 2, 10, use_24h, 0)
    }

    fn spans_to_string(spans: &[Span]) -> String {
        spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn day_label_aligns_with_midnight_digit_24h() {
        let p = make_params(true);
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        let midnight_cell = p.selected_idx as usize;
        let expected_pos = midnight_cell * p.cell_w + 2;
        let first_non_space = text.find(|c: char| c != ' ').unwrap();
        assert_eq!(
            first_non_space, expected_pos,
            "24h: label should start at digit offset +2, got text: '{text}'"
        );
    }

    #[test]
    fn day_label_aligns_with_midnight_digit_12h() {
        let p = make_params(false);
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        let midnight_cell = p.selected_idx as usize;
        let expected_pos = midnight_cell * p.cell_w + 1;
        let first_non_space = text.find(|c: char| c != ' ').unwrap();
        assert_eq!(
            first_non_space, expected_pos,
            "12h: label should start at digit offset +1, got text: '{text}'"
        );
    }

    #[test]
    fn day_label_includes_month_on_month_boundary() {
        let tz: Tz = chrono_tz::UTC;
        // 2026-03-31 22:00 UTC, select +2h → midnight 2026-04-01 (Wed) at center.
        let now_tz = tz.with_ymd_and_hms(2026, 3, 31, 22, 0, 0).unwrap();
        let selected_dt = tz.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();
        let p = params_h1(now_tz, selected_dt, 2, 10, true, 0);
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        assert!(
            text.contains("WED 1, April"),
            "month boundary label should include month name, got: '{text}'"
        );
        assert!(
            !text.contains("2026"),
            "same-year label should not include year, got: '{text}'"
        );
    }

    #[test]
    fn day_label_includes_month_and_year_on_year_boundary() {
        let tz: Tz = chrono_tz::UTC;
        // Wider 14-cell window so the long "FRI 1, January, 2027" label fits.
        let now_tz = tz.with_ymd_and_hms(2026, 12, 31, 22, 0, 0).unwrap();
        let selected_dt = tz.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let p = params_h1(now_tz, selected_dt, 2, 16, true, 0);
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        assert!(
            text.contains("FRI 1, January, 2027"),
            "year boundary label should include month and year, got: '{text}'"
        );
    }

    fn selected_bg_text(spans: &[Span]) -> String {
        spans
            .iter()
            .filter(|s| s.style.bg == Some(theme::SELECTED_BG))
            .map(|s| s.content.as_ref())
            .collect()
    }

    #[test]
    fn selection_at_midnight_highlights_full_day_label() {
        // make_params: now=2026-03-19 22:00, +2h selected → midnight 2026-03-20 (Fri).
        let p = make_params(true);
        let spans = build_day_spans(&p);
        let sel_text = selected_bg_text(&spans);
        assert!(
            sel_text.contains("FRI 20"),
            "selection at midnight should highlight full label, got selected text: '{sel_text}'"
        );
    }

    #[test]
    fn selection_not_at_midnight_keeps_single_cell_highlight() {
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 3, 19, 22, 0, 0).unwrap();
        // Selected = 03:00 next day (cell_offset = +5 hours from now=22:00)
        let selected_dt = tz.with_ymd_and_hms(2026, 3, 20, 3, 0, 0).unwrap();
        let p = params_h1(now_tz, selected_dt, 5, 10, true, 0);
        let spans = build_day_spans(&p);
        let sel_text = selected_bg_text(&spans);
        assert!(
            !sel_text.contains("THU"),
            "non-midnight selection should not highlight day label, got selected text: '{sel_text}'"
        );
    }

    #[test]
    fn selection_at_midnight_highlights_label_with_month_suffix() {
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 3, 31, 22, 0, 0).unwrap();
        // Select midnight crossing into April 1 (cell_offset = +2 hours)
        let selected_dt = tz.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();
        let p = params_h1(now_tz, selected_dt, 2, 10, true, 0);
        let spans = build_day_spans(&p);
        let sel_text = selected_bg_text(&spans);
        assert!(
            sel_text.contains("WED 1, April"),
            "selection at month-boundary midnight should highlight full label including month, got: '{sel_text}'"
        );
    }

    // --- Spec: "Three-tier hour shading" ---

    #[test]
    fn hour_fg_working_hours() {
        let wh = WorkingHoursConfig::default();
        assert_eq!(hour_fg_color(9, &wh), theme::HOUR_FG);
        assert_eq!(hour_fg_color(12, &wh), theme::HOUR_FG);
        assert_eq!(hour_fg_color(17, &wh), theme::HOUR_FG);
    }

    #[test]
    fn hour_fg_transition_hours() {
        let wh = WorkingHoursConfig::default();
        assert_eq!(hour_fg_color(7, &wh), theme::HOUR_FG_TRANSITION);
        assert_eq!(hour_fg_color(8, &wh), theme::HOUR_FG_TRANSITION);
        assert_eq!(hour_fg_color(18, &wh), theme::HOUR_FG_TRANSITION);
        assert_eq!(hour_fg_color(19, &wh), theme::HOUR_FG_TRANSITION);
    }

    #[test]
    fn hour_fg_night_hours() {
        let wh = WorkingHoursConfig::default();
        assert_eq!(hour_fg_color(0, &wh), theme::HOUR_FG_NIGHT);
        assert_eq!(hour_fg_color(2, &wh), theme::HOUR_FG_NIGHT);
        assert_eq!(hour_fg_color(6, &wh), theme::HOUR_FG_NIGHT);
        assert_eq!(hour_fg_color(20, &wh), theme::HOUR_FG_NIGHT);
        assert_eq!(hour_fg_color(23, &wh), theme::HOUR_FG_NIGHT);
    }

    #[test]
    fn hour_fg_boundaries() {
        let wh = WorkingHoursConfig::default();
        assert_eq!(hour_fg_color(6, &wh), theme::HOUR_FG_NIGHT, "hour 6 should be night");
        assert_eq!(hour_fg_color(7, &wh), theme::HOUR_FG_TRANSITION, "hour 7 should be transition");
        assert_eq!(hour_fg_color(9, &wh), theme::HOUR_FG, "hour 9 should be working");
        assert_eq!(hour_fg_color(17, &wh), theme::HOUR_FG, "hour 17 should be working");
        assert_eq!(hour_fg_color(18, &wh), theme::HOUR_FG_TRANSITION, "hour 18 should be transition");
        assert_eq!(hour_fg_color(19, &wh), theme::HOUR_FG_TRANSITION, "hour 19 should be transition");
        assert_eq!(hour_fg_color(20, &wh), theme::HOUR_FG_NIGHT, "hour 20 should be night");
    }

    // --- Spec: "Hour-offset indicator" / "format_offset" ---

    #[test]
    fn format_offset_zero_is_empty() {
        assert_eq!(format_offset(0), "");
    }

    #[test]
    fn format_offset_pure_hours_singular() {
        assert_eq!(format_offset(60), "in 1 hour");
        assert_eq!(format_offset(-60), "1 hour ago");
    }

    #[test]
    fn format_offset_pure_hours_plural() {
        assert_eq!(format_offset(120), "in 2 hours");
        assert_eq!(format_offset(-300), "5 hours ago");
        assert_eq!(format_offset(6000), "in 100 hours");
        assert_eq!(format_offset(-2880), "48 hours ago");
    }

    #[test]
    fn format_offset_pure_minutes_singular() {
        assert_eq!(format_offset(1), "in 1 minute");
        assert_eq!(format_offset(-1), "1 minute ago");
    }

    #[test]
    fn format_offset_pure_minutes_plural() {
        assert_eq!(format_offset(15), "in 15 minutes");
        assert_eq!(format_offset(30), "in 30 minutes");
        assert_eq!(format_offset(45), "in 45 minutes");
        assert_eq!(format_offset(-30), "30 minutes ago");
    }

    #[test]
    fn format_offset_mixed_hours_and_minutes() {
        assert_eq!(format_offset(75), "in 1 hour 15 minutes");
        assert_eq!(format_offset(-150), "2 hours 30 minutes ago");
        assert_eq!(format_offset(61), "in 1 hour 1 minute");
        assert_eq!(format_offset(-121), "2 hours 1 minute ago");
    }

    // --- Spec: "is_hour_cell helper" ---

    /// Left-info time string preserves the actual wall-clock minute when
    /// `cell_offset == 0`. With a sub-hour interval, naively using the
    /// floored cell-anchor would clamp `:37` → `:30`/`:15`, which is the
    /// regression this test guards against.
    #[test]
    fn left_info_time_preserves_actual_minute_at_cell_offset_zero() {
        let tz: Tz = "Europe/Bucharest".parse().unwrap();
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 15, 37, 0).unwrap();
        let interval_minutes = 30;
        let cell_offset: i32 = 0;
        // Mirrors the render-time formula:
        //   display_dt = compute_datetime_for_minutes(now_tz, cell_offset * interval)
        let display_dt =
            compute_datetime_for_minutes(now_tz, cell_offset * interval_minutes);
        assert_eq!(display_dt.format("%H:%M").to_string(), "15:37");
    }

    /// At sub-hour intervals, navigating preserves the original wall-clock
    /// minute (mirroring the H1 path where 15:37 → 16:37 → 17:37). At M30
    /// with `cell_offset == 2`, 15:37 → 17:37 (not 17:30).
    #[test]
    fn left_info_time_preserves_minute_through_navigation() {
        let tz: Tz = "Europe/Bucharest".parse().unwrap();
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 15, 37, 0).unwrap();
        let interval_minutes = 30;
        let cell_offset: i32 = 2; // +2 cells × 30 min = +1h
        let display_dt =
            compute_datetime_for_minutes(now_tz, cell_offset * interval_minutes);
        assert_eq!(display_dt.format("%H:%M").to_string(), "16:37");

        // Same minute is preserved at M15 too: +4 cells × 15 min = +1h.
        let interval_15 = 15;
        let cell_offset_15: i32 = 4;
        let display_dt_15 =
            compute_datetime_for_minutes(now_tz, cell_offset_15 * interval_15);
        assert_eq!(display_dt_15.format("%H:%M").to_string(), "16:37");
    }

    /// The cell-anchor (used for timeline rendering) stays floored to the
    /// interval grid so hour digits column-align across zones, even when
    /// the left-info display preserves the actual minute.
    #[test]
    fn cell_anchor_at_cell_offset_zero_is_floored_for_sub_hour_intervals() {
        let tz: Tz = "Europe/Bucharest".parse().unwrap();
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 15, 37, 0).unwrap();
        let interval_minutes = 30;
        let anchor_dt = floor_to_interval(now_tz, interval_minutes);
        let cell_anchor = compute_datetime_for_minutes(anchor_dt, 0);
        assert_eq!(cell_anchor.format("%H:%M").to_string(), "15:30");

        // At H1 the anchor is identical to `now_tz` (no flooring), so
        // the cell-anchor and display-dt collapse to the same value —
        // this is the property that makes 60-min mode preserve `:37`
        // today, and what we now mirror for sub-hour intervals on the
        // info column.
        let anchor_h1 = floor_to_interval(now_tz, 60);
        assert_eq!(anchor_h1.format("%H:%M").to_string(), "15:37");
    }

    #[test]
    fn is_hour_cell_whole_hour_zone() {
        let tz: Tz = chrono_tz::UTC;
        let dt0 = tz.with_ymd_and_hms(2026, 4, 25, 14, 0, 0).unwrap();
        let dt15 = tz.with_ymd_and_hms(2026, 4, 25, 14, 15, 0).unwrap();
        let dt30 = tz.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap();
        let dt45 = tz.with_ymd_and_hms(2026, 4, 25, 14, 45, 0).unwrap();
        assert!(is_hour_cell(dt0, 0, 15));
        assert!(!is_hour_cell(dt15, 0, 15));
        assert!(!is_hour_cell(dt30, 0, 15));
        assert!(!is_hour_cell(dt45, 0, 15));
    }

    #[test]
    fn is_hour_cell_half_hour_zone_at_m30() {
        let tz: Tz = chrono_tz::UTC;
        let dt30 = tz.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap();
        let dt0 = tz.with_ymd_and_hms(2026, 4, 25, 15, 0, 0).unwrap();
        assert!(is_hour_cell(dt30, 30, 30));
        assert!(!is_hour_cell(dt0, 30, 30));
    }

    #[test]
    fn is_hour_cell_quarter_hour_zone_at_m15() {
        let tz: Tz = chrono_tz::UTC;
        let dt45 = tz.with_ymd_and_hms(2026, 4, 25, 14, 45, 0).unwrap();
        let dt15 = tz.with_ymd_and_hms(2026, 4, 25, 14, 15, 0).unwrap();
        assert!(is_hour_cell(dt45, 45, 15));
        assert!(!is_hour_cell(dt15, 45, 15));
    }

    /// Regression: Nepal `+5:45` at M30 — cells land on `:00`/`:30` so the
    /// natural `:45` hour boundary falls inside the `[:30, :00)` window.
    /// The `:30` cell must be marked as the hour cell so the row still
    /// shows hour digits aligned with other timezones.
    #[test]
    fn is_hour_cell_quarter_hour_zone_at_m30() {
        let tz: Tz = chrono_tz::UTC;
        let dt00 = tz.with_ymd_and_hms(2026, 4, 25, 14, 0, 0).unwrap();
        let dt30 = tz.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap();
        assert!(!is_hour_cell(dt00, 45, 30));
        assert!(is_hour_cell(dt30, 45, 30));
    }

    // --- Spec: "floor_to_interval respects byte-for-byte H1" ---

    #[test]
    fn floor_to_interval_h1_returns_now_unchanged() {
        let tz: Tz = chrono_tz::UTC;
        let now = tz.with_ymd_and_hms(2026, 4, 25, 14, 23, 17).unwrap();
        assert_eq!(floor_to_interval(now, 60), now);
    }

    #[test]
    fn floor_to_interval_m30_floors_to_half_hour() {
        let tz: Tz = chrono_tz::UTC;
        let now = tz.with_ymd_and_hms(2026, 4, 25, 14, 47, 33).unwrap();
        let expected = tz.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap();
        assert_eq!(floor_to_interval(now, 30), expected);
    }

    #[test]
    fn floor_to_interval_m15_floors_to_quarter_hour() {
        let tz: Tz = chrono_tz::UTC;
        let now = tz.with_ymd_and_hms(2026, 4, 25, 14, 23, 17).unwrap();
        let expected = tz.with_ymd_and_hms(2026, 4, 25, 14, 15, 0).unwrap();
        assert_eq!(floor_to_interval(now, 15), expected);
    }

    // --- Spec: "60-minute interval is byte-for-byte identical to today" /
    //          "Hour row renders a tick on intermediate cells" /
    //          "Sub row renders superscript minute markers on intermediate cells" ---

    fn params_for(
        now_tz: chrono::DateTime<Tz>,
        selected_dt: chrono::DateTime<Tz>,
        interval_minutes: i32,
        num_cells: i32,
        use_24h: bool,
        offset_m: i32,
    ) -> TimelineParams {
        let selected_idx = num_cells / 2;
        TimelineParams {
            base_dt: selected_dt,
            cell_offset: 0,
            num_cells,
            selected_idx,
            current_idx: selected_idx,
            interval_minutes,
            cell_w: CELL_WIDTH as usize,
            use_24h,
            offset_m,
            now_tz,
            shading: None,
        }
    }

    /// 8.1 — At H1 the UTC row hour text contains plain hour digits and the
    /// sub-row is blank in 24h mode (no `·`, no `⁰⁰`, no superscripts).
    #[test]
    fn h1_utc_row_is_byte_for_byte_legacy() {
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 14, 0, 0).unwrap();
        let p = params_for(now_tz, now_tz, 60, 8, true, 0);

        let hour = spans_to_string(&build_hour_spans(&p));
        assert!(
            !hour.contains('·'),
            "H1 hour row must not contain `·`, got: '{hour}'"
        );

        let sub = spans_to_string(&build_ampm_spans(&p));
        assert!(
            !sub.contains("⁰⁰") && !sub.contains("¹⁵") && !sub.contains("³⁰") && !sub.contains("⁴⁵"),
            "H1 24h whole-hour zone sub-row must be blank (legacy), got: '{sub}'"
        );
        // Sub-row is purely whitespace in this configuration.
        assert!(
            sub.chars().all(char::is_whitespace),
            "H1 24h whole-hour sub-row should be whitespace only, got: '{sub}'"
        );
    }

    /// 8.2 — At M15 the UTC row hour text shows `· · ·` between consecutive
    /// hour digits. The sub-row shows only intermediate markers `¹⁵ ³⁰ ⁴⁵`;
    /// hour cells of 24h whole-hour zones stay blank (no `⁰⁰`) for visual
    /// contrast — the hour digit on the row above already marks the slot.
    #[test]
    fn m15_utc_row_has_three_intermediates_and_minute_markers() {
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 14, 0, 0).unwrap();
        // 8 cells × 15 min = 2 hours window, selected at center (14:00).
        let p = params_for(now_tz, now_tz, 15, 8, true, 0);

        let hour = spans_to_string(&build_hour_spans(&p));
        // Between two consecutive hour cells we expect exactly 3 `·` ticks.
        // Each cell is 3 chars wide so ticks are separated by 2 spaces.
        assert!(
            hour.contains("·  ·  ·"),
            "M15 hour row should contain three consecutive `·` ticks between hour digits, got: '{hour}'"
        );
        // Sanity: total ticks in the 8-cell window = (8 - 2 hour cells) = 6.
        assert_eq!(
            hour.matches('·').count(),
            6,
            "expected 6 intermediate ticks across the M15 8-cell window, got: '{hour}'"
        );

        let sub = spans_to_string(&build_ampm_spans(&p));
        // Hour cells (whole-hour 24h zone) stay blank: no ⁰⁰ in sub-row.
        assert!(
            !sub.contains("⁰⁰"),
            "M15 24h whole-hour sub-row must NOT contain `⁰⁰` on hour cells, got: '{sub}'"
        );
        for marker in ["¹⁵", "³⁰", "⁴⁵"] {
            assert!(
                sub.contains(marker),
                "M15 sub-row should contain intermediate '{marker}', got: '{sub}'"
            );
        }
        // Order check: ¹⁵ then ³⁰ then ⁴⁵.
        let p15 = sub.find("¹⁵").unwrap();
        let p30 = sub.find("³⁰").unwrap();
        let p45 = sub.find("⁴⁵").unwrap();
        assert!(p15 < p30 && p30 < p45, "ordered markers expected, got: '{sub}'");
    }

    /// 8.3 — At M15 a Nepal-like row (offset_m = 45) keeps `⁴⁵` on hour cells
    /// and shows `⁰⁰ ¹⁵ ³⁰` on the three intermediate cells. The `⁰⁰` on the
    /// `:00` intermediate is intentional: the hour row above only shows a
    /// `·` tick on intermediate cells (no hour digit), so the sub-row needs
    /// to mark the natural hour boundary explicitly.
    #[test]
    fn m15_nepal_row_keeps_45_on_hour_cells() {
        let tz: Tz = chrono_tz::UTC;
        // We use UTC's wall clock but lie that offset_m = 45 so that hour
        // cells live at minute :45 (matching Nepal +5:45 semantics).
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 10, 45, 0).unwrap();
        let p = params_for(now_tz, now_tz, 15, 8, true, 45);

        let sub = spans_to_string(&build_ampm_spans(&p));
        // Two hour cells in the window each emit ⁴⁵ on the sub-row.
        assert!(
            sub.matches("⁴⁵").count() >= 2,
            "M15 Nepal sub-row should keep ⁴⁵ on hour cells (≥2 occurrences), got: '{sub}'"
        );
        for marker in ["⁰⁰", "¹⁵", "³⁰"] {
            assert!(
                sub.contains(marker),
                "M15 Nepal sub-row should contain intermediate '{marker}', got: '{sub}'"
            );
        }

        let hour = spans_to_string(&build_hour_spans(&p));
        assert!(
            hour.contains("·  ·  ·"),
            "M15 Nepal hour row should still show three intermediate ticks, got: '{hour}'"
        );
    }

    /// Regression for the M30 + quarter-hour-zone bug: Nepal-like row
    /// (`offset_m = 45`) at M30 must still show one hour digit per hour
    /// (on the `:30` cells), with `·` ticks on intermediate `:00` cells
    /// and `⁰⁰` markers on the sub-row of those intermediate cells. The
    /// hour cells keep their existing `⁴⁵` glyph in 24h mode so column
    /// alignment with whole-hour zones is preserved.
    #[test]
    fn m30_nepal_row_shows_hour_digits_on_30_cells() {
        let tz: Tz = chrono_tz::UTC;
        // Anchor at :30 wall (a "fake Nepal" cell at minute 30 representing
        // the [:30, :00 next) window that contains the natural :45 boundary).
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap();
        let p = params_for(now_tz, now_tz, 30, 8, true, 45);

        let hour = spans_to_string(&build_hour_spans(&p));
        // Must contain at least one wall-hour digit (regression: previously
        // the row was 100% ticks because `is_hour_cell` never matched).
        let has_digits = hour.chars().any(|c| c.is_ascii_digit());
        assert!(
            has_digits,
            "M30 Nepal hour row must show wall-hour digits, got: '{hour}'"
        );
        // Tick count = number of intermediate (`:00`) cells = num_cells / 2.
        assert_eq!(
            hour.matches('·').count(),
            (p.num_cells / 2) as usize,
            "M30 Nepal: expected one tick per `:00` intermediate cell, got: '{hour}'"
        );

        let sub = spans_to_string(&build_ampm_spans(&p));
        // Hour cells (`:30`) keep ⁴⁵ marker.
        assert!(
            sub.contains("⁴⁵"),
            "M30 Nepal sub-row should keep ⁴⁵ on hour cells, got: '{sub}'"
        );
        // Intermediate cells at wall :00 show ⁰⁰ superscript so the sub-row
        // marks the natural hour boundary that the `·` tick above does not.
        assert!(
            sub.contains("⁰⁰"),
            "M30 Nepal sub-row should show ⁰⁰ on intermediate `:00` cells, got: '{sub}'"
        );
    }

    /// 8.4 — At M15 in 12h mode (San Jose) hour cells keep am/pm and
    /// intermediate cells show wall-clock minute superscripts.
    #[test]
    fn m15_12h_row_keeps_ampm_on_hour_cells() {
        let tz: Tz = chrono_tz::UTC; // wall-clock chosen for simplicity
        let now_tz = tz.with_ymd_and_hms(2026, 4, 25, 14, 0, 0).unwrap();
        let p = params_for(now_tz, now_tz, 15, 8, false, 0);

        let sub = spans_to_string(&build_ampm_spans(&p));
        // 14:00 → "pm" on the hour cells; should appear at least once.
        assert!(
            sub.contains("pm") || sub.contains("am"),
            "M15 12h sub-row should keep am/pm on hour cells, got: '{sub}'"
        );
        for marker in ["¹⁵", "³⁰", "⁴⁵"] {
            assert!(
                sub.contains(marker),
                "M15 12h sub-row should contain intermediate '{marker}', got: '{sub}'"
            );
        }
    }
}
