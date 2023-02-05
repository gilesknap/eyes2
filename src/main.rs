mod gui2;
use clap::Parser;
use eyes2::settings::Settings;
use eyes2::world;
use gui2::EyesTui;
use num_format::{Locale, ToFormattedString};
use std::{thread::sleep, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run a performance test
    #[arg(short, long)]
    performance: bool,
    // reset settings to defaults
    #[arg(short, long)]
    reset: bool,
    // test the tui gui
    #[arg(short, long)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    let settings = if args.reset {
        Settings::reset()
    } else {
        Settings::load()
    };

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

        let mut gui = EyesTui::new();

        let mut tick: u64 = 0;
        // inner loop runs until all creatures die
        loop {
            if tick % 100 == 0 {
                gui.render(&world).unwrap();
            }
            tick += 1;
            world.tick();
            // TODO make this delay configurable and for larger delays make
            // the gui.render run every tick so you can see details of progress
            sleep(time::Duration::from_micros(1));
            if world.creature_count() == 0 {
                break;
            }
        }
    }
}

fn performance_test(settings: Settings) {
    // for performance testing, we use 1 creature which survives indefinitely
    let test_settings = Settings {
        size: 40,
        grass_count: 1000,
        creature_count: 1,
        grass_interval: 5000,
        max_grass_per_interval: 200,
        grass_energy: 1000,
        creature_move_energy: 0,
        creature_idle_energy: 0,
        creature_move_rate: 0.05,
        // The below adds all the other settings from the original 'settings'
        // meaning that if I add a new setting, I don't have to add it here too
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
