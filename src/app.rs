use std::{error, time::Duration};

use ratatui::widgets::ListState;
use state::AppState;
use strsim::normalized_damerau_levenshtein;

use crate::project::Project;

mod state;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Value above which project are considered relevant enough
const MIN_SCORE: f64 = 100.;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub state: AppState,
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
            state: Default::default(),
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
        let mut app = Self {
            projects: projects.into_iter().collect(),
            ..Default::default()
        };
        app.dedup();
        app.filtered_projects = app.projects.clone();
        app
    }

    /// Remove duplicates and subprojects
    fn dedup(&mut self) {
        self.projects.dedup();

        let projects = self.projects.clone();

        self.projects.retain(|p| {
            for p2 in &projects {
                if p.path.starts_with(&p2.path) && p != p2 {
                    return false;
                }
            }
            true
        });
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.state = AppState::Stopped;
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
                ) * 1000.;

                if score < MIN_SCORE {
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
