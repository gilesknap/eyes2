// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{io, thread, time::Duration};
// use tui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders, Widget},
//     Terminal,
// };

// struct Tui {
//     terminal: Terminal<CrosstermBackend<io::Stdout>>,
// }

// impl Tui {
//     fn new() -> Tui {
//         let backend = CrosstermBackend::new(io::stdout());
//         Tui {
//             terminal: Terminal::new(backend).unwrap(),
//         }
//     }

//     fn draw(&mut self) -> Result<(), io::Error> {
//         // setup terminal
//         enable_raw_mode()?;
//         let mut stdout = io::stdout();
//         execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

//         self.terminal.draw(|f| {
//             let size = f.size();
//             let block = Block::default().title("Block").borders(Borders::ALL);
//             f.render_widget(block, size);
//         })?;

//         thread::sleep(Duration::from_millis(5000));

//         // restore terminal
//         disable_raw_mode()?;
//         execute!(
//             self.terminal.backend_mut(),
//             LeaveAlternateScreen,
//             DisableMouseCapture
//         )?;
//         self.terminal.show_cursor()?;

//         Ok(())
//     }
// }
