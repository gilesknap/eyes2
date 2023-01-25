extern crate eyes2;

use eyes2::settings::SETTINGS;
use eyes2::{gui, world};
use std::{thread, time};

fn main() {
    loop {
        let mut world = world::World::new(SETTINGS.world_size);
        world.populate(SETTINGS.grass_count, SETTINGS.creature_count);

        let mut gui = gui::EyesGui::new();

        for i in 0..100000 {
            if i % 100 == 0 {
                gui.render(&world);
            }
            world.tick();
            thread::sleep(time::Duration::from_micros(5));
            if world.creature_count() == 0 {
                break;
            }
        }
    }
}
