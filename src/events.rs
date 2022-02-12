use crossterm::event::{KeyEvent, KeyCode};

use crate::{App, AppState};

pub fn on_events(event:KeyEvent, app:&mut App) {
    let code = event.code;
    let total = app.stocks.len(); 
    let sel = app.stocks_state.selected().unwrap_or(0);
    match app.state {
        AppState::Normal => {
            if code == KeyCode::Char('q') {
                app.should_exit = true;
            }
            else if code == KeyCode::Up && total > 0 {
                //注意这里如果不加判断直接用sel - 1, 在sel为0时会导致异常
                app.stocks_state.select(Some(if sel > 0 {sel - 1} else {0}));
            }
            else if code == KeyCode::Down && total > 0 {
                app.stocks_state.select(Some(if sel < total - 1 {sel + 1} else {sel}));
            }
        },

        AppState::Adding => {
        }
    }
}