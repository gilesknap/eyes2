use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io, thread,
    time::{Duration, Instant},
};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

use std::sync::mpsc;
enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    View,
    Monitor,
    Pause,
    Debug,
    Edit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::View => 0,
            MenuItem::Monitor => 1,
            MenuItem::Pause => 2,
            MenuItem::Debug => 3,
            MenuItem::Edit => 4,
        }
    }
}

pub struct EyesTui {
    stdout: io::Stdout,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    backend: CrosstermBackend<io::Stdout>,
    menu_titles: Vec<&'static str>,
    active_menu_item: MenuItem,
    menu_state: ListState,
    rx: mpsc::Receiver<Event<KeyCode>>,
}

impl EyesTui {
    pub fn new() -> EyesTui {
        let backend = CrosstermBackend::new(io::stdout());
        let (tx, rx) = mpsc::channel();

        let mut new = EyesTui {
            stdout: io::stdout(),
            backend: CrosstermBackend::new(io::stdout()),
            terminal: Terminal::new(backend).expect("can create terminal"),
            menu_titles: vec!["View", "Monitor", "Pause", "Debug", "Edit"],
            active_menu_item: MenuItem::View,
            menu_state: ListState::default(),
            rx,
        };

        new.terminal.clear().ok();
        new.menu_state.select(Some(0));

        enable_raw_mode().expect("can run in raw mode");

        let tick_rate = Duration::from_millis(200);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key.code)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        new
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let statusbar = Paragraph::new("Status: ")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            let menu = self
                .menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(self.active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match self.active_menu_item {
                MenuItem::View => rect.render_widget(EyesTui::render_home(), chunks[1]),
                MenuItem::Monitor => {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    // let (left, right) = EyesTui::render_monitor();
                    // rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                    // rect.render_widget(right, pets_chunks[1]);
                }
                _ => {}
            }
            rect.render_widget(statusbar, chunks[2]);
        })?;

        match self.rx.recv()? {
            Event::Input(event) => match event {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    self.terminal.show_cursor()?;
                    panic!("Quit");
                }
                KeyCode::Char('v') => self.active_menu_item = MenuItem::View,
                KeyCode::Char('m') => self.active_menu_item = MenuItem::Monitor,
                KeyCode::Down => {
                    // if let Some(selected) = pet_list_state.selected() {
                    //     let amount_pets = read_db().expect("can fetch pet list").len();
                    //     if selected >= amount_pets - 1 {
                    //         pet_list_state.select(Some(0));
                    //     } else {
                    //         pet_list_state.select(Some(selected + 1));
                    //     }
                    // }
                }
                KeyCode::Up => {
                    // if let Some(selected) = pet_list_state.selected() {
                    //     let amount_pets = read_db().expect("can fetch pet list").len();
                    //     if selected > 0 {
                    //         pet_list_state.select(Some(selected - 1));
                    //     } else {
                    //         pet_list_state.select(Some(amount_pets - 1));
                    //     }
                    // }
                }
                _ => {}
            },
            Event::Tick => {}
        }

        Ok(())
    }

    fn render_monitor<'a>() -> Paragraph<'a> {
        Paragraph::new(vec![Spans::from(vec![Span::raw("monitor")])])
    }

    fn render_home<'a>() -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "eyes1",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("some instructions")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }
}

impl Drop for EyesTui {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().ok();
        // execute!(
        //     self.terminal.backend_mut(),
        //     LeaveAlternateScreen,
        //     DisableMouseCapture
        // )
        // .ok();
        self.terminal.show_cursor().ok();
    }
}
