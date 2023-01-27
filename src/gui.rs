//! The GUI for the evolution simulation. Renders the current state of the world
//! and handles user input.
//!
use crate::entity::Cell;
use crate::world::World;
extern crate pancurses;

use direction::Coord;
use pancurses::{
    endwin, init_pair, initscr, start_color, ColorPair, COLOR_BLACK, COLOR_GREEN, COLOR_RED,
};

const RED: u8 = 1;
const GREEN: u8 = 2;

pub struct EyesGui {
    window: pancurses::Window,
}

// TODO I picked an ncurses library for this but I now see that much more
// can be done in the terminal. e.g. https://github.com/fdehau/tui-rs
// TODO switch to a richer terminal library

impl EyesGui {
    pub fn new() -> EyesGui {
        let window = initscr();
        start_color();
        init_pair(RED as i16, COLOR_RED, COLOR_BLACK);
        init_pair(GREEN as i16, COLOR_BLACK, COLOR_GREEN);
        EyesGui { window }
    }

    pub fn render(&mut self, world: &World) {
        self.window.mv(0, 0);
        self.window.printw(format!(
            "ticks:{}  creatures:{}  grass:{}\n\n",
            world.get_ticks(),
            world.creature_count(),
            world.grass_count()
        ));

        for y in 0..world.get_size() {
            for x in 0..world.get_size() {
                match world.get_cell(Coord { x, y }) {
                    Cell::Empty => {
                        self.window.printw("  ");
                    }
                    Cell::Grass(_) => {
                        self.window.attron(ColorPair(GREEN));
                        self.window.printw("  ");
                        self.window.attroff(ColorPair(GREEN));
                    }
                    Cell::Creature(_) => {
                        self.window.attron(ColorPair(RED));
                        self.window.printw("><");
                        self.window.attroff(ColorPair(RED));
                    }
                };
            }
            self.window.printw("\n");
        }

        self.window.refresh();
    }
}

impl Drop for EyesGui {
    fn drop(&mut self) {
        endwin();
    }
}
