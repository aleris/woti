use std::time::Duration;

use chrono::{Datelike, Offset, Timelike, Utc};
use chrono_tz::Tz;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

use crate::config::TimeFormat;

use super::app::App;
use super::theme;
use super::{BLOCK_HEIGHT, CELL_WIDTH, INFO_COL_WIDTH, TIMELINE_GAP};

pub(super) fn compute_datetime_for_hour(
    _tz: Tz,
    now_tz: chrono::DateTime<Tz>,
    offset_from_current: i32,
) -> chrono::DateTime<Tz> {
    let duration = chrono::Duration::hours(offset_from_current as i64);
    now_tz + duration
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
            .fg(theme::SWITCHER_ACTIVE_FG)
            .bg(theme::SWITCHER_ACTIVE_BG)
            .bold();
        let dim = Style::default().fg(theme::SWITCHER_DIM_FG);
        let sep = Style::default().fg(theme::SWITCHER_SEP);

        let fmt_switcher: Vec<Span> = vec![
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
        let right_w: usize = fmt_switcher.iter().map(|s| s.width()).sum();
        let spacer = (area.width as usize).saturating_sub(left_w + right_w);

        let mut spans = shortcuts;
        spans.push(Span::raw(" ".repeat(spacer)));
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

        let utc_secs = now_tz.offset().fix().local_minus_utc();
        let offset_m = (utc_secs.abs() % 3600) / 60;

        let tl = TimelineParams {
            start_hour,
            base_hour,
            current_hour,
            hour_offset: self.hour_offset,
            num_cells,
            cell_w,
            use_24h,
            offset_m,
            tz,
            now_tz,
        };

        let hour_spans = build_hour_spans(&tl);
        let ampm_spans = build_ampm_spans(&tl);
        let day_spans = build_day_spans(&tl);

        let mut line1 = vec![Span::raw(" ".repeat(left_pad))];
        line1.extend(day_spans);
        frame.render_widget(Paragraph::new(Line::from(line1)), rows[0]);

        let mut line2 = build_info_line(entry, &tz_abbr, &time_str, now_tz, info_w);
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
    start_hour: i32,
    base_hour: i32,
    current_hour: i32,
    hour_offset: i32,
    num_cells: i32,
    cell_w: usize,
    use_24h: bool,
    offset_m: i32,
    tz: Tz,
    now_tz: chrono::DateTime<Tz>,
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

fn build_hour_spans(p: &TimelineParams) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    for i in 0..p.num_cells {
        let h = p.start_hour + i;
        let hour_in_day = ((h % 24) + 24) % 24;
        let is_selected = h == p.base_hour;
        let is_local = h == p.current_hour && p.hour_offset != 0;

        let h_num = if p.use_24h {
            format!("{:>2}", hour_in_day)
        } else {
            let h12 = hour_in_day % 12;
            let h12 = if h12 == 0 { 12 } else { h12 };
            format!("{:>2}", h12)
        };

        let style = if is_selected {
            selected_style()
        } else if is_local {
            local_style().fg(theme::HOUR_FG)
        } else {
            Style::default().fg(theme::HOUR_FG)
        };

        spans.push(Span::raw(" "));
        spans.push(Span::styled(h_num, style));
    }
    spans
}

fn minutes_superscript(m: i32) -> &'static str {
    match m {
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
        let h = p.start_hour + i;
        let hour_in_day = ((h % 24) + 24) % 24;
        let is_selected = h == p.base_hour;
        let is_local = h == p.current_hour && p.hour_offset != 0;

        let (text, style) = if !p.use_24h {
            let style = if is_selected {
                selected_style()
            } else if is_local {
                local_style().fg(theme::AMPM_FG)
            } else {
                Style::default()
                    .fg(theme::AMPM_FG)
                    .add_modifier(Modifier::DIM)
            };

            let text = if p.offset_m != 0 {
                let frac = minutes_fraction(p.offset_m);
                let meridiem = if hour_in_day < 12 { "a" } else { "p" };
                format!("{frac}{meridiem}")
            } else {
                let ampm = if hour_in_day < 12 { "am" } else { "pm" };
                ampm.to_string()
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

            let text = if p.offset_m != 0 {
                minutes_superscript(p.offset_m).to_string()
            } else {
                "  ".to_string()
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

    for i in 0..p.num_cells {
        let h = p.start_hour + i;
        let hour_in_day = ((h % 24) + 24) % 24;

        if hour_in_day == 0 {
            let dt = compute_datetime_for_hour(p.tz, p.now_tz, h - p.current_hour);
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
                }
            }
        }
    }

    let style_for = |pos: usize| -> Style {
        let cell_idx = pos / p.cell_w;
        let pos_in_cell = pos % p.cell_w;
        let h = p.start_hour + cell_idx as i32;
        let is_sel = h == p.base_hour;
        let is_loc = h == p.current_hour && p.hour_offset != 0;
        let is_lab = day_is_label[pos];
        let has_bg = pos_in_cell > 0;
        if is_sel && has_bg {
            Style::default()
                .fg(theme::SELECTED_FG)
                .bg(theme::SELECTED_BG)
                .bold()
        } else if is_loc && has_bg {
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_params(use_24h: bool) -> TimelineParams {
        let tz: Tz = chrono_tz::UTC;
        let now_tz = tz.with_ymd_and_hms(2026, 3, 19, 22, 0, 0).unwrap();
        TimelineParams {
            start_hour: -2,
            base_hour: 0,
            current_hour: 22,
            hour_offset: -22,
            num_cells: 10,
            cell_w: CELL_WIDTH as usize,
            use_24h,
            offset_m: 0,
            tz,
            now_tz,
        }
    }

    fn spans_to_string(spans: &[Span]) -> String {
        spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn day_label_aligns_with_midnight_digit_24h() {
        let p = make_params(true);
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        let midnight_cell = 2_usize;
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
        let midnight_cell = 2_usize;
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
        // March 31, 2026 22:00 — cell 2 crosses midnight into April 1 (Wednesday)
        let now_tz = tz.with_ymd_and_hms(2026, 3, 31, 22, 0, 0).unwrap();
        let p = TimelineParams {
            start_hour: 22,
            base_hour: 22,
            current_hour: 22,
            hour_offset: 0,
            num_cells: 10,
            cell_w: CELL_WIDTH as usize,
            use_24h: true,
            offset_m: 0,
            tz,
            now_tz,
        };
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
        // Dec 31, 2026 22:00 — cell 2 crosses midnight into Jan 1, 2027 (Friday)
        let now_tz = tz.with_ymd_and_hms(2026, 12, 31, 22, 0, 0).unwrap();
        let p = TimelineParams {
            start_hour: 22,
            base_hour: 22,
            current_hour: 22,
            hour_offset: 0,
            num_cells: 10,
            cell_w: CELL_WIDTH as usize,
            use_24h: true,
            offset_m: 0,
            tz,
            now_tz,
        };
        let spans = build_day_spans(&p);
        let text = spans_to_string(&spans);
        assert!(
            text.contains("FRI 1, January, 2027"),
            "year boundary label should include month and year, got: '{text}'"
        );
    }
}
