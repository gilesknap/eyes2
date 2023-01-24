use crate::entity::Cell;
use crate::types::Position;
use crate::world::World;
extern crate pancurses;

use pancurses::{endwin, initscr};

// TODO rudimentary rendering of the world as a placeholder
pub fn render(world: &World) {
    let window = initscr();

    window.printw(format!("Rendering World of size: {}\n\n", world.get_size()));

    for y in 0..world.get_size() {
        for x in 0..world.get_size() {
            match world.get_cell(Position { x, y }) {
                Cell::Empty => window.printw(" "),
                Cell::Grass(_) => window.printw("*"),
                Cell::Creature(_) => window.printw("o"),
            };
        }
        window.printw("\n");
    }
    window.printw("\n");

    window.refresh();
    window.getch();
    endwin();
}
