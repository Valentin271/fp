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
        (KeyModifiers::CONTROL, KeyCode::Char('u')) => app.clear_search(),
        // Toggle preview
        (KeyModifiers::ALT, KeyCode::Char('p')) => {
            app.preview = !app.preview;
        }
        // search input
        (_, KeyCode::Char(c)) => app.push_search(c),
        // search input del
        (_, KeyCode::Backspace) => app.pop_search(),
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
