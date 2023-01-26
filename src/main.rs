extern crate eyes2;

use eyes2::settings::Settings;
use eyes2::{gui, world};
use std::{thread, time};

fn main() {
    let settings = Settings::load();
    println!("{:?}", settings);
    let mut world = world::World::new(settings);

    world.populate();

    let mut gui = gui::EyesGui::new();

    for i in 0..1000000 {
        if i % 1000 == 0 {
            gui.render(&world);
        }
        world.tick();
        thread::sleep(time::Duration::from_micros(5));
        // if world.creature_count() == 0 {
        //     break;
        // }
    }
    println!("Done! ({} ticks)", world.get_ticks());
}
