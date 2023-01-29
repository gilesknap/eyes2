use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
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

    pub fn my_draw(&mut self) -> Result<(), io::Error> {
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
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);
            let left_pane = Block::default().title("World").borders(Borders::ALL);
            let upper_right_pane = Block::default().title("Status").borders(Borders::ALL);
            let lower_right_pane = Block::default().title("Settings").borders(Borders::ALL);
            f.render_widget(left_pane, chunks[0]);
            f.render_widget(upper_right_pane, right_chunks[0]);
            f.render_widget(lower_right_pane, right_chunks[1]);
        })?;
        Ok(())
    }

    pub fn render(&mut self) {
        for _ in 0..15 {
            self.my_draw();
            thread::sleep(Duration::from_millis(1000));
        }
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
