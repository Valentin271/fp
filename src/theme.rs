use lscolors::LsColors;
use once_cell::sync::Lazy;
use ratatui::prelude::*;

pub const THEME: Theme = Theme::enfocado();

pub static LS_COLORS: Lazy<LsColors> = Lazy::new(|| LsColors::from_env().unwrap_or_default());

pub struct Theme {
    pub border: Style,
    pub selected: Style,
    pub background: Style,
}

impl Theme {
    /// Creates a dark theme from <https://github.com/wuelnerdotexe/vim-enfocado>
    pub const fn enfocado() -> Self {
        Self {
            border: Style::new().fg(Color::Rgb(131, 199, 70)),
            selected: Style::new().bg(Color::Rgb(59, 59, 59)),
            background: Style::new().bg(Color::Rgb(24, 24, 24)),
        }
    }
}
