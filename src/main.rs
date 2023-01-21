extern crate eyes2;

use eyes2::{gui, world};

fn main() {
    let size = 50;
    let grass_count = 80;
    let creature_count = 30;
    let energy = 1000;

    let mut world = world::World::new(size);
    world.populate(grass_count, creature_count, energy);

    gui::render();
}
