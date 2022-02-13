use std::{io::Stdout, fs, path::Path};

use serde::{Serialize, Deserialize};
use tui::{backend::CrosstermBackend, widgets::ListState};

pub mod events;
pub mod widget;

pub type DynResult = Result<(), Box<dyn std::error::Error>>;
pub type CrossTerminal = tui::Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

pub const DB_PATH: &str="stocks.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub input:String,
    pub stocks:Vec<Stock>,
    //记录了当前选中和滚动位置两个状态
    pub stocks_state:ListState,
}

impl App {
    //这里传入参数声明为&[Stock], &Vec<Stock>似乎都可以
    pub fn new() -> Self {

        //ListState:default为未选择, 如果需要也可以初始化为0
        //let mut sel = ListState::default();
        //sel.select(Option::Some(0));

        Self {
            should_exit: false,
            state: AppState::Normal,
            input: String::new(),
            stocks: [].to_vec(),
            stocks_state: ListState::default(),
        }
    }

    pub fn save_stocks(&self) -> DynResult{
        fs::write(DB_PATH, serde_json::to_string(&self.stocks)?)?;
        Ok(())
    }

    pub fn load_stocks(&mut self) -> DynResult{
        //必须有,否则db不存在时报FileNotFound异常
        if !Path::new(DB_PATH).exists() {
            fs::File::create(DB_PATH)?;
        }

        let content = fs::read_to_string(DB_PATH)?;
        //必须所有key都对上,否则异常,用unwrap_or_default来屏蔽异常
        self.stocks = serde_json::from_str(&content).unwrap_or_default();

        Ok(())
    }
}

