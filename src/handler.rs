use std::env::current_dir;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.modifiers, key_event.code) {
        // Exit application on `Ctrl-C` or `ESC`
        (KeyModifiers::CONTROL, KeyCode::Char('c')) | (_, KeyCode::Esc) => {
            print!("{}", current_dir()?.display());
            app.quit();
        }
        // move up
        (KeyModifiers::CONTROL, KeyCode::Char('k')) => app.up(),
        // move down
        (KeyModifiers::CONTROL, KeyCode::Char('j')) => app.down(),
        // Clear search box
        (KeyModifiers::CONTROL, KeyCode::Char('u')) => {
            app.search.clear();
            app.filtered_projects = app.projects.clone();
        }
        // Toggle preview
        (KeyModifiers::ALT, KeyCode::Char('p')) => {
            app.preview = !app.preview;
        }
        // search input
        (_, KeyCode::Char(c)) => {
            app.search.push(c);
            app.list_state.select(Some(0));
            app.filter_projects();
        }
        // search input del
        (_, KeyCode::Backspace) => {
            let _ = app.search.pop();
            if app.search.is_empty() {
                app.filtered_projects = app.projects.clone();
            } else {
                app.filter_projects();
            }
        }
        // select project
        (_, KeyCode::Enter) => {
            if let Some(selected) = app.selected() {
                print!("{}", selected.path.display());
                app.quit()
            }
        }
        _ => {}
    }
    Ok(())
}
