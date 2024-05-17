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
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match self.current_screen {
                    app::CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') => self.current_pane = CurrentPane::Collections,
                        KeyCode::Char('j') => self.current_pane = CurrentPane::ApiPaths,
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
