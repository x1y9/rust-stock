use std::io::Stdout;

use tui::backend::CrosstermBackend;

pub mod events;
pub mod widget;

pub type DynResult = Result<(), Box<dyn std::error::Error>>;
pub type CrossTerminal = tui::Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

pub struct Stock {
    pub title: String,
    pub code: String,
    pub price: f32
}

impl Stock {
    pub fn new(code:String) ->Self {
        Self {
            code,
            title:String::from(""),
            price:0.0
        }
    }
}

pub enum AppState {
    /// Browsing quests
    Normal,
    /// Adding a new quest
    Adding,
}
pub struct App {
    pub should_exit:bool,
    pub state:AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_exit: false,
            state:AppState::Normal,
        }
    }
}