use std::{error::Error, time::{Instant, Duration}};

use stock::{DynResult, CrossTerminal, App, TerminalFrame, events, widget, AppState};
use tui::{Terminal, backend::CrosstermBackend, widgets};
use unicode_width::UnicodeWidthStr;


fn main() -> DynResult{
    let mut app = App::new();
    let mut terminal = init_terminal()?;
    main_loop(&mut terminal, &mut app)?;
    close_terminal(terminal)?;

    Ok(())
}

fn init_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    //必须先执行EnableMouseCapture后面才能支持鼠标事件
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn close_terminal(mut terminal: CrossTerminal) -> DynResult{
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::event::DisableMouseCapture, crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

//主事件循环
fn main_loop(terminal: &mut CrossTerminal, app: &mut App) -> DynResult {
    let mut last_tick = Instant::now();
    while !app.should_exit {
        terminal.draw(|f| {on_draw(f, app);})?;

        if crossterm::event::poll(Duration::from_secs(1).checked_sub(last_tick.elapsed()).unwrap_or_default())? {
            events::on_events(crossterm::event::read()?, app);
        }
        else {
            events::on_tick(app);
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn on_draw(frame: &mut TerminalFrame, app: &mut App) {
    let chunks = widget::main_chunks(frame.size());

    //list的render需要调render_stateful_widget,否则滚动状态不对,这里第一个参数不能是app,否则会和后面的mut stock_state冲突
    frame.render_stateful_widget(widget::stock_list(&app.stocks.lock().unwrap()), chunks[1], &mut app.stocks_state);
    //因为render stock_list时会修改滚动状态，后面如果要用到这个值，就需要先做list的render
    frame.render_widget(widget::title_bar(app, frame.size()), chunks[0]);
    frame.render_widget(widget::stock_detail(app), chunks[2]);
    frame.render_widget(widget::status_bar(app), chunks[3]);

    if let AppState::Adding = app.state {
        //popup需要先clear一下,否则下面的背景色会透上来
        frame.render_widget(widgets::Clear, chunks[4]);
        frame.render_widget(widget::stock_input(app), chunks[4]);
        
        //显示光标, width()接口依赖一个外部包,可以正确处理中文宽度
        frame.set_cursor(chunks[4].x + app.input.width() as u16 + 1, chunks[4].y + 1);
    }
    
}