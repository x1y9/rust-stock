use tui::{layout::{Rect, Layout, Direction, Constraint, Alignment}, 
widgets::{Paragraph, Block, Borders, BorderType, List, ListItem}, 
style::{Style, Color, Modifier}, text::{Spans, Span}};

use crate::{App, Stock, AppState};

const VERSION:&str = env!("CARGO_PKG_VERSION");

//计算所有的屏幕窗口区域,供后续render使用
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ].as_ref())
        .split(area);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ].as_ref())
        .split(parent[1]);

    //计算新建stock时的弹框位置    
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Percentage(40),
        ].as_ref())
        .split(area); 

    let popline = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ].as_ref())
        .split(popup[1]);       
    
    vec!(parent[0], center[0], center[1], parent[2], popline[1])    
}

pub fn stock_list(stocks: &Vec<Stock>) -> List {
    let items: Vec<_> = stocks.iter()
        .map(|stock| {
            ListItem::new(Spans::from(vec![
                Span::styled(format!("{:+.2}% ",stock.percent * 100.0),
                    Style::default().fg(if stock.percent < 0.0 {Color::Green} else {Color::Red})),
                Span::styled(stock.title.clone(),Style::default()),
                ]))
        }).collect();

    List::new(items)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("列表")
            .border_type(BorderType::Plain))
        .highlight_style(
            Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD))
}

pub fn stock_detail(app: &App) -> Paragraph {
    let mut info = String::new();
    let sel = app.stocks_state.selected().unwrap_or(0);
    //这样要防止sel超出stocks范围
    if app.stocks_state.selected().is_some() && sel < app.stocks.len() {
        let stock = app.stocks.get(sel).unwrap();
        info = format!("代码:{}\n当前:{}\n涨跌:{:+.2}%", stock.code, stock.price.to_string(), stock.percent * 100.0);
    }

    Paragraph::new(info)
    .alignment(Alignment::Center)
    .style(Style::default())
    .block(Block::default().title("详情")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain))
}

pub fn stock_input(app: &App) -> Paragraph {
    Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("输入证券代码"))
}

pub fn title_bar(_app: &App) -> Paragraph {
    //可以在这里显示一些调试信息，比如stocks_state，但要每次render stocks之后才会刷新
    Paragraph::new(format!("Stock v{}", VERSION, ))
    .alignment(Alignment::Left)
}

pub fn status_bar(app: &mut App) -> Paragraph {
    let mut status = app.error.clone();
    if app.error.is_empty() {
        status = match app.state {
            AppState::Normal => "退出[Q] | 新建[N] | 删除[D] | 刷新[R] | 上移[U] | 下移[J]",
            AppState::Adding => "确认[Enter] | 取消[ESC] | 上交所代码前需要加0，深市加1"
        }.to_string();
    }
    else {
        app.error.clear();
    }
    Paragraph::new(status).alignment(Alignment::Left)
}
