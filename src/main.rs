use std::{error::Error};

use crossterm::event::Event;
use stock::{DynResult, CrossTerminal, App, TerminalFrame, events::on_events, widget};
use tui::{Terminal, backend::CrosstermBackend};

const DB_PATH: &str="db.json";

fn main() -> DynResult{
    let mut terminal = init_terminal()?;
    let mut app = App::new();

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
    while !app.should_exit {
        terminal.draw(|f| {
            on_draw(f, app);
        })?;

        //这里或许可以处理一下超时
        if let Event::Key(event) = crossterm::event::read()? {
            on_events(event, app);
        }
    }

    Ok(())
}

fn on_draw(frame: &mut TerminalFrame, app: &App) {
    let chunks = widget::main_chunks(frame.size());

    // let quest_list = widget::quest_list(app);
    // frame.render_widget(quest_list, main_chunks[0]);

    // let quest_input = widget::quest_input(app);
    // frame.render_widget(quest_input, main_chunks[1]);
    // handle_input_cursor(&app, frame, &main_chunks);

    frame.render_widget(widget::title_bar(app), chunks[0]);
    frame.render_widget(widget::stock_list(app), chunks[1]);
    frame.render_widget(widget::stock_detail(app), chunks[2]);
    frame.render_widget(widget::status_bar(app), chunks[3]);
}