use crate::entity::Cell;
use crate::types::Position;
use crate::world::World;
extern crate pancurses;

use pancurses::{
    endwin, init_pair, initscr, start_color, Attribute, Attributes, ColorPair, A_NORMAL,
    COLOR_BLACK, COLOR_GREEN, COLOR_RED,
};

const RED: u8 = 1;
const GREEN: u8 = 2;

pub struct EyesGui {
    window: pancurses::Window,
}

impl EyesGui {
    pub fn new() -> EyesGui {
        let window = initscr();
        start_color();
        init_pair(RED as i16, COLOR_BLACK, COLOR_RED);
        init_pair(GREEN as i16, COLOR_BLACK, COLOR_GREEN);
        EyesGui { window }
    }

    pub fn render(&mut self, world: &World) {
        self.window.mv(0, 0);
        self.window.printw(format!(
            "ticks:{}  creatures:{}  grass{}\n\n",
            world.get_ticks(),
            world.creature_count(),
            world.grass_count()
        ));

        for y in 0..world.get_size() {
            for x in 0..world.get_size() {
                match world.get_cell(Position { x, y }) {
                    Cell::Empty => {
                        self.window.printw("  ");
                    }
                    Cell::Grass(_) => {
                        self.window.attron(ColorPair(GREEN));
                        self.window.printw("  ");
                        self.window.attroff(ColorPair(GREEN));
                    }
                    Cell::Creature(_) => {
                        self.window.printw("><");
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
        self.window.printw("\nPress enter to exit\n");
        self.window.getch();
        endwin();
    }
}
