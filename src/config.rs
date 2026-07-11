use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub clock: ClockConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub widgets: Vec<WidgetConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// When true, widgets may use legacy `command` via `sh -c`.
    #[serde(default = "default_true")]
    pub allow_shell_command: bool,
    /// If non-empty, only these basenames or absolute paths may be executed.
    #[serde(default)]
    pub widget_allowlist: Vec<String>,
    /// Redact obvious secrets from widget stdout (wired by SEC-020).
    #[serde(default = "default_true")]
    pub redact_widget_output: bool,
    /// Mask agenda event titles when true (wired by SEC-023B).
    #[serde(default)]
    pub privacy_mode: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_shell_command: true,
            widget_allowlist: Vec::new(),
            redact_widget_output: true,
            privacy_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockConfig {
    #[serde(default = "default_true")]
    pub show_date: bool,
    #[serde(default = "default_true")]
    pub show_seconds: bool,
    /// Digit scale: 1 = compact, 2 = large, 3 = huge
    #[serde(default = "default_size")]
    pub size: u8,
    /// Theme / color: akita (default), tclock, cyan, green, magenta, yellow, blue, white
    #[serde(default = "default_color")]
    pub color: String,
    /// Language used for human-readable dates: pt-BR (default) or en.
    #[serde(default = "default_language", alias = "locale")]
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    #[serde(default)]
    pub title: Option<String>,
    /// Preferred: argv without shell.
    #[serde(default)]
    pub command_argv: Vec<String>,
    /// Legacy shell string, run via `sh -c` when allow_shell_command.
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default = "default_refresh")]
    pub refresh_secs: u64,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_true() -> bool {
    true
}
fn default_size() -> u8 {
    2
}
fn default_color() -> String {
    "akita".into()
}
fn default_language() -> String {
    "pt-BR".into()
}
fn default_refresh() -> u64 {
    300
}
fn default_timeout() -> u64 {
    15
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            show_date: true,
            show_seconds: true,
            size: 2,
            color: default_color(),
            language: default_language(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock: ClockConfig::default(),
            security: SecurityConfig::default(),
            widgets: vec![
                WidgetConfig {
                    title: Some("ESAA".into()),
                    command_argv: vec!["eclock-esaa".into()],
                    command: None,
                    refresh_secs: 10,
                    timeout_secs: 20,
                },
                WidgetConfig {
                    title: Some("Agenda (hoje)".into()),
                    command_argv: vec!["eclock-gcal".into()],
                    command: None,
                    refresh_secs: 600,
                    timeout_secs: 15,
                },
                WidgetConfig {
                    title: Some("Sistema".into()),
                    command_argv: default_sysinfo_argv(),
                    command: None,
                    refresh_secs: 30,
                    timeout_secs: 5,
                },
            ],
        }
    }
}

fn default_sysinfo_argv() -> Vec<String> {
    // Prefer helper on PATH (user: cp scripts/eclock-sysinfo ~/.local/bin/).
    vec!["eclock-sysinfo".into()]
}


#[cfg(unix)]
fn apply_restricted_dir_perms(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
}

#[cfg(unix)]
fn apply_restricted_file_perms(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn apply_restricted_dir_perms(_path: &Path) {}
#[cfg(not(unix))]
fn apply_restricted_file_perms(_path: &Path) {}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("elzo-clock")
            .join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            let cfg = Self::default();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
                apply_restricted_dir_perms(parent);
            }
            if let Ok(toml) = toml::to_string_pretty(&cfg) {
                let header = "# elzo-clock config — edite e reinicie com `eclock`\n\
# Prefira command_argv (sem shell). Ver [security] e docs/spec/ECLOCK-SEC-REMEDIATION.md\n\n";
                let _ = fs::write(&path, format!("{header}{toml}"));
                apply_restricted_file_perms(&path);
            }
            return cfg;
        }
        match fs::read_to_string(&path) {
            Ok(raw) => Self::parse(&raw).unwrap_or_else(|err| {
                eprintln!("elzo-clock: config inválido em {}: {err}", path.display());
                Self::default()
            }),
            Err(err) => {
                eprintln!("elzo-clock: não li {}: {err}", path.display());
                Self::default()
            }
        }
    }

    pub fn parse(raw: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(raw)
    }
}

