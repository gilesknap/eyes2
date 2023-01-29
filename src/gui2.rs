use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};

pub struct EyesTui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl EyesTui {
    pub fn new() -> EyesTui {
        let backend = CrosstermBackend::new(io::stdout());
        EyesTui {
            terminal: Terminal::new(backend).unwrap(),
        }
    }

    fn data_table() -> Table<'static> {
        Table::new(vec![
            // Row can be created from simple strings.
            Row::new(vec!["Row11", "Row12", "Row13"]),
            // You can style the entire row.
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
            // If you need more control over the styling you may need to create Cells directly
            Row::new(vec![
                Cell::from("Row31"),
                Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                Cell::from(Spans::from(vec![
                    Span::raw("Row"),
                    Span::styled("33", Style::default().fg(Color::Green)),
                ])),
            ]),
            // If a Row need to display some content over multiple lines, you just have to change
            // its height.
            Row::new(vec![
                Cell::from("Row\n41"),
                Cell::from("Row\n42"),
                Cell::from("Row\n43"),
            ])
            .height(2),
        ])
    }

    pub fn draw(&mut self) -> Result<(), io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[0]);
            let upper_left_pane = Block::default().title("BlockUL").borders(Borders::ALL);
            let lower_left_pane = Block::default().title("BlockLL").borders(Borders::ALL);
            let right_pane = Block::default().title("BlockR").borders(Borders::ALL);
            f.render_widget(upper_left_pane, left_chunks[0]);
            f.render_widget(lower_left_pane, left_chunks[1]);
            f.render_widget(right_pane, chunks[1]);
        })?;

        thread::sleep(Duration::from_millis(5000));
        Ok(())
    }
}

impl Drop for EyesTui {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().ok();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .ok();
        self.terminal.show_cursor().ok();
    }
}
