//! The GUI for the evolution simulation. Renders the current state of the world
//! and handles user input.
//!
use crate::entity::Cell;
use crate::world::types::World;

use num_format::{Locale, ToFormattedString};
use std::{
    cmp::{max, min},
    time::{self, Instant},
};

use direction::Coord;
use pancurses::{
    endwin, init_pair, initscr, start_color, ColorPair, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN,
    COLOR_RED,
};

const RED: u8 = 1;
const GREEN: u8 = 2;
const BLACK: u8 = 3;
const BLUE: u8 = 4;

pub struct EyesGui {
    window: pancurses::Window,
    left_pane: pancurses::Window,
    right_pane: pancurses::Window,
    y_max: i32,
    x_max: i32,
    last_tick: u64,
    last_tick_time: Instant,
    pub speed: u64,
}

impl EyesGui {
    pub fn new() -> EyesGui {
        let window = initscr();
        // choose some minimal initial sizes
        let left_pane = pancurses::newwin(1, 1, 0, 0);
        let right_pane = pancurses::newwin(1, 1, 0, 3);

        start_color();
        init_pair(RED as i16, COLOR_RED, COLOR_BLACK);
        init_pair(GREEN as i16, COLOR_GREEN, COLOR_BLACK);
        init_pair(BLUE as i16, COLOR_BLUE, COLOR_BLACK);
        init_pair(BLACK as i16, COLOR_BLACK, COLOR_BLACK);

        window.timeout(0);
        window.keypad(true);
        window.clear();
        window.refresh();
        pancurses::curs_set(0);
        pancurses::noecho();
        // pancurses::raw();

        EyesGui {
            window,
            left_pane,
            right_pane,
            y_max: 0,
            x_max: 0,
            speed: 1,
            last_tick: 0,
            last_tick_time: time::Instant::now(),
        }
    }

    pub fn render(&mut self, world: &World) {
        let (y_max, x_max) = self.window.get_max_yx();
        if (y_max, x_max) != (self.y_max, self.x_max) {
            (self.y_max, self.x_max) = (y_max, x_max);
            self.right_pane.clear();
            self.resize(world);
            self.window.refresh();
            self.render_grid(world);
        } else {
            self.render_grid(world);
        }

        let ticks = world.get_ticks().to_formatted_string(&Locale::en);
        let creatures = world.creature_count().to_string();
        let grass = world.grass_count().to_string();

        let rate = {
            let ticks = world.get_ticks() - self.last_tick;
            let time = self.last_tick_time.elapsed().as_secs_f64();
            ((ticks as f64 / time) as u64).to_formatted_string(&Locale::en)
        };
        self.last_tick = world.get_ticks();
        self.last_tick_time = time::Instant::now();
        self.status(1, "ticks:", &ticks);
        self.status(3, "creatures:", &creatures);
        self.status(5, "grass:", &grass);
        self.status(7, "ticks/s:", &rate);
        self.status(9, "speed:", &(self.speed).to_string());
        self.status(11, "grass rate:", &world.grass_rate().to_string());
    }

    pub fn handle_input(&mut self, world: &mut World) -> bool {
        match self.window.getch() {
            Some(pancurses::Input::Character('q')) => return true,
            Some(pancurses::Input::Character(' ')) => {
                self.speed = 10;
            }
            Some(pancurses::Input::KeyUp) => {
                self.speed += 1;
            }
            Some(pancurses::Input::KeyDown) => {
                self.speed -= 1;
            }
            Some(pancurses::Input::KeyRight) => {
                world.increment_grass_rate(true);
            }
            Some(pancurses::Input::KeyLeft) => {
                world.increment_grass_rate(false);
            }
            Some(_) => {}
            None => {}
        };
        self.speed = self.speed.clamp(1, 10);
        false
    }
}

// private methods
impl EyesGui {
    fn resize(&mut self, world: &World) -> bool {
        let status_width = 30;
        let world_width = world.get_size();

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

    fn render_grid(&mut self, world: &World) {
        let (height, width) = self.left_pane.get_max_yx();
        for y in 0..height {
            self.left_pane.mv(y, 0);
            for x in 0..width {
                match world.get_cell(Coord { x, y }) {
                    Cell::Empty => {
                        self.left_pane.attron(ColorPair(BLACK));
                        self.left_pane.addstr(" ");
                        self.left_pane.attroff(ColorPair(BLACK));
                    }
                    Cell::Grass(_) => {
                        self.left_pane.attron(ColorPair(GREEN));
                        self.left_pane.printw("o");
                        self.left_pane.attroff(ColorPair(GREEN));
                    }
                    Cell::Creature(_) => {
                        self.left_pane.attron(ColorPair(RED));
                        self.left_pane.printw("X");
                        self.left_pane.attroff(ColorPair(RED));
                    }
                };
            }
        }
        self.left_pane.refresh();
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
}

impl Drop for EyesGui {
    fn drop(&mut self) {
        endwin();
    }
}
