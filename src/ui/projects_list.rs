use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{app::App, theme::THEME};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let projects = List::new(app.filtered_projects.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(THEME.border)
                .title(
                    Title::default()
                        .alignment(Alignment::Center)
                        .content(format!(" Results in {}ms ", app.start_time.as_millis()).reset()),
                ),
        )
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(THEME.selected)
        .direction(ListDirection::BottomToTop);

    f.render_stateful_widget(projects, area, &mut app.list_state);
}
