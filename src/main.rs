//! elzo-clock (`eclock`) — personal terminal clock dashboard.
//! Inspired by the idea of clock-tui/tclock (widgets under a big clock), written from scratch.

mod app;
mod clock_digits;
mod config;
mod sanitize;
mod theme;
mod widgets;

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, stdout};
use std::path::{Path, PathBuf};
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
    let options = match CliOptions::parse(env::args().skip(1)) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("eclock: {message}\n\n{}", usage());
            std::process::exit(2);
        }
    };
    if options.help {
        println!("{}", usage());
        return Ok(());
    }

    let mut config = Config::load();
    if let Some(root) = options.esaa_root {
        apply_esaa_root(&mut config, &root)?;
    }
    let mut terminal = TerminalSession::new()?;
    let mut app = App::new(config);

    run(terminal.terminal_mut(), &mut app)
}

#[derive(Debug, Default, PartialEq)]
struct CliOptions {
    esaa_root: Option<PathBuf>,
    help: bool,
}

impl CliOptions {
    fn parse<I, S>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut options = Self::default();
        let mut args = args.into_iter().map(Into::into);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--path" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--path exige um diretório".to_string())?;
                    if options.esaa_root.is_some() {
                        return Err("--path foi informado mais de uma vez".into());
                    }
                    options.esaa_root = Some(validate_esaa_root(Path::new(&value))?);
                }
                "-h" | "--help" => options.help = true,
                _ => return Err(format!("argumento desconhecido: {arg}")),
            }
        }
        Ok(options)
    }
}

fn validate_esaa_root(path: &Path) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path)
        .map_err(|err| format!("não foi possível acessar {}: {err}", path.display()))?;
    if !canonical.join(".roadmap").is_dir() {
        return Err(format!(
            "{} não é um workspace ESAA: .roadmap/ não encontrado",
            canonical.display()
        ));
    }
    Ok(canonical)
}

fn apply_esaa_root(config: &mut Config, root: &Path) -> Result<(), String> {
    let root = root.to_string_lossy().into_owned();
    let widget = config
        .widgets
        .iter_mut()
        .find(|widget| {
            widget.command_argv.first().is_some_and(|arg| {
                Path::new(arg).file_name().and_then(|name| name.to_str()) == Some("eclock-esaa")
            })
        })
        .ok_or_else(|| "widget eclock-esaa não encontrado na configuração".to_string())?;
    widget.command_argv = vec!["eclock-esaa".into(), root];
    widget.command = None;
    Ok(())
}

fn usage() -> &'static str {
    "Uso: eclock [--path <workspace-esaa>]\n\nOpções:\n  --path <diretório>  usa esse workspace no widget ESAA nesta execução\n  -h, --help          mostra esta ajuda"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_esaa_root_as_two_literal_arguments() {
        let mut config = Config::default();
        apply_esaa_root(&mut config, Path::new("/tmp/workspace esaa")).unwrap();
        assert_eq!(
            config.widgets[0].command_argv,
            vec!["eclock-esaa", "/tmp/workspace esaa"]
        );
        assert!(config.widgets[0].command.is_none());
    }

    #[test]
    fn rejects_unknown_and_incomplete_arguments() {
        assert!(CliOptions::parse(["--unknown"]).is_err());
        assert!(CliOptions::parse(["--path"]).is_err());
    }

    #[test]
    fn help_does_not_require_a_terminal() {
        let options = CliOptions::parse(["--help"]).unwrap();
        assert!(options.help);
    }
}
