use std::{io::Stdout, fs};

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
    //pub slice: Vec<f64>
}

impl Stock {
    pub fn new(code:String) ->Self {
        Self {
            code,
            title:String::from(""),
            price:0.0,
            percent:0.0,
            //slice:vec![],
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
        //用unwrap_or_default屏蔽文件不存在时的异常
        let content = fs::read_to_string(dirs_next::home_dir().unwrap().join(DB_PATH)).unwrap_or_default();
        //如果直接转换stocks，必须所有key都对上, 兼容性不好 
        //self.stocks = serde_json::from_str(&content).unwrap_or_default();

        //先读成Vec<Value>再转换，可以增加兼容性，
        let json: Vec<Value> = serde_json::from_str(&content).unwrap_or_default();
        self.stocks = json.iter().map(|s| Stock::new(
            s.as_object().unwrap().get("code").unwrap().as_str().unwrap().to_string()))
            .collect();

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
                    stock.title = obj.get("name").unwrap_or(&json!("无效代码")).as_str().unwrap().to_owned();
                    stock.price = obj.get("price").unwrap_or(&json!(0.0)).as_f64().unwrap();
                    stock.percent = obj.get("percent").unwrap_or(&json!(0.0)).as_f64().unwrap();

                    // if json.contains_key(&stock.code) {
                    //     let mut writer2 = Vec::new();
                    //     request::get(format!("http://img1.money.126.net/data/hs/time/today/{}.json",stock.code), &mut writer2)?;
                    //     println!("{:?}", format!("http://img1.money.126.net/data/hs/time/today/{}.json",stock.code));  
                    //     let json2: Map<String, Value> = serde_json::from_str(&String::from_utf8_lossy(&writer2).to_string())?;
                    //     stock.slice = json2.get("data").unwrap().as_array().unwrap()
                    //         .iter().map(|item| item.as_array().unwrap().get(2).unwrap().as_f64().unwrap())
                    //         .collect();
                    // }
                }
            }
        }
        Ok(())
    }
}

