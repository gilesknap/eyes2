extern crate eyes2;

use eyes2::{gui, world};
use std::{thread, time};

fn main() {
    let size = 25;
    let grass_count = 10;
    let creature_count = 30;

    let mut world = world::World::new(size);
    world.populate(grass_count, creature_count);

    let mut gui = gui::EyesGui::new();

    for _ in 0..30000 {
        gui.render(&world);
        world.tick();
        thread::sleep(time::Duration::from_millis(1));
    }
}
