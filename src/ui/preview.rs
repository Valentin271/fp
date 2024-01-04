use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{
    app::App,
    theme::{LS_COLORS, THEME},
};

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let files = if let Some(selected) = app.selected() {
        selected
            .sorted_files()
            .iter()
            .map(|e| {
                if let Some(style) = LS_COLORS.style_for_path(e.path()) {
                    ListItem::new(Span::styled(
                        e.file_name().to_str().unwrap().to_owned(),
                        style.to_crossterm_style().into(),
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
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(THEME.border)
            .title(
                Title::default()
                    .alignment(Alignment::Center)
                    .content(" Files ".reset()),
            ),
    );

    f.render_widget(preview, area);
}
