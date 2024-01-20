use argh::FromArgs;

use crate::theme::ThemeVariant;

/// Find Project.
///
/// A TUI to find projects and navigate to them.
#[derive(FromArgs)]
pub struct Cli {
    /// theme to use, can be "light" or "dark" (default)
    #[argh(option, default = "ThemeVariant::default()")]
    pub theme: ThemeVariant,
}
