#[derive(Debug, Default)]
pub enum AppState {
    #[default]
    Running,
    Stopped,
}

impl AppState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}
