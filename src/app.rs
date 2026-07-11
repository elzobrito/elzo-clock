use chrono::{DateTime, Datelike, Local, NaiveDate};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::clock_digits;
use crate::config::Config;
use crate::theme::Theme;
use crate::widgets::WidgetRuntime;

pub struct App {
    pub widgets: Vec<WidgetRuntime>,
    show_seconds: bool,
    show_date: bool,
    size: u8,
    language: String,
    theme: Theme,
    privacy_mode: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let show_seconds = config.clock.show_seconds;
        let show_date = config.clock.show_date;
        let size = config.clock.size.clamp(1, 3);
        let language = config.clock.language.clone();
        let theme = Theme::named(&config.clock.color);
        let privacy_mode = config.security.privacy_mode;
        let security = config.security.clone();
        let widgets = config
            .widgets
            .iter()
            .map(|w| WidgetRuntime::from_config(w, &security))
            .collect();
        let mut app = Self {
            widgets,
            show_seconds,
            show_date,
            size,
            language,
            theme,
            privacy_mode,
        };
        app.refresh_widgets(true);
        app
    }

    pub fn toggle_seconds(&mut self) {
        self.show_seconds = !self.show_seconds;
    }

    pub fn toggle_date(&mut self) {
        self.show_date = !self.show_date;
    }

    pub fn grow(&mut self) {
        self.size = (self.size + 1).min(3);
    }

    pub fn shrink(&mut self) {
        self.size = (self.size.saturating_sub(1)).max(1);
    }

    pub fn refresh_widgets(&mut self, force: bool) {
        for w in &mut self.widgets {
            if force || w.needs_refresh() {
                w.refresh();
            }
        }
    }

    pub fn tick(&mut self) {
        for widget in &mut self.widgets {
            widget.poll_result();
        }
        self.refresh_widgets(false);
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(self.clock_block_height()),
                Constraint::Min(6),
                Constraint::Length(1),
            ])
            .split(area);

        self.draw_header(frame, chunks[0]);
        self.draw_clock(frame, chunks[1]);
        self.draw_widgets(frame, chunks[2]);
        self.draw_footer(frame, chunks[3]);
    }

    fn clock_block_height(&self) -> u16 {
        // bricks font: 5 rows × scale (+ date + borders/padding)
        let digit_h = clock_digits::digit_height(self.size);
        let date_h = if self.show_date { 2 } else { 0 };
        digit_h + date_h + 4
    }

    fn draw_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = &self.theme;
        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                " elzo-clock ",
                Style::default()
                    .fg(t.brand_fg)
                    .bg(t.brand_bg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "eclock",
                Style::default()
                    .fg(t.subtitle)
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::raw("  ·  "),
            Span::styled(
                format!("tema {}", t.name),
                Style::default().fg(t.widget_title),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(t.header_border)),
        );
        frame.render_widget(title, area);
    }

    fn draw_clock(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = &self.theme;
        let now = Local::now();
        let fmt = if self.show_seconds {
            "%H:%M:%S"
        } else {
            "%H:%M"
        };
        let time = now.format(fmt).to_string();
        let lines = clock_digits::render_time(&time, self.size);
        let mut text_lines: Vec<Line> = lines
            .into_iter()
            .map(|l| {
                Line::from(Span::styled(
                    l,
                    Style::default().fg(t.clock).add_modifier(Modifier::BOLD),
                ))
            })
            .collect();

        if self.show_date {
            let date = format_date(&now, &self.language);
            text_lines.push(Line::from(""));
            text_lines.push(Line::from(Span::styled(
                format!("  {date}"),
                Style::default().fg(t.date),
            )));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(" relógio ", Style::default().fg(t.clock)))
            .border_style(Style::default().fg(t.clock_border));
        let para = Paragraph::new(text_lines)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(para, area);
    }

    fn draw_widgets(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = &self.theme;
        if self.widgets.is_empty() {
            let p = Paragraph::new("Nenhum widget configurado em ~/.config/elzo-clock/config.toml")
                .style(Style::default().fg(t.widget_body))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled(
                            " widgets ",
                            Style::default().fg(t.widget_title),
                        ))
                        .border_style(Style::default().fg(t.widget_border)),
                );
            frame.render_widget(p, area);
            return;
        }

        let n = self.widgets.len().min(3);
        let constraints = widget_constraints(&self.widgets[..n]);
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for (i, w) in self.widgets.iter().take(n).enumerate() {
            let title = format!(" ● {} · {} ", w.title, w.running_hint);
            let block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(t.widget_title)
                        .add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(t.widget_border));
            let lines = colorize_widget_body(
                &w.output,
                t,
                is_calendar_widget(&w.title),
                Local::now().date_naive(),
                self.privacy_mode,
            );
            let para = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });
            frame.render_widget(para, cols[i]);
        }
    }

    fn draw_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = &self.theme;
        let help = Paragraph::new(Line::from(vec![
            Span::styled("q", Style::default().fg(t.footer_key)),
            Span::styled(" sair  ", Style::default().fg(t.footer)),
            Span::styled("r", Style::default().fg(t.footer_key)),
            Span::styled(" refresh  ", Style::default().fg(t.footer)),
            Span::styled("s", Style::default().fg(t.footer_key)),
            Span::styled(" segundos  ", Style::default().fg(t.footer)),
            Span::styled("d", Style::default().fg(t.footer_key)),
            Span::styled(" data  ", Style::default().fg(t.footer)),
            Span::styled("+/-", Style::default().fg(t.footer_key)),
            Span::styled(" tamanho  ", Style::default().fg(t.footer)),
            Span::styled(
                format!("config: {}", Config::config_path().display()),
                Style::default().fg(t.subtitle),
            ),
        ]));
        frame.render_widget(help, area);
    }
}

