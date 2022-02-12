use crossterm::event::{KeyEvent, KeyCode};

use crate::{App, AppState};

pub fn on_events(event:KeyEvent, app:&mut App) {
    let code = event.code;
    match app.state {
        AppState::Normal => {
            if code == KeyCode::Char('q') {
                app.should_exit = true;
            }
            else if code == KeyCode::Down {
                if let Some(sel) = app.stocks_state.selected() {
                    if sel < app.stocks.len() - 1 {
                        app.stocks_state.select(Some(sel + 1));
                    }
                }
            }
            else if code == KeyCode::Up {
                if let Some(sel) = app.stocks_state.selected() {
                    if sel > 0 {
                        app.stocks_state.select(Some(sel - 1));
                    }
                }
            }
        },

        AppState::Adding => {
        }
    }
}