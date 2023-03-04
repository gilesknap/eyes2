//! The GUI for the evolution simulation. Renders the current state of the world
//! and handles user input.
//!
use chrono::Utc;
use eyes2_lib::{Cell, WorldGrid};

use num_format::{Locale, ToFormattedString};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::{
    cmp::{max, min},
    time::{self, Instant},
};

use direction::Coord;
use pancurses::{
    endwin, init_pair, initscr, start_color, ColorPair, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN,
    COLOR_RED, COLOR_WHITE,
};

const RED: u8 = 1;
const GREEN: u8 = 2;
const BLACK: u8 = 3;
const BLUE: u8 = 4;
const CREATURE: u8 = 5;

#[derive(Debug, Clone)]
pub enum GuiCmd {
    None,
    Quit,
    Reset,
    Pause,
    Resume,
    Save,
    Load,
    SpeedUp,
    SpeedDown,
    SpeedMax,
    GrassUp,
    GrassDown,
}

pub struct EyesGui {
    window: pancurses::Window,
    left_pane: pancurses::Window,
    right_pane: pancurses::Window,
    help_pane: pancurses::Window,
    y_max: i32,
    x_max: i32,
    last_tick: u64,
    last_tick_time: Instant,
}

const DATE_FMT: &'static str = "%y-%m-%d %H:%M:%S";

impl EyesGui {
    pub fn new() -> EyesGui {
        let window = initscr();
        // choose some minimal initial sizes
        let left_pane = pancurses::newwin(1, 1, 0, 0);
        let right_pane = pancurses::newwin(1, 1, 0, 3);
        let help_pane = pancurses::newwin(20, 44, 3, 10);

        start_color();
        init_pair(RED as i16, COLOR_RED, COLOR_BLACK);
        init_pair(CREATURE as i16, COLOR_WHITE, COLOR_BLACK);
        init_pair(GREEN as i16, COLOR_GREEN, COLOR_BLACK);
        init_pair(BLUE as i16, COLOR_BLUE, COLOR_BLACK);
        init_pair(BLACK as i16, COLOR_BLACK, COLOR_BLACK);

        window.timeout(0);
        window.keypad(true);
        window.clear();
        window.refresh();
        window.nodelay(true);

        pancurses::curs_set(0);
        pancurses::noecho();

        EyesGui {
            window,
            left_pane,
            right_pane,
            help_pane,
            y_max: 0,
            x_max: 0,
            last_tick: 0,
            last_tick_time: time::Instant::now(),
        }
    }

    pub fn gui_loop(
        &mut self,
        rx_grid: mpsc::Receiver<WorldGrid>,
        tx_gui_cmd: mpsc::Sender<GuiCmd>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let cmd = self.get_cmd();
            tx_gui_cmd.send(cmd.clone())?;
            match cmd {
                GuiCmd::Quit => break,
                GuiCmd::Reset => continue,
                _ => {}
            }

            let grid: WorldGrid = rx_grid.recv()?;
            self.render(grid);
            thread::sleep(time::Duration::from_millis(50));
        }
        Ok(())
    }

    pub fn render(&mut self, grid: WorldGrid) {
        let grid_ref = &grid;
        let (y_max, x_max) = self.window.get_max_yx();

        if (y_max, x_max) != (self.y_max, self.x_max) {
            (self.y_max, self.x_max) = (y_max, x_max);
            self.right_pane.clear();
            self.resize(grid_ref);
            self.window.refresh();
        }
        self.render_grid(grid_ref);

        let l = &Locale::en;
        let rate = {
            // use i64 and max to avoid anomalous behavior when ticks reset
            let ticks = max(0, grid.ticks as i64 - self.last_tick as i64);
            let time = self.last_tick_time.elapsed().as_secs_f64();
            ((ticks as f64 / time) as u64).to_formatted_string(l)
        };
        self.last_tick = grid.ticks;
        self.last_tick_time = time::Instant::now();
        let dur = Utc::now() - grid.start_time;

        let mut y = 1;
        self.status(y, "ticks:", &grid.ticks.to_formatted_string(l));
        y += 1;
        self.status(y, "ticks/s:", &rate);
        y += 1;
        self.status(y, "restarts:", &grid.restarts.to_formatted_string(l));
        y += 1;
        self.status(y, "started:", &grid.start_time.format(DATE_FMT).to_string());
        y += 1;
        self.status(y, "runtime:", &dur.num_seconds().to_formatted_string(l));
        y += 2;
        self.status(y, "living:", &grid.creature_count.to_string());
        y += 1;
        self.status(
            y,
            "deceased:",
            &(grid.next_id - grid.creature_count).to_formatted_string(l),
        );
        y += 1;
        self.status(y, "grass:", &grid.grass_count().to_string());
        y += 2;
        self.status(y, "speed:", &grid.speed.to_string());
        y += 1;
        self.status(y, "grass rate:", &grid.grass_rate.to_string());

        self.footer(" q: quit, h: help ");
    }

    pub fn get_cmd(&mut self) -> GuiCmd {
        let result = match self.window.getch() {
            Some(pancurses::Input::Character('q')) => GuiCmd::Quit,
            Some(pancurses::Input::Character(' ')) => GuiCmd::Pause,
            Some(pancurses::Input::Character('r')) => GuiCmd::Reset,
            Some(pancurses::Input::Character('s')) => GuiCmd::Save,
            Some(pancurses::Input::Character('l')) => GuiCmd::Load,
            Some(pancurses::Input::KeyUp) => GuiCmd::SpeedUp,
            Some(pancurses::Input::KeyDown) => GuiCmd::SpeedDown,
            Some(pancurses::Input::KeyRight) => GuiCmd::GrassUp,
            Some(pancurses::Input::KeyLeft) => GuiCmd::GrassDown,
            Some(pancurses::Input::Character('h')) => {
                self.show_help();
                GuiCmd::None
            }
            _ => GuiCmd::None,
        };
        pancurses::flushinp();

        result
    }
}

// private methods
impl EyesGui {
    fn resize(&mut self, grid: &WorldGrid) -> bool {
        let status_width = 33;
        let world_width = grid.get_size();

        let mut x_space = self.x_max - status_width;
        x_space = x_space.clamp(0, world_width as i32);
        let y_space = self.y_max.clamp(0, world_width as i32);
        let w_stats = max(min(status_width, 1 + self.x_max - x_space), 0);

        self.left_pane.resize(y_space, x_space);
        self.right_pane.mvwin(0, x_space);
        self.right_pane.resize(y_space, w_stats);

        self.right_pane.draw_box(0, 0);
        x_space > 0 && self.y_max > 0
    }

    fn render_grid(&mut self, grid: &WorldGrid) {
        let (height, width) = self.left_pane.get_max_yx();
        for y in 0..height {
            self.left_pane.mv(y, 0);
            for x in 0..width {
                match grid.get_cell(Coord { x, y }) {
                    Cell::Empty => {
                        self.left_pane.attron(ColorPair(BLACK));
                        self.left_pane.addstr(" ");
                        self.left_pane.attroff(ColorPair(BLACK));
                    }
                    Cell::Grass => {
                        self.left_pane.attron(ColorPair(GREEN));
                        self.left_pane.printw("o");
                        self.left_pane.attroff(ColorPair(GREEN));
                    }
                    Cell::Entity(_, sigil) => {
                        self.left_pane.attron(ColorPair(CREATURE));
                        self.left_pane.addch(sigil);
                        self.left_pane.attroff(ColorPair(CREATURE));
                    }
                };
            }
        }
        self.left_pane.refresh();
    }

    fn footer(&mut self, text: &str) {
        let (height, width) = self.right_pane.get_max_yx();
        // center the text
        let left = (width - text.len() as i32) / 2;
        self.right_pane.mv(height - 1, left);
        self.right_pane.addnstr(text, width as usize - 2);
        self.right_pane.refresh();
    }

    fn status(&mut self, pos: i32, label: &str, value: &str) {
        let margin = 14;
        let borders = 2;
        let (height, width) = self.right_pane.get_max_yx();

        if pos >= height - 1 || pos <= 0 {
            return;
        }

        self.right_pane.mv(pos, 1);
        self.right_pane.attron(ColorPair(BLUE));
        self.right_pane.printw(label);
        self.right_pane.attroff(ColorPair(BLUE));

        let padded = format!("{}                  ", value);
        self.right_pane.mv(pos, margin);
        self.right_pane
            .addnstr(padded, (width - margin - borders) as usize);

        self.right_pane.refresh();
    }

    fn show_help(&mut self) {
        let help = "
  -------------- COMMANDS ---------------

                q:   quit
                r:   reset world
            space:   pause the world
          up/down:   speed up/down
       left/right:   grass up/down
                h:   show this help

  ---------------------------------------

  See this file for world config:
  $HOME/.config/eyes2/default-config.toml

  launch with -r to reset the config file
  launch with --help for more options";
        self.help_pane.mvaddstr(1, 3, help);
        self.help_pane.refresh();
        self.help_pane.draw_box(0, 0);
        self.help_pane.getch();
        self.y_max = 0; // force a resize which will redraw everything
    }
}

impl Drop for EyesGui {
    fn drop(&mut self) {
        endwin();
    }
}
