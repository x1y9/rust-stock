use std::{io::Stdout, fs, path::Path};

use http_req::request;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map, json};
use tui::{backend::CrosstermBackend, widgets::ListState};

pub mod events;
pub mod widget;

pub type DynResult = Result<(), Box<dyn std::error::Error>>;
pub type CrossTerminal = tui::Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

pub const DB_PATH: &str=".stocks.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stock {
    pub title: String,
    pub code: String,
    pub price: f64,
    pub percent: f64,
}

impl Stock {
    pub fn new(code:String) ->Self {
        Self {
            code,
            title:String::from(""),
            price:0.0,
            percent:0.0
        }
    }
}

pub enum AppState {
    Normal,
    Adding,
}
pub struct App {
    pub should_exit:bool,
    pub state:AppState,
    pub error:String,
    pub input:String,
    pub stocks:Vec<Stock>,
    //TUI的List控件需要这个state记录当前选中和滚动位置两个状态
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
            error: String::new(),
            stocks: [].to_vec(),
            stocks_state: ListState::default(),
        }
    }

    pub fn save_stocks(&self) -> DynResult{
        let db=dirs_next::home_dir().unwrap().join(DB_PATH);
        fs::write(&db, serde_json::to_string(&self.stocks)?)?;
        Ok(())
    }

    pub fn load_stocks(&mut self) -> DynResult{
        //必须有,否则db不存在时报FileNotFound异常
        let db=dirs_next::home_dir().unwrap().join(DB_PATH);
        if !Path::new(&db).exists() {
            fs::File::create(&db)?;
        }

        let content = fs::read_to_string(&db)?;
        //必须所有key都对上,否则异常,用unwrap_or_default来屏蔽异常
        self.stocks = serde_json::from_str(&content).unwrap_or_default();

        Ok(())
    }

    pub fn refresh_stocks(&mut self) -> DynResult{ 
        if self.stocks.len() > 0 {
            let codes: Vec<_> = self.stocks.iter()
                .map(|stock| stock.code.clone())
                .collect();
            let mut writer = Vec::new();
            request::get(format!("{}{}","http://api.money.126.net/data/feed/",codes.join(",")), &mut writer)?;
            let content = String::from_utf8_lossy(&writer);
            if content.starts_with("_ntes_quote_callback") {
                //网易的返回包了一个js call，用skip,take,collect实现一个substring剥掉它
                let json: Map<String, Value> = serde_json::from_str(&content.chars().skip(21).take(content.len() - 23).collect::<String>())?;
                for stock in &mut self.stocks {
                    //如果code不对,返回的json里不包括这个对象, 用unwrap_or生成一个空对象,防止异常
                    let obj = json.get(&stock.code).unwrap_or(&json!({})).as_object().unwrap().to_owned();
                    stock.title = obj.get("name").unwrap_or(&json!(stock.code)).as_str().unwrap().to_owned();
                    stock.price = obj.get("price").unwrap_or(&json!(0.0)).as_f64().unwrap();
                    stock.percent = obj.get("percent").unwrap_or(&json!(0.0)).as_f64().unwrap();
                }
            }
        }
        Ok(())
    }
}

