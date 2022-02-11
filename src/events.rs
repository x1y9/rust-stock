use crossterm::event::{KeyEvent, KeyCode};

use crate::{App, AppState};

pub fn on_events(event:KeyEvent, app:&mut App) {
    let code = event.code;
    match app.state {
        AppState::Normal => {
            if code == KeyCode::Char('q') {
                app.should_exit = true;
            }
        },
        AppState::Adding => {
        }
    }
}