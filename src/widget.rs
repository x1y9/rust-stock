use tui::{layout::{Rect, Layout, Direction, Constraint, Alignment}, widgets::{Paragraph, Block, Borders, BorderType}, style::{Style, Color}};

use crate::{App};

pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(parent[1]);
    
    vec!(parent[0], center[0], center[1], parent[2])    
}

pub fn stock_list(_app: &App) -> Paragraph {
    Paragraph::new("Stocks")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightCyan))
    .block(Block::default().title("stocks")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain))
}

pub fn stock_detail(_app: &App) -> Paragraph {
    Paragraph::new("price")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightCyan))
    .block(Block::default().title("info")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain))
}

pub fn title_bar(_app: &App) -> Paragraph {
    Paragraph::new("Stock 1.0")
    .alignment(Alignment::Center)
}

pub fn status_bar(_app: &App) -> Paragraph {
    Paragraph::new("Quit[Q] | New[N]")
    .alignment(Alignment::Left)
}