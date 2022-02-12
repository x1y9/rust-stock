use std::{error::Error};

use crossterm::event::Event;
use stock::{DynResult, CrossTerminal, App, TerminalFrame, events::on_events, widget};
use tui::{Terminal, backend::CrosstermBackend};


fn main() -> DynResult{
    //Log和SimpleLogger在TUI应用里意义不大,看不到
    //SimpleLogger::new().init()?;
    
    let mut app = App::new();
    let mut terminal = init_terminal()?;
    
    main_loop(&mut terminal, &mut app)?;
    close_terminal(terminal)?;

    Ok(())
}

fn init_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn close_terminal(mut terminal: CrossTerminal) -> DynResult{
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

//主事件循环
fn main_loop(terminal: &mut CrossTerminal, app: &mut App) -> DynResult {
    app.load_stocks()?;
    while !app.should_exit {
        terminal.draw(|f| {
            on_draw(f, app);
        })?;

        //read是block的,如果要非block,可以考虑poll
        if let Event::Key(event) = crossterm::event::read()? {
            on_events(event, app);
        }
    }

    Ok(())
}

fn on_draw(frame: &mut TerminalFrame, app: &mut App) {
    let chunks = widget::main_chunks(frame.size());

    frame.render_widget(widget::title_bar(app), chunks[0]);
    //list需要render_stateful_widget,否则滚动状态不对,这里第一个参数不能是app,否则会和后面的mut stock_state冲突
    frame.render_stateful_widget(widget::stock_list(&app.stocks), chunks[1], &mut app.stocks_state);
    frame.render_widget(widget::stock_detail(app), chunks[2]);
    frame.render_widget(widget::status_bar(app), chunks[3]);

    //如果有输入控件,处理光标
    //frame.set_cursor(chunks[3].x, chunks[3].y)
}