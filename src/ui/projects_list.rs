use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{app::App, theme::theme};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let projects = List::new(app.filtered_projects.clone())
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(theme().border)
                .title(
                    Title::default()
                        .alignment(Alignment::Center)
                        .content(format!(" Results in {}ms ", app.start_time.as_millis()).reset()),
                ),
        )
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(theme().selected)
        .direction(ListDirection::BottomToTop);

    f.render_stateful_widget(projects, area, &mut app.list_state);
}
