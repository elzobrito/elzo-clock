use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::{run_widget, SecurityConfig, WidgetConfig};

pub struct WidgetRuntime {
    pub title: String,
    pub command_argv: Vec<String>,
    pub command: Option<String>,
    pub security: SecurityConfig,
    pub refresh: Duration,
    pub timeout_secs: u64,
    pub last_run: Option<Instant>,
    pub output: String,
    pub running_hint: String,
    running: bool,
    result_rx: Option<Receiver<String>>,
}

impl WidgetRuntime {
    pub fn from_config(cfg: &WidgetConfig, security: &SecurityConfig) -> Self {
        let title = cfg
            .title
            .clone()
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| {
                if let Some(first) = cfg.command_argv.first() {
                    return first.clone();
                }
                cfg.command
                    .as_deref()
                    .and_then(|c| c.split_whitespace().next())
                    .unwrap_or("widget")
                    .to_string()
            });
        Self {
            title,
            command_argv: cfg.command_argv.clone(),
            command: cfg.command.clone(),
            security: security.clone(),
            refresh: Duration::from_secs(cfg.refresh_secs.max(5)),
            timeout_secs: cfg.timeout_secs.max(1),
            last_run: None,
            output: "carregando…".into(),
            running_hint: String::new(),
            running: false,
            result_rx: None,
        }
    }

    pub fn needs_refresh(&self) -> bool {
        if self.running {
            return false;
        }
        match self.last_run {
            None => true,
            Some(t) => t.elapsed() >= self.refresh,
        }
    }

    pub fn refresh(&mut self) {
        if self.running {
            return;
        }

        let widget_cfg = WidgetConfig {
            title: Some(self.title.clone()),
            command_argv: self.command_argv.clone(),
            command: self.command.clone(),
            refresh_secs: self.refresh.as_secs(),
            timeout_secs: self.timeout_secs,
        };
        let security = self.security.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(run_widget(&widget_cfg, &security));
        });

        self.running = true;
        self.result_rx = Some(rx);
        self.running_hint = "executando…".into();
    }

    pub fn poll_result(&mut self) {
        let result = match self.result_rx.as_ref().map(Receiver::try_recv) {
            Some(Ok(output)) => Some(output),
            Some(Err(TryRecvError::Disconnected)) => {
                Some("erro: worker do widget encerrou sem resultado\n".into())
            }
            Some(Err(TryRecvError::Empty)) | None => None,
        };

        if let Some(output) = result {
            self.output = output;
            self.last_run = Some(Instant::now());
            self.running = false;
            self.result_rx = None;
            self.running_hint = format!("↻ {}s", self.refresh.as_secs());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(command: &str, refresh_secs: u64, timeout_secs: u64) -> WidgetConfig {
        WidgetConfig {
            title: Some("teste".into()),
            command_argv: vec![],
            command: Some(command.into()),
            refresh_secs,
            timeout_secs,
        }
    }

    #[test]
    fn clamps_refresh_and_timeout_limits() {
        let widget = WidgetRuntime::from_config(&config("printf ok", 0, 0), &SecurityConfig::default());
        assert_eq!(widget.refresh, Duration::from_secs(5));
        assert_eq!(widget.timeout_secs, 1);
    }

    #[test]
    fn refresh_is_non_blocking_and_does_not_duplicate_running_work() {
        let mut widget = WidgetRuntime::from_config(
            &config("sleep 1; printf pronto", 5, 3),
            &SecurityConfig::default(),
        );
        let started = Instant::now();
        widget.refresh();
        assert!(started.elapsed() < Duration::from_millis(200));
        assert!(widget.running);
        widget.refresh();
        assert!(widget.running);

        let deadline = Instant::now() + Duration::from_secs(3);
        while widget.running && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(20));
            widget.poll_result();
        }
        assert!(!widget.running);
        assert_eq!(widget.output, "pronto");
        assert!(widget.last_run.is_some());
    }

    #[test]
    fn argv_widget_title_from_binary() {
        let cfg = WidgetConfig {
            title: None,
            command_argv: vec!["eclock-esaa".into()],
            command: None,
            refresh_secs: 10,
            timeout_secs: 5,
        };
        let w = WidgetRuntime::from_config(&cfg, &SecurityConfig::default());
        assert_eq!(w.title, "eclock-esaa");
    }
}
