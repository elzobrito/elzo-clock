//! Paletas de cor do eclock.
//! "akita" / "tclock" imita o visual do screenshot do clock-tui (dígitos sage + painéis coloridos).

use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    /// Dígitos do relógio
    pub clock: Color,
    /// Borda do bloco do relógio
    pub clock_border: Color,
    /// Data sob o relógio
    pub date: Color,
    /// Fundo do badge do título
    pub brand_bg: Color,
    pub brand_fg: Color,
    pub subtitle: Color,
    /// Títulos dos widgets (ex.: "GitHub pending", "Google Calendar")
    pub widget_title: Color,
    pub widget_border: Color,
    pub widget_body: Color,
    /// Acentos secundários no corpo (links, horas)
    pub accent_a: Color,
    pub accent_b: Color,
    pub accent_ok: Color,
    /// Current day in calendar widgets.
    pub accent_today: Color,
    pub footer_key: Color,
    pub footer: Color,
    pub header_border: Color,
}

impl Theme {
    pub fn named(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "akita" | "tclock" | "sage" => Self::akita(),
            "cyan" => Self::simple(Color::Cyan, Color::Yellow),
            "green" => Self::simple(Color::Green, Color::Yellow),
            "magenta" | "pink" => Self::simple(Color::Magenta, Color::Cyan),
            "yellow" => Self::simple(Color::Yellow, Color::Magenta),
            "blue" => Self::simple(Color::Blue, Color::Yellow),
            "white" => Self::simple(Color::White, Color::Yellow),
            _ => Self::akita(),
        }
    }

    /// Cores próximas do screenshot do fork clock-tui (Akita).
    pub fn akita() -> Self {
        Self {
            name: "akita".into(),
            // dígitos verde-sage / olive suave
            clock: Color::Rgb(168, 184, 154),
            clock_border: Color::Rgb(90, 100, 88),
            date: Color::Rgb(150, 150, 145),
            brand_bg: Color::Rgb(168, 184, 154),
            brand_fg: Color::Rgb(20, 24, 28),
            subtitle: Color::Rgb(110, 115, 120),
            // títulos dos painéis em laranja/âmbar
            widget_title: Color::Rgb(232, 156, 74),
            widget_border: Color::Rgb(70, 78, 88),
            widget_body: Color::Rgb(210, 214, 220),
            // roxo/azul dos eventos e metas
            accent_a: Color::Rgb(130, 150, 230),
            accent_b: Color::Rgb(180, 140, 220),
            accent_ok: Color::Rgb(140, 200, 150),
            accent_today: Color::Rgb(235, 80, 80),
            footer_key: Color::Rgb(232, 156, 74),
            footer: Color::Rgb(100, 105, 110),
            header_border: Color::Rgb(55, 60, 68),
        }
    }

    fn simple(primary: Color, accent: Color) -> Self {
        Self {
            name: "simple".into(),
            clock: primary,
            clock_border: primary,
            date: Color::Gray,
            brand_bg: primary,
            brand_fg: Color::Black,
            subtitle: Color::DarkGray,
            widget_title: accent,
            widget_border: Color::DarkGray,
            widget_body: Color::White,
            accent_a: primary,
            accent_b: accent,
            accent_ok: Color::Green,
            accent_today: Color::Red,
            footer_key: accent,
            footer: Color::DarkGray,
            header_border: Color::DarkGray,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_all_documented_themes() {
        for name in [
            "akita", "tclock", "sage", "cyan", "green", "magenta", "pink", "yellow", "blue",
            "white",
        ] {
            let theme = Theme::named(name);
            assert!(!theme.name.is_empty());
        }
    }

    #[test]
    fn unknown_theme_falls_back_to_akita() {
        assert_eq!(Theme::named("desconhecido").name, "akita");
    }
}
