use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
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
    let app = app::App::new();
    let res = run_app(&mut terminal, &mut app);
    dbg!("{}", app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}")
    }
    Ok(())
}