fn widget_constraints(widgets: &[WidgetRuntime]) -> Vec<Constraint> {
    if widgets.len() == 3 && widgets.iter().any(|widget| is_esaa_widget(&widget.title)) {
        return widgets
            .iter()
            .map(|widget| {
                if is_esaa_widget(&widget.title) {
                    Constraint::Ratio(2, 4)
                } else {
                    Constraint::Ratio(1, 4)
                }
            })
            .collect();
    }
    let count = widgets.len().max(1) as u32;
    widgets
        .iter()
        .map(|_| Constraint::Ratio(1, count))
        .collect()
}

fn is_esaa_widget(title: &str) -> bool {
    title.trim().eq_ignore_ascii_case("ESAA")
}

fn format_date(now: &DateTime<Local>, language: &str) -> String {
    match language.to_ascii_lowercase().as_str() {
        "en" | "en-us" | "en-gb" => now.format("%A, %d %B %Y").to_string(),
        _ => {
            const WEEKDAYS: [&str; 7] = [
                "segunda-feira",
                "terça-feira",
                "quarta-feira",
                "quinta-feira",
                "sexta-feira",
                "sábado",
                "domingo",
            ];
            const MONTHS: [&str; 12] = [
                "janeiro",
                "fevereiro",
                "março",
                "abril",
                "maio",
                "junho",
                "julho",
                "agosto",
                "setembro",
                "outubro",
                "novembro",
                "dezembro",
            ];
            let weekday = WEEKDAYS[now.weekday().num_days_from_monday() as usize];
            let month = MONTHS[now.month0() as usize];
            format!("{weekday}, {:02} de {month} de {}", now.day(), now.year())
        }
    }
}

/// Colorir linhas do widget no espírito do painel do tclock (títulos, datas, status).
fn colorize_widget_body(
    output: &str,
    t: &Theme,
    is_calendar: bool,
    today: NaiveDate,
    privacy_mode: bool,
) -> Vec<Line<'static>> {
    output
        .lines()
        .map(|raw| {
            let line = crate::sanitize::strip_control_chars(raw);
            let line = if privacy_mode && is_calendar {
                mask_agenda_line_for_privacy(&line)
            } else {
                line
            };
            let lower = line.to_lowercase();
            let style = if is_calendar && is_today_agenda_line(&line, today) {
                Style::default()
                    .fg(t.accent_today)
                    .add_modifier(Modifier::BOLD)
            } else if lower.contains("verify: ok")
                || lower.contains("ready")
                || lower.starts_with("verify: ok")
            {
                Style::default().fg(t.accent_ok)
            } else if lower.contains("verify:") && !lower.contains("ok")
                || lower.contains("warn")
                || lower.contains("fail")
                || lower.contains("mismatch")
            {
                Style::default().fg(t.widget_title)
            } else if lower.starts_with("pr ")
                || lower.contains(" pr #")
                || lower.starts_with("iss ")
                || lower.contains("issue")
            {
                Style::default().fg(t.accent_b)
            } else if looks_like_date_line(&line) {
                Style::default().fg(t.accent_a)
            } else if lower.starts_with("root:")
                || lower.starts_with("cli:")
                || lower.starts_with("seq:")
                || lower.starts_with("eligible:")
                || lower.starts_with("profile:")
                || lower.starts_with("próximas:")
                || lower.starts_with("proximas:")
                || lower.starts_with("suprimidas:")
            {
                Style::default()
                    .fg(t.widget_title)
                    .add_modifier(Modifier::BOLD)
            } else if line.trim_start().starts_with('·') || line.trim_start().starts_with('-') {
                Style::default().fg(t.accent_a)
            } else {
                Style::default().fg(t.widget_body)
            };
            Line::from(Span::styled(line, style))
        })
        .collect()
}

/// Keep times when present; mask event titles under privacy_mode.
fn mask_agenda_line_for_privacy(line: &str) -> String {
    let trimmed = line.trim_end();
    if trimmed.is_empty() {
        return line.to_string();
    }
    if looks_like_date_line(trimmed) && !trimmed.contains(':') {
        return line.to_string();
    }
    let t = trimmed.trim_start();
    if t.len() >= 5 && t.as_bytes().get(2) == Some(&b':') {
        let time = &t[..5];
        if time.chars().all(|c| c.is_ascii_digit() || c == ':') {
            return format!("{time} •••");
        }
    }
    if t.to_ascii_lowercase().starts_with("all-day") {
        return "all-day •••".into();
    }
    "•••".into()
}

