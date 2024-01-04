use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{app::App, theme::THEME};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let searchbox = Paragraph::new(app.search.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(THEME.border)
            .title(
                Title::default()
                    .alignment(Alignment::Center)
                    .content(" Find projects ".reset()),
            ),
    );

    f.render_widget(searchbox, area);

    f.set_cursor(area.x + 1 + app.search.len() as u16, area.y + 1);
}
