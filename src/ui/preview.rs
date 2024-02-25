use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{
    app::App,
    theme::{theme, LS_COLORS},
};

pub fn render(area: Rect, buf: &mut Buffer, app: &mut App) {
    let files = if let Some(selected) = app.selected() {
        selected
            .sorted_files()
            .iter()
            .map(|e| {
                if let Some(style) = LS_COLORS.style_for_path(e.path()) {
                    ListItem::new(Span::styled(
                        e.file_name().to_str().unwrap().to_owned(),
                        style.to_crossterm_style(),
                    ))
                } else {
                    ListItem::new(e.file_name().to_str().unwrap().to_owned())
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let preview = List::new(files).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(theme().border)
            .title(
                Title::default()
                    .alignment(Alignment::Center)
                    .content(" Files ".reset()),
            ),
    );

    Widget::render(preview, area, buf);
}
