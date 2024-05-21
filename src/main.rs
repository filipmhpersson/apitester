use app::App;
use app::CurrentPane;

use crossterm::event;
use crossterm::event::*;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::Backend;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::fs;
use std::io;
mod api;
mod apirunner;
mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let file = fs::read_to_string("sample.json")?;
    let json: api::ApiDocs = serde_json::from_str(&file).unwrap();
    //dbg!("{}", json);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let res = App::new(json).run(&mut terminal);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;
            if let Event::Key(key) = event::read()? {
                match self.current_pane {
                    CurrentPane::FilterApi => match key.code {
                        KeyCode::Char(value) if key.modifiers.is_empty() => {
                            self.push_filter(value);
                        }
                        KeyCode::Enter => self.current_pane = CurrentPane::ApiPaths,
                        KeyCode::Esc => {
                            self.clear_filter();
                            self.current_pane = CurrentPane::ApiPaths;
                        }
                        KeyCode::Backspace => {
                            self.pop_filter();
                        }
                        _ => (),
                    },
                    CurrentPane::Collections => match key.code {
                        KeyCode::Char('j') if key.modifiers.is_empty() => {
                            self.scroll_down_selected_env(1)
                        }
                        KeyCode::Char('k') if key.modifiers.is_empty() => {
                            self.scroll_up_selected_env(1)
                        }
                        _ => {}
                    },
                    CurrentPane::ApiPaths => match key.code {
                        KeyCode::Char('j') if key.modifiers.is_empty() => {
                            self.scroll_down_cursor_path(1)
                        }
                        KeyCode::Char('k') if key.modifiers.is_empty() => {
                            self.scroll_up_cursor_path(1)
                        }

                        KeyCode::Char('f') if key.modifiers.is_empty() => {
                            self.current_pane = CurrentPane::FilterApi;
                        }
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.scroll_down_cursor_path(15);
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.scroll_up_cursor_path(15);
                        }
                        _ => {}
                    },
                    _ => {}
                }
                match self.current_screen {
                    app::CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.current_pane = CurrentPane::Collections
                        }
                        KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.current_pane = CurrentPane::ApiPaths
                        }
                        KeyCode::Tab if key.modifiers.is_empty() => {
                            self.next_action();
                        }

                        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                            self.prev_action();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        self.print_json()?;
        Ok(true)
    }
}
