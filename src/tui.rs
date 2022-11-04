use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend, 
    widgets::{Block, Table, Row, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn draw_terminal() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //terminal.draw(|f| {
    //    let size = f.size();
    //    let block = Block::default()
    //        .title("*Tim Allen Voice* HUHHHH?")
    //        .borders(Borders::ALL);
    //    f.render_widget(block, size);
    //})?;

    terminal.draw(|obj| {
        //let size = obj.size();
        let size = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(obj.size());

        let table = Table::new(vec![
            Row::new(vec!["1", "2", "3"]),
            Row::new(vec!["Apples", "Oranges", "Grapes"]).style(tui::style::Style::default().fg(tui::style::Color::Blue)),
        ])
        .header(
            Row::new(vec!["COLUMN 1", "COLUMN 2", "COLUMN 3"]).bottom_margin(2)
        )
        .block(Block::default().title("Table").borders(Borders::ALL))
        .widths(&[Constraint::Length(10), Constraint::Length(10), Constraint::Length(10)])
        .column_spacing(5)
        .highlight_style(tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD))
        .highlight_symbol(">>");
        obj.render_widget(table, size[1]);
    })?;

    thread::sleep(Duration::from_millis(2000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}