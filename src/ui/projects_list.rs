use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{app::App, theme::theme};

pub fn render(area: Rect, buf: &mut Buffer, app: &mut App) {
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

    StatefulWidget::render(projects, area, buf, &mut app.list_state);
}