fn truncate_widget_output(text: &mut String, max_bytes: usize) -> bool {
    if text.len() <= max_bytes {
        return false;
    }

    let mut boundary = max_bytes;
    while !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    text.truncate(boundary);
    text.push_str("\n… (truncado)\n");
    true
}

fn basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
}

/// Returns Ok(executable_key) if allowlist empty or match; Err(message) otherwise.
pub fn check_allowlist(exec: &str, allowlist: &[String]) -> Result<(), String> {
    if allowlist.is_empty() {
        return Ok(());
    }
    let base = basename(exec);
    let allowed = allowlist.iter().any(|entry| {
        entry == exec || entry == base || basename(entry) == base || basename(entry) == exec
    });
    if allowed {
        Ok(())
    } else {
        Err(format!(
            "widget bloqueado pela allowlist: executável '{exec}' não permitido\n\
             allowlist: {allowlist:?}\n"
        ))
    }
}

fn timeout_available() -> bool {
    Command::new("timeout")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_program(program: &str, args: &[String], timeout_secs: u64) -> String {
    let mut cmd = if timeout_available() {
        let mut c = Command::new("timeout");
        c.arg(format!("{timeout_secs}s")).arg(program);
        for a in args {
            c.arg(a);
        }
        c
    } else {
        let mut c = Command::new(program);
        for a in args {
            c.arg(a);
        }
        c
    };

    match cmd.output() {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).to_string();
            if text.trim().is_empty() {
                let err = String::from_utf8_lossy(&out.stderr);
                if !err.trim().is_empty() {
                    text = err.to_string();
                } else if !out.status.success() {
                    text = format!("comando falhou (status {})\n", out.status);
                } else {
                    text = "(sem saída)\n".into();
                }
            }
            const MAX: usize = 8_000;
            truncate_widget_output(&mut text, MAX);
            text
        }
        Err(err) => format!("erro ao executar: {err}\n"),
    }
}

/// Legacy helper used by tests and shell path.
pub fn run_command(command: &str, timeout_secs: u64) -> String {
    run_program("sh", &["-c".into(), command.into()], timeout_secs)
}

/// Execute a widget according to security policy (argv preferred, shell opt-in).
pub fn run_widget(cfg: &WidgetConfig, security: &SecurityConfig) -> String {
    let timeout_secs = cfg.timeout_secs.max(1);

    let raw = if !cfg.command_argv.is_empty() {
        let program = &cfg.command_argv[0];
        if let Err(msg) = check_allowlist(program, &security.widget_allowlist) {
            return msg;
        }
        let args: Vec<String> = cfg.command_argv[1..].to_vec();
        run_program(program, &args, timeout_secs)
    } else if let Some(command) = cfg.command.as_ref().filter(|c| !c.is_empty()) {
        if !security.allow_shell_command {
            return "shell desabilitado (security.allow_shell_command=false); use command_argv\n"
                .into();
        }
        // Allowlist against first shell token heuristic
        let first = command.split_whitespace().next().unwrap_or(command);
        if let Err(msg) = check_allowlist(first, &security.widget_allowlist) {
            return msg;
        }
        run_command(command, timeout_secs)
    } else {
        return "widget sem command_argv nem command\n".into();
    };

    post_process_output(raw, security)
}

