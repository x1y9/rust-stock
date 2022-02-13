
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
            else if code == KeyCode::Char('d') {
                //删除当前选中的stock
                if app.stocks_state.selected().is_some() {
                    app.stocks.remove(sel);
                    app.save_stocks().unwrap();
                    app.stocks_state.select(None);
                }
            }
            else if code == KeyCode::Char('n') {
                //新建stock
                app.state = AppState::Adding;
                app.input = String::new();
            }
            else if code == KeyCode::Up && total > 0 {
                //注意这里如果不加判断直接用sel - 1, 在sel为0时会导致异常
                app.stocks_state.select(Some(if sel > 0 {sel - 1} else {0}));
            }
            else if code == KeyCode::Down && total > 0 {
                app.stocks_state.select(Some(if sel < total - 1 {sel + 1} else {sel}));
            }
        },

        AppState::Adding => match code {
            KeyCode::Enter => {
                app.state = AppState::Normal;
            }
            KeyCode::Esc => {
                app.state = AppState::Normal;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        },
    }
}