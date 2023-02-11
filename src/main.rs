use clap::Parser;
use eyes2::gui::GuiCmd;
use eyes2::world;
use eyes2::world::grid::WorldGrid;
use eyes2::{gui::EyesGui, settings::Settings};
use std::io;
use std::io::prelude::*;
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

const SPEED_TICKS: [u64; 10] = [1, 1, 1, 1, 10, 50, 100, 1000, 10000, 100000];
const SPEED_DELAY: [u64; 10] = [1000, 10, 2, 1, 1, 1, 1, 1, 1, 0];

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
    // setup channels for gui and world thread communications
    let (tx_grid, rx_grid) = mpsc::channel();
    let (tx_gui_cmd, rx_gui_cmd) = mpsc::channel::<GuiCmd>();

    // launch the gui thread
    thread::spawn(move || {
        let mut gui = EyesGui::new();
        gui.gui_loop(rx_grid, tx_gui_cmd).ok()
    });

    let mut restarts = 0;
    // outer loop continues until user quits or resets the world
    'outer: loop {
        let mut world = world::types::World::new(settings, restarts);

        world.populate();

        // inner loop runs until all creatures die
        'inner: loop {
            if (world.grid.ticks % SPEED_TICKS[world.grid.speed as usize - 1]) == 0 {
                // Gui loop sends a command or GuiCmd::None every 100ms
                let next_cmd = rx_gui_cmd.try_recv();
                if next_cmd.is_ok() {
                    match next_cmd.unwrap() {
                        GuiCmd::Reset => break 'inner,
                        GuiCmd::Quit => break 'outer,
                        input => handle_input(input, &mut world.grid),
                    }
                    tx_grid.send(world.grid.clone()).unwrap();
                }

                if world.grid.speed < 10 {
                    thread::sleep(time::Duration::from_millis(
                        SPEED_DELAY[world.grid.speed as usize - 1],
                    ));
                }
            }
            world.grid.ticks += 1;
            world.tick();

            if world.creature_count() == 0 {
                break;
            }
        }
        // copy variable config to the next world
        settings.grass_rate = world.grid.grass_rate;
        settings.speed = world.grid.speed;
        restarts += 1;
    }
}

fn handle_input(cmd: GuiCmd, grid: &mut WorldGrid) {
    match cmd {
        GuiCmd::SpeedUp => {
            grid.increment_speed(true);
        }
        GuiCmd::SpeedDown => {
            grid.increment_speed(false);
        }
        GuiCmd::GrassUp => {
            grid.increment_grass_rate(true);
        }
        GuiCmd::GrassDown => {
            grid.increment_grass_rate(false);
        }
        _ => {}
    }
}

fn performance_test(settings: Settings) {
    // for performance testing, we use 1 creature which survives indefinitely
    let test_settings = Settings {
        size: 40,
        grass_count: 1000,
        creature_count: 50,
        grass_rate: 50,
        creature_move_energy: 0,
        creature_idle_energy: 0,
        creature_move_rate: 0.005,
        speed: 10,

        ..settings
    };

    println!("{:#?}", test_settings);
    println!("\nPerformance test with above settings ...");
    println!("\ntypical rate on giles ws1 is 6.7 million ticks/s (49 creatures)\n");

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();

    world_loop(test_settings);
}
