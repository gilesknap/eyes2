//! The GUI for the evolution simulation. Renders the current state of the world
//! and handles user input.
//!
use crate::entity::Cell;
use crate::world::World;

use num_format::{Locale, ToFormattedString};
use std::cmp::{max, min};

use direction::Coord;
use pancurses::{
    endwin, init_pair, initscr, start_color, ColorPair, COLOR_BLACK, COLOR_GREEN, COLOR_RED,
};

const RED: u8 = 1;
const GREEN: u8 = 2;
const BLACK: u8 = 3;
const GREEN_FG: u8 = 4;

struct Pane {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub struct EyesGui {
    window: pancurses::Window,
    left_pane: Pane,
    right_pane: Pane,
    y_max: i32,
    x_max: i32,
    view_world: bool,
}

impl EyesGui {
    pub fn new() -> EyesGui {
        let window = initscr();

        start_color();
        init_pair(RED as i16, COLOR_RED, COLOR_BLACK);
        init_pair(GREEN as i16, COLOR_GREEN, COLOR_BLACK);
        init_pair(GREEN_FG as i16, COLOR_BLACK, COLOR_GREEN);
        init_pair(BLACK as i16, COLOR_BLACK, COLOR_BLACK);

        window.keypad(true);
        window.clear();
        window.refresh();
        pancurses::curs_set(0);

        EyesGui {
            window,
            left_pane: Pane {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            right_pane: Pane {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            y_max: 0,
            x_max: 0,
            view_world: true,
        }
    }

    pub fn render(&mut self, world: &World) {
        let (y_max, x_max) = self.window.get_max_yx();
        if (y_max, x_max) != (self.y_max, self.x_max) {
            (self.y_max, self.x_max) = (y_max, x_max);
            self.window.clear();
            self.resize(world);
            self.window.refresh();
        }

        if self.view_world {
            self.render_grid(world);
        }

        let ticks = { world.get_ticks().to_formatted_string(&Locale::en) }.to_string();
        let creatures = world.creature_count().to_string();
        let grass = world.grass_count().to_string();
        self.status(0, world, "ticks:", &ticks);
        self.status(1, world, "creatures:", &creatures);
        self.status(2, world, "grass:", &grass);
    }
}

// private methods
impl EyesGui {
    fn resize(&mut self, world: &World) -> bool {
        let status_width = 30;
        let borders = 2;
        let world_width = world.get_size();

        let x_space = min(
            max(self.x_max - world_width as i32 - status_width - borders, 0),
            world_width,
        );

        self.left_pane = Pane {
            x: 0,
            y: 0,
            width: x_space as u16,
            height: self.y_max as u16,
        };

        self.right_pane = Pane {
            x: (x_space + 1) as u16,
            y: 0,
            width: max(min(status_width, self.x_max - x_space - borders), 0) as u16,
            height: self.y_max as u16,
        };

        self.right_pane.mvwin(0, x_space + 1);
        self.right_pane.resize(
            self.y_max,
            max(min(status_width, self.x_max - x_space - borders), 0),
        );

        self.right_pane.draw_box(0, 0);

        x_space > 0 && self.y_max > 0
    }

    fn render_grid(&mut self, world: &World) {
        for y in 0..world.get_size() {
            if y >= self.y_max {
                break;
            }
            self.left_pane.mv(y, 0);
            for x in 0..world.get_size() {
                if x >= self.x_max - 1 {
                    break;
                }
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

    fn status(&mut self, pos: i32, world: &World, label: &str, value: &str) {
        let world_width = world.get_size();

        let y = 1 + pos * 2;
        if world_width >= self.x_max {
            return;
        }

        self.right_pane.mv(y, 1);
        self.right_pane.printw(label);

        self.right_pane.mv(y + 1, 1);
        self.right_pane.attron(ColorPair(GREEN));
        self.right_pane.addstr(value);
        self.right_pane.attroff(ColorPair(GREEN));

        self.right_pane.refresh();
    }
}

impl Drop for EyesGui {
    fn drop(&mut self) {
        endwin();
    }
}