fn is_calendar_widget(title: &str) -> bool {
    let lower = title.to_lowercase();
    lower.contains("agenda") || lower.contains("calendar") || lower.contains("calendário")
}

fn is_today_agenda_line(line: &str, today: NaiveDate) -> bool {
    let clean = crate::sanitize::strip_control_chars(line);
    let value = clean.trim_start();
    let english = today.format("%a %b %d").to_string();
    let iso = today.format("%Y-%m-%d").to_string();
    value.starts_with(&english) || value.starts_with(&iso)
}

fn looks_like_date_line(line: &str) -> bool {
    let s = line.trim();
    // "Mon Jul 13", "Sun Jul 12", "Tue Jul 14", "all-day", "11:00", etc.
    let days = [
        "mon", "tue", "wed", "thu", "fri", "sat", "sun", "seg", "ter", "qua", "qui", "sex", "sáb",
        "sab", "dom",
    ];
    let lower = s.to_lowercase();
    if days.iter().any(|d| lower.starts_with(d)) {
        return true;
    }
    if lower.contains("all-day") || lower.contains("all day") {
        return true;
    }
    // HH:MM at start
    if s.len() >= 5 {
        let b = s.as_bytes();
        if b[0].is_ascii_digit()
            && b[1].is_ascii_digit()
            && b[2] == b':'
            && b[3].is_ascii_digit()
            && b[4].is_ascii_digit()
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn formats_date_in_portuguese_and_english() {
        let date = Local
            .with_ymd_and_hms(2026, 7, 11, 12, 0, 0)
            .single()
            .expect("valid local date");
        assert_eq!(format_date(&date, "pt-BR"), "sábado, 11 de julho de 2026");
        assert_eq!(format_date(&date, "en"), "Saturday, 11 July 2026");
    }

    #[test]
    fn invalid_language_falls_back_to_portuguese() {
        let date = Local
            .with_ymd_and_hms(2026, 7, 11, 12, 0, 0)
            .single()
            .expect("valid local date");
        assert_eq!(
            format_date(&date, "xx-invalid"),
            "sábado, 11 de julho de 2026"
        );
    }

    #[test]
    fn detects_only_today_in_real_calendar_formats() {
        let today = NaiveDate::from_ymd_opt(2026, 7, 11).expect("valid date");
        assert!(is_today_agenda_line("Sat Jul 11  12:00 Reunião", today));
        assert!(is_today_agenda_line(
            "\u{1b}[0;33mSat Jul 11\u{1b}[0m  12:00 Reunião",
            today
        ));
        assert!(is_today_agenda_line("2026-07-11  12:00 Reunião", today));
        assert!(!is_today_agenda_line("Fri Jul 10  12:00 Ontem", today));
        assert!(!is_today_agenda_line("Sun Jul 12  12:00 Amanhã", today));
    }

    #[test]
    fn today_is_red_only_for_calendar_widgets() {
        let theme = Theme::akita();
        let today = NaiveDate::from_ymd_opt(2026, 7, 11).expect("valid date");
        let agenda = colorize_widget_body("Sat Jul 11", &theme, true, today, false);
        let esaa = colorize_widget_body("Sat Jul 11", &theme, false, today, false);
        assert_eq!(agenda[0].spans[0].style.fg, Some(theme.accent_today));
        assert_eq!(esaa[0].spans[0].style.fg, Some(theme.accent_a));
    }

    #[test]
    fn calendar_widget_names_are_recognized() {
        assert!(is_calendar_widget("Agenda Google"));
        assert!(is_calendar_widget("Google Calendar"));
        assert!(!is_calendar_widget("ESAA"));
    }

    fn widget(title: &str) -> WidgetRuntime {
        WidgetRuntime::from_config(
            &crate::config::WidgetConfig {
                title: Some(title.into()),
                command_argv: vec!["printf".into(), "ok".into()],
                command: None,
                refresh_secs: 10,
                timeout_secs: 1,
            },
            &crate::config::SecurityConfig::default(),
        )
    }

    #[test]
    fn esaa_gets_half_width_without_changing_order() {
        for titles in [
            ["ESAA", "Agenda", "Sistema"],
            ["Agenda", "ESAA", "Sistema"],
            ["Agenda", "Sistema", "ESAA"],
        ] {
            let widgets: Vec<_> = titles.iter().map(|title| widget(title)).collect();
            let constraints = widget_constraints(&widgets);
            for (index, title) in titles.iter().enumerate() {
                let expected = if *title == "ESAA" {
                    Constraint::Ratio(2, 4)
                } else {
                    Constraint::Ratio(1, 4)
                };
                assert_eq!(constraints[index], expected);
            }
        }
    }

    #[test]
    fn widgets_without_esaa_keep_equal_widths() {
        let widgets = vec![widget("GitHub"), widget("Agenda"), widget("Sistema")];
        assert_eq!(
            widget_constraints(&widgets),
            vec![Constraint::Ratio(1, 3); 3]
        );
    }
}
