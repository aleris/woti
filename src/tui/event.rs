use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use super::app::App;
use super::{ACCEL_MAX_MS, ACCEL_MAX_STEP, DEBOUNCE_MS};

fn compute_step(elapsed: Duration) -> i32 {
    let t = (elapsed.as_millis() as f64 / ACCEL_MAX_MS as f64).clamp(0.0, 1.0);
    1 + ((ACCEL_MAX_STEP - 1) as f64 * t) as i32
}

impl App {
    pub(super) fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> io::Result<()> {
        let mut last_render = Instant::now();
        // Counts pending cell steps. One cell == self.interval.minutes() minutes.
        let mut pending_cell_offset: i32 = 0;
        let mut nav_start: Option<Instant> = None;
        let mut nav_dir: i32 = 0;

        terminal.draw(|f| self.render(f))?;

        loop {
            let timeout =
                Duration::from_millis(if pending_cell_offset != 0 { DEBOUNCE_MS } else { 250 });

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key)
                        if key.kind == KeyEventKind::Press
                            || key.kind == KeyEventKind::Repeat =>
                    {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('q'), _)
                            | (KeyCode::Char('x'), _)
                            | (KeyCode::Esc, _)
                            | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                self.should_quit = true;
                                nav_start = None;
                                nav_dir = 0;
                            }
                            (KeyCode::Left, _) => {
                                let dir = -1;
                                if nav_dir != dir {
                                    nav_start = Some(Instant::now());
                                    nav_dir = dir;
                                }
                                let step = compute_step(nav_start.unwrap().elapsed());
                                pending_cell_offset -= step;
                            }
                            (KeyCode::Right, _) => {
                                let dir = 1;
                                if nav_dir != dir {
                                    nav_start = Some(Instant::now());
                                    nav_dir = dir;
                                }
                                let step = compute_step(nav_start.unwrap().elapsed());
                                pending_cell_offset += step;
                            }
                            (KeyCode::Up, _) => {
                                nav_start = None;
                                nav_dir = 0;
                                if self.scroll_offset > 0 {
                                    self.scroll_offset -= 1;
                                    terminal.draw(|f| self.render(f))?;
                                    last_render = Instant::now();
                                }
                            }
                            (KeyCode::Down, _) => {
                                nav_start = None;
                                nav_dir = 0;
                                let body_h = terminal.size()?.height.saturating_sub(2);
                                let max = self.max_scroll(body_h);
                                if self.scroll_offset < max {
                                    self.scroll_offset += 1;
                                    terminal.draw(|f| self.render(f))?;
                                    last_render = Instant::now();
                                }
                            }
                            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                                nav_start = None;
                                nav_dir = 0;
                                self.copy_selection();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                                nav_start = None;
                                nav_dir = 0;
                                self.cycle_time_format();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                                nav_start = None;
                                nav_dir = 0;
                                self.shading_enabled = !self.shading_enabled;
                                self.config.working_hours.enabled = self.shading_enabled;
                                let _ = self.config.save();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                                nav_start = None;
                                nav_dir = 0;
                                self.cycle_interval();
                                terminal.draw(|f| self.render(f))?;
                                last_render = Instant::now();
                            }
                            _ => {
                                nav_start = None;
                                nav_dir = 0;
                            }
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

                if pending_cell_offset != 0
                    && last_render.elapsed() >= Duration::from_millis(DEBOUNCE_MS)
                {
                    self.cell_offset += pending_cell_offset;
                    pending_cell_offset = 0;
                    terminal.draw(|f| self.render(f))?;
                    last_render = Instant::now();
                }
            } else {
                if pending_cell_offset != 0 {
                    self.cell_offset += pending_cell_offset;
                    pending_cell_offset = 0;
                } else {
                    nav_start = None;
                    nav_dir = 0;
                }
                terminal.draw(|f| self.render(f))?;
                last_render = Instant::now();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_is_1_at_zero() {
        assert_eq!(compute_step(Duration::from_millis(0)), 1);
    }

    #[test]
    fn step_is_2_at_400ms() {
        assert_eq!(compute_step(Duration::from_millis(400)), 2);
    }

    #[test]
    fn step_is_4_at_midpoint() {
        assert_eq!(compute_step(Duration::from_millis(1000)), 4);
    }

    #[test]
    fn step_is_max_at_accel_max_ms() {
        assert_eq!(compute_step(Duration::from_millis(2000)), ACCEL_MAX_STEP);
    }

    #[test]
    fn step_clamps_beyond_max() {
        assert_eq!(compute_step(Duration::from_millis(5000)), ACCEL_MAX_STEP);
        assert_eq!(compute_step(Duration::from_millis(10000)), ACCEL_MAX_STEP);
    }

    #[test]
    fn step_increases_monotonically() {
        let mut prev = 0;
        for ms in (0..=2000).step_by(100) {
            let step = compute_step(Duration::from_millis(ms));
            assert!(step >= prev, "step decreased at {ms}ms: {prev} -> {step}");
            prev = step;
        }
    }
}
