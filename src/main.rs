extern crate eyes2;

use eyes2::settings::SETTINGS;
use eyes2::{gui, world};
use std::{thread, time};

fn main() {
    let mut world = world::World::new(SETTINGS.world_size);
    world.populate(SETTINGS.grass_count, SETTINGS.creature_count);

    let mut gui = gui::EyesGui::new();

    for i in 0..30000 {
        if i % 10 == 0 {
            gui.render(&world);
        }
        world.tick();
        thread::sleep(time::Duration::from_micros(100));
    }
}
