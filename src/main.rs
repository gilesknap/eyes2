use clap::Parser;
use eyes2::gui::GuiCmd;
use eyes2::world;
use eyes2::world::grid::WorldGrid;
use eyes2::{gui::EyesGui, settings::Settings};
use num_format::{Locale, ToFormattedString};
use pancurses::endwin;
use std::{sync::mpsc, thread, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run a performance test
    #[arg(short, long)]
    performance: bool,
    // reset settings to defaults
    #[arg(short, long)]
    reset: bool,
}

const SPEED_TICKS: [u64; 10] = [1, 1, 1, 1, 1, 10, 100, 1000, 10000, 1000000];
const SPEED_DELAY: [u64; 10] = [1000, 100, 10, 1, 1, 1, 1, 1, 1, 0];

fn main() {
    let args = Args::parse();

    let settings = if args.reset {
        Settings::reset()
    } else {
        Settings::load()
    };

    if args.performance {
        performance_test(settings);
    } else {
        world_loop(settings);
    }
}

fn world_loop(mut settings: Settings) {
    // outer loop continues until user cancels
    'outer: loop {
        let mut world = world::types::World::new(settings);

        world.populate();

        let (tx_grid, rx_grid) = mpsc::channel();
        let (tx_gui_cmd, rx_gui_cmd) = mpsc::channel::<GuiCmd>();

        thread::spawn(move || {
            let mut gui = EyesGui::new();
            gui.gui_loop(rx_grid, tx_gui_cmd).ok()
        });

        // inner loop runs until all creatures die
        loop {
            if world.grid.ticks % SPEED_TICKS[world.grid.speed as usize - 1] == 0 {
                // Gui loop sends a command every 100ms, the None command indicates
                // no user input, but ready to receive the next world update
                let next_cmd = rx_gui_cmd.try_recv();
                if next_cmd.is_ok() {
                    let grid = &mut world.grid;
                    if handle_input(next_cmd.unwrap(), grid) {
                        break 'outer;
                    }
                    tx_grid.send(world.grid.clone()).unwrap();
                }

                thread::sleep(time::Duration::from_millis(
                    SPEED_DELAY[world.grid.speed as usize - 1],
                ));
            }
            world.grid.ticks += 1;
            world.tick();

            if world.creature_count() == 0 {
                // copy variable config to the next world
                settings.grass_rate = world.grid.grass_rate;
                settings.speed = world.grid.speed;
                break;
            }
        }
    }
    endwin();
}

fn handle_input(cmd: GuiCmd, grid: &mut WorldGrid) -> bool {
    match cmd {
        GuiCmd::Quit => {
            return true;
        }
        GuiCmd::SpeedUp => {
            grid.increment_speed(true);
        }
        GuiCmd::SpeedDown => {
            grid.increment_speed(false);
        }
        _ => {}
    }
    false
}

fn performance_test(settings: Settings) {
    // for performance testing, we use 1 creature which survives indefinitely
    let test_settings = Settings {
        size: 40,
        grass_count: 1000,
        creature_count: 1,
        grass_rate: 95,
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
    let mut world = world::types::World::new(test_settings);

    world.populate();

    println!("{:#?}", test_settings);
    println!(
        "\nPerformance test with {} ticks ...",
        ticks.to_formatted_string(&Locale::en)
    );
    println!("\ntypical result on giles ws1 is 500ms\n");

    let now = time::Instant::now();

    for _ in 0..ticks {
        world.tick();
    }

    println!(
        "Performance test ends with {} creatures and {} grass.\n\
        Performed {} ticks in {} milliseconds.",
        world.creature_count(),
        world.grid.grass_count(),
        ticks.to_formatted_string(&Locale::en),
        now.elapsed().as_millis(),
    );
}
