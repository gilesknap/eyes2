extern crate eyes2;

use eyes2::{gui, world};

fn main() {
    let size = 50;
    let world = world::World::new(size);

    gui::render();
}
