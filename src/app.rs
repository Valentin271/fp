use std::{error, time::Duration};

use ratatui::widgets::ListState;
use strsim::normalized_damerau_levenshtein;

use crate::project::Project;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Value above which project are considered relevant enough
const MIN_SCORE: f64 = 200.;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// The time the app took to startup
    pub start_time: Duration,
    /// Search string
    pub search: String,
    /// Whether the preview is enabled.
    ///
    /// Note that even if `true`, preview might be hidden if there is not enough space.
    pub preview: bool,
    /// List of projects
    pub projects: Vec<Project>,
    /// List of projects filtered
    pub filtered_projects: Vec<Project>,
    /// UI list state
    pub list_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            start_time: Duration::default(),
            search: String::new(),
            preview: true,
            projects: Vec::new(),
            filtered_projects: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl App {
    /// Creates a new app from a list of project
    pub fn new<I>(projects: I) -> Self
    where
        I: IntoIterator<Item = Project>,
    {
        Self {
            projects: projects.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn up(&mut self) {
        let mut new = self.list_state.selected().unwrap_or(0) + 1;

        if new >= self.filtered_projects.len() {
            new = 0;
        }

        self.list_state.select(Some(new))
    }

    pub fn down(&mut self) {
        let new = if let Some(n) = self.list_state.selected().unwrap_or(0).checked_sub(1) {
            n
        } else {
            self.filtered_projects.len() - 1
        };

        self.list_state.select(Some(new));
    }

    pub fn filter_projects(&mut self) {
        let mut tmp: Vec<(&Project, i32)> = self
            .projects
            .iter()
            .filter_map(|p| {
                let score = normalized_damerau_levenshtein(
                    p.path.file_name().unwrap().to_str().unwrap(),
                    &self.search,
                ) * -1000.;

                if score < -MIN_SCORE {
                    Some((p, score as i32))
                } else {
                    None
                }
            })
            .collect();

        tmp.sort_unstable_by_key(|(_, s)| *s);
        self.filtered_projects = tmp.into_iter().map(|(p, _)| p.clone()).collect();
    }

    /// Returns the currently selected project
    pub fn selected(&mut self) -> Option<&Project> {
        let selected = self.list_state.selected().unwrap_or(0);
        self.filtered_projects.get(selected)
    }
}
