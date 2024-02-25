use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::theme::theme;

#[derive(Default)]
pub struct Searchbar {
    /// Search string
    search: String,
}

impl Searchbar {
    pub fn content(&self) -> &str {
        &self.search
    }

    pub fn push(&mut self, c: char) {
        self.search.push(c);
    }

    pub fn pop(&mut self) {
        self.search.pop();
    }

    pub fn clear(&mut self) {
        self.search.clear();
    }
}

impl Widget for &Searchbar {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let searchbox = Paragraph::new(self.search.clone()).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(theme().border)
                .title(
                    Title::default()
                        .alignment(Alignment::Center)
                        .content(" Find projects ".reset()),
                ),
        );

        searchbox.render(area, buf);

        // f.set_cursor(area.x + 1 + self.search.len() as u16, area.y + 1);
    }
}
