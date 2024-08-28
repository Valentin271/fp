use std::sync::{LazyLock, OnceLock};

use lscolors::LsColors;
use ratatui::prelude::*;

mod variant;

pub use variant::*;

static THEME: OnceLock<Theme> = OnceLock::new();

pub static LS_COLORS: LazyLock<LsColors> = LazyLock::new(|| LsColors::from_env().unwrap_or_default());

#[derive(Clone, Copy)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub border: Style,
    pub selected: Style,
    pub project_path: Style,
}

impl Theme {
    /// Creates a dark theme from <https://github.com/wuelnerdotexe/vim-enfocado>
    pub const fn enfocado_dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            border: Style::new().fg(Color::Rgb(131, 199, 70)),
            selected: Style::new().bg(Color::Rgb(59, 59, 59)),
            project_path: Style::new().fg(Color::Rgb(119, 119, 119)),
        }
    }

    /// Creates a light theme from <https://github.com/wuelnerdotexe/vim-enfocado>
    pub const fn enfocado_light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            border: Style::new().fg(Color::Rgb(0, 132, 0)),
            selected: Style::new().bg(Color::Rgb(205, 205, 205)),
            project_path: Style::new().fg(Color::Rgb(135, 135, 135)),
        }
    }
}

pub fn init_theme(variant: ThemeVariant) {
    let theme = match variant {
        ThemeVariant::Light => Theme::enfocado_light(),
        _ => Theme::enfocado_dark(),
    };

    THEME.set(theme).unwrap_or_else(|_| {
        panic!(
            "Theme should not be set before calling {}",
            stringify!(init_theme)
        )
    });
}

pub fn theme() -> Theme {
    *THEME.get().expect("Theme should be set")
}
