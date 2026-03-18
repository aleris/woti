use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use super::app::App;
use super::DEBOUNCE_MS;

impl App {
    pub(super) fn event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> io::Result<()> {
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
}
