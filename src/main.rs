use std::time::Instant;

use clap::Parser;
use eyes2::settings::Settings;
use eyes2::{gui, world};
use std::{thread, time};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run a performance test
    #[arg(short, long)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let settings = Settings::load();
    println!("Launching eyes2 evolution simulator ...");
    println!("{:#?}", settings);

    if args.test {
        performance_test(settings);
    } else {
        world_loop(settings);
    }
}

fn world_loop(settings: Settings) {
    // outer loop continues until user cancels
    loop {
        let mut world = world::World::new(settings);

        world.populate();

        // let mut tui = tui::Tui::new();
        let mut gui = gui::EyesGui::new();

        let mut tick: u64 = 0;
        // inner loop runs until all creatures die
        loop {
            tick += 1;
            if tick % 100 == 0 {
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

fn performance_test(settings: Settings) {
    // for performance testing, we want all creatures to survive indefinitely
    // and move on every tick (this means the same load for all runs)
    let test_settings = Settings {
        creature_idle_energy: 0,
        creature_move_energy: 0,
        ..settings
    };

    let ticks = 10000000;
    let mut world = world::World::new(test_settings);

    world.populate();

    println!("Performance test with {} ticks ...", ticks);
    let now = Instant::now();

    for _ in 0..ticks {
        world.tick();
    }

    println!(
        "Performed {} ticks in {} milliseconds.",
        ticks,
        now.elapsed().as_millis()
    );
}
