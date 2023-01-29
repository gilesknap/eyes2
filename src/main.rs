use clap::Parser;
use eyes2::settings::Settings;
use eyes2::{gui, world};
use num_format::{Locale, ToFormattedString};
use std::{thread::sleep, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run a performance test
    #[arg(short, long)]
    performance: bool,
}

fn main() {
    let args = Args::parse();

    let settings = Settings::load();
    println!("Launching eyes2 evolution simulator ...");
    println!("{:#?}", settings);

    if args.performance {
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
            if tick % 1 == 0 {
                gui.render(&world);
            }
            tick += 1;
            world.tick();
            sleep(time::Duration::from_micros(5));
            if world.creature_count() == 0 {
                break;
            }
        }
    }
}

fn performance_test(settings: Settings) {
    // for performance testing, we use 1 creature which survive indefinitely
    // and move on every tick (this means the same load for all runs)
    let test_settings = Settings {
        size: 40,
        grass_count: 1000,
        creature_count: 1,
        grass_interval: 5000,
        max_grass_per_interval: 200,
        grass_energy: 1000,
        creature_move_energy: 100,
        creature_idle_energy: 1,
        creature_move_rate: 0.05,
        // info: the below adds all the other settings from the original settings
        // (which is none in this case as I've already listed them all)
        ..settings
    };

    let ticks = 10000000;
    let mut world = world::World::new(test_settings);

    world.populate();

    println!(
        "Performance test with {} ticks ...",
        ticks.to_formatted_string(&Locale::en)
    );
    let now = time::Instant::now();

    for _ in 0..ticks {
        world.tick();
    }

    println!(
        "Performance test ends with {} creatures and {} grass.\n\
        Performed {} ticks in {} milliseconds.",
        world.creature_count(),
        world.grass_count(),
        ticks.to_formatted_string(&Locale::en),
        now.elapsed().as_millis(),
    );
}
