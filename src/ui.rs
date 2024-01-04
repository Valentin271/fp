use ratatui::prelude::*;

use crate::app::App;

mod preview;
mod projects_list;
mod searchbar;

/// Renders the user interface widgets.
pub fn render(f: &mut Frame, app: &mut App) {
    let show_preview = app.preview && f.size().width > 100;
    let projects_pane_width = if show_preview { 70 } else { 100 };

    let panes = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(projects_pane_width),
            Constraint::Min(0),
        ],
    )
    .split(f.size());

    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Min(3), Constraint::Length(3)],
    )
    .split(panes[0]);

    projects_list::render(f, app, chunks[0]);
    searchbar::render(f, app, chunks[1]);

    if show_preview {
        preview::render(f, app, panes[1]);
    }
}
