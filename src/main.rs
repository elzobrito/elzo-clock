//! elzo-clock (`eclock`) — personal terminal clock dashboard.
//! Inspired by the idea of clock-tui/tclock (widgets under a big clock), written from scratch.

mod app;
mod clock_digits;
mod config;
mod sanitize;
mod theme;
mod widgets;

use std::error::Error;
use std::io::{self, stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::App;
use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load();
    let mut terminal = TerminalSession::new()?;
    let mut app = App::new(config);

    run(terminal.terminal_mut(), &mut app)
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalSession {
    fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;

        let mut out = stdout();
        if let Err(err) = execute!(out, EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(err.into());
        }

        match Terminal::new(CrosstermBackend::new(out)) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(err) => {
                let _ = execute!(stdout(), LeaveAlternateScreen);
                let _ = disable_raw_mode();
                Err(err.into())
            }
        }
    }

    fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        app.tick();
        terminal.draw(|frame| app.draw(frame))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('r') => app.refresh_widgets(true),
                    KeyCode::Char('s') => app.toggle_seconds(),
                    KeyCode::Char('d') => app.toggle_date(),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.grow(),
                    KeyCode::Char('-') => app.shrink(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
