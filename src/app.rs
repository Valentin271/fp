use std::{error, time::Duration};

use ratatui::{prelude::*, widgets::*};
use state::AppState;

use crate::{
    project::Project,
    sorter::Sorter,
    ui::projects_list,
    widgets::{Preview, Searchbar},
};

mod state;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub state: AppState,
    /// The time the app took to startup
    pub start_time: Duration,
    /// List of projects
    pub projects: Vec<Project>,
    /// List of projects filtered
    pub filtered_projects: Vec<Project>,
    /// UI list state
    pub list_state: ListState,
    searchbar: Searchbar,
    preview: Preview,
    sorter: Sorter,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: Default::default(),
            start_time: Duration::default(),
            projects: Vec::new(),
            filtered_projects: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
            searchbar: Searchbar::default(),
            preview: Preview::default(),
            sorter: Sorter::default(),
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
        app.filtered_projects.clone_from(&app.projects);
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

    /// Closes the application
    pub fn quit(&mut self) {
        self.state = AppState::Stopped;
    }

    /// Go up one in the list.
    ///
    /// If at the top, go back to bottom.
    pub fn up(&mut self) {
        let mut new = self.list_state.selected().unwrap_or(0) + 1;

        if new >= self.filtered_projects.len() {
            new = 0;
        }

        self.list_state.select(Some(new));
        self.preview.select(self.selected().cloned());
    }

    /// Go down one in the list.
    ///
    /// If at the bottom, go back to the top.
    pub fn down(&mut self) {
        let new = self
            .list_state
            .selected()
            .unwrap_or(0)
            .checked_sub(1)
            .unwrap_or(self.filtered_projects.len() - 1);

        self.list_state.select(Some(new));
        self.preview.select(self.selected().cloned());
    }

    /// Filters the projects according to the search prompt.
    ///
    /// Should likely be called on every keystroke.
    pub fn filter_projects(&mut self) {
        self.filtered_projects = self.sorter.sort(&self.projects, self.searchbar.content());
        self.preview.select(self.selected().cloned())
    }

    /// Returns the currently selected project
    pub fn selected(&self) -> Option<&Project> {
        let selected = self.list_state.selected().unwrap_or(0);
        self.filtered_projects.get(selected)
    }

    /// Add one character to the current search.
    ///
    /// Also reset the currently selected item and triggers the list filtering.
    pub fn push_search(&mut self, c: char) {
        self.searchbar.push(c);
        self.list_state.select(Some(0));
        self.filter_projects();
    }

    /// Remove one character from the current search.
    ///
    /// Triggers a new list filtering.
    pub fn pop_search(&mut self) {
        self.searchbar.pop();
        if self.searchbar.content().is_empty() {
            self.filtered_projects.clone_from(&self.projects);
        } else {
            self.filter_projects();
        }
    }

    /// Reset the current search.
    pub fn clear_search(&mut self) {
        self.searchbar.clear();
        self.filtered_projects.clone_from(&self.projects);
    }

    pub fn toggle_preview(&mut self) {
        self.preview.toggle();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let show_preview = self.preview.is_visible() && area.width > 100;
        let projects_pane_width = if show_preview { 70 } else { 100 };

        let [projects_pane, preview_pane] = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(projects_pane_width),
                Constraint::Min(0),
            ],
        )
        .areas(area);

        let [project_chunk, searchbar_chunk] = Layout::new(
            Direction::Vertical,
            [Constraint::Min(3), Constraint::Length(3)],
        )
        .areas(projects_pane);

        projects_list::render(project_chunk, buf, self);
        self.searchbar.render(searchbar_chunk, buf);

        if show_preview {
            self.preview.render(preview_pane, buf)
        }
    }
}
