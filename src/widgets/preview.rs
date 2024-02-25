use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{
    project::Project,
    theme::{theme, LS_COLORS},
};

pub struct Preview {
    /// Whether the preview is enabled.
    ///
    /// Note that even if `true`, preview might be hidden if there is not enough space.
    visible: bool,
    selected: Option<Project>,
}

impl Preview {
    pub fn select(&mut self, project: Option<Project>) {
        self.selected = project
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            visible: true,
            selected: None,
        }
    }
}

impl Widget for &Preview {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let files = if let Some(selected) = &self.selected {
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
}
