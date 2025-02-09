use std::{cmp::Ordering, fmt::Display, fs::DirEntry, path::PathBuf};

use ratatui::{
    prelude::{style::Styled, Line},
    widgets::ListItem,
};

use crate::theme::theme;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    pub path: PathBuf,
}

impl Project {
    pub fn new(mut path: PathBuf) -> Self {
        path.pop();
        Self { path }
    }

    pub fn files(&self) -> Vec<DirEntry> {
        self.path
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .collect()
    }

    pub fn sorted_files(&self) -> Vec<DirEntry> {
        let mut files = self.files();

        files.sort_unstable_by(|a, b| {
            let adir = a.file_type().unwrap().is_dir();
            let bdir = b.file_type().unwrap().is_dir();

            if adir == bdir {
                let aname = a.file_name().to_string_lossy().to_lowercase();
                let aname = aname.trim_start_matches('.');
                let bname = b.file_name().to_string_lossy().to_lowercase();
                let bname = bname.trim_start_matches('.');

                aname.cmp(bname)
            } else if adir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        files
    }
}

impl From<Project> for ListItem<'_> {
    fn from(value: Project) -> Self {
        Self::from(Line::from(vec![
            value
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .into(),
            " ".into(),
            value
                .path
                .to_str()
                .unwrap()
                .to_string()
                .set_style(theme().project_path),
        ]))
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}