fn post_process_output(raw: String, security: &SecurityConfig) -> String {
    let mut text = crate::sanitize::strip_control_chars(&raw);
    if security.redact_widget_output {
        text = crate::sanitize::redact_secrets(&text);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_config_and_rejects_invalid_config() {
        let cfg = Config::parse(
            r#"
[clock]
show_date = false
show_seconds = true
size = 3
color = "cyan"
language = "en"

[[widgets]]
title = "teste"
command = "printf ok"
refresh_secs = 7
timeout_secs = 2
"#,
        )
        .expect("valid TOML");
        assert!(!cfg.clock.show_date);
        assert_eq!(cfg.clock.size, 3);
        assert_eq!(cfg.clock.language, "en");
        assert_eq!(cfg.widgets[0].refresh_secs, 7);
        assert_eq!(cfg.widgets[0].command.as_deref(), Some("printf ok"));
        assert!(Config::parse("[clock\nsize = nope").is_err());
    }

    #[test]
    fn parses_command_argv_and_security() {
        let cfg = Config::parse(
            r#"
[security]
allow_shell_command = false
widget_allowlist = ["eclock-esaa", "printf"]
redact_widget_output = true
privacy_mode = true

[[widgets]]
title = "ESAA"
command_argv = ["eclock-esaa", "/tmp/root"]
refresh_secs = 10
"#,
        )
        .expect("argv config");
        assert!(!cfg.security.allow_shell_command);
        assert!(cfg.security.privacy_mode);
        assert_eq!(cfg.widgets[0].command_argv, vec!["eclock-esaa", "/tmp/root"]);
    }

    #[test]
    fn legacy_config_defaults_to_portuguese() {
        let cfg = Config::parse(
            r#"
[clock]
show_date = true
show_seconds = true
size = 2
color = "akita"
"#,
        )
        .expect("legacy config remains valid");
        assert_eq!(cfg.clock.language, "pt-BR");
        assert!(cfg.security.allow_shell_command);
        assert!(cfg.security.redact_widget_output);
    }

    #[test]
    fn truncation_preserves_utf8_boundaries() {
        let mut text = format!("{}🙂fim", "a".repeat(7_999));
        assert!(truncate_widget_output(&mut text, 8_000));
        assert!(text.starts_with(&"a".repeat(7_999)));
        assert!(text.ends_with("… (truncado)\n"));
        assert!(!text.contains("🙂"));
    }

    #[test]
    fn command_timeout_returns_without_hanging() {
        let started = std::time::Instant::now();
        let output = run_command("sleep 5", 1);
        assert!(started.elapsed() < std::time::Duration::from_secs(3));
        assert!(output.contains("status") || output.contains("sem saída"));
    }

    #[test]
    fn argv_runs_without_shell_metachar_expansion() {
        let cfg = WidgetConfig {
            title: Some("t".into()),
            command_argv: vec!["printf".into(), "%s".into(), "ok".into()],
            command: None,
            refresh_secs: 5,
            timeout_secs: 5,
        };
        let security = SecurityConfig::default();
        let out = run_widget(&cfg, &security);
        assert_eq!(out.trim(), "ok");
    }

    #[test]
    fn allowlist_blocks_disallowed_program() {
        let cfg = WidgetConfig {
            title: Some("t".into()),
            command_argv: vec!["rm".into(), "-rf".into(), "/tmp/nope".into()],
            command: None,
            refresh_secs: 5,
            timeout_secs: 5,
        };
        let security = SecurityConfig {
            widget_allowlist: vec!["eclock-esaa".into(), "printf".into()],
            ..SecurityConfig::default()
        };
        let out = run_widget(&cfg, &security);
        assert!(out.contains("allowlist") || out.contains("bloqueado"));
    }

    #[test]
    fn shell_blocked_when_disallowed() {
        let cfg = WidgetConfig {
            title: Some("t".into()),
            command_argv: vec![],
            command: Some("printf hi".into()),
            refresh_secs: 5,
            timeout_secs: 5,
        };
        let security = SecurityConfig {
            allow_shell_command: false,
            ..SecurityConfig::default()
        };
        let out = run_widget(&cfg, &security);
        assert!(out.contains("shell desabilitado"));
    }

    #[test]
    fn redacts_secrets_when_enabled() {
        let cfg = WidgetConfig {
            title: Some("t".into()),
            command_argv: vec![
                "printf".into(),
                "%s".into(),
                "token ya29.a0ABCDEFGHIJKLMNOP more".into(),
            ],
            command: None,
            refresh_secs: 5,
            timeout_secs: 5,
        };
        let security = SecurityConfig {
            redact_widget_output: true,
            ..SecurityConfig::default()
        };
        let out = run_widget(&cfg, &security);
        assert!(!out.contains("ya29.a0ABCDEFGHIJKLMNOP"));
        assert!(out.contains("[REDACTED]"));
    }

    #[cfg(unix)]
    #[test]
    fn restricted_perms_unix() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("elzo-clock-perm-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        apply_restricted_dir_perms(&dir);
        let mode = fs::metadata(&dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700);
        let file = dir.join("config.toml");
        fs::write(&file, "x").unwrap();
        apply_restricted_file_perms(&file);
        let fmode = fs::metadata(&file).unwrap().permissions().mode() & 0o777;
        assert_eq!(fmode, 0o600);
        let _ = fs::remove_dir_all(&dir);
    }
}
