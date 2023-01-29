use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
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

    pub fn draw(&mut self) -> Result<(), io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(f.size());
            let block = Block::default().title("World").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Control").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        thread::sleep(Duration::from_millis(5000));

        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
