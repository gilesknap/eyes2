#![feature(test)]
extern crate test;

pub mod gui;

use clap::Parser;
use eyes2_lib::{save_world, Settings, World, WorldGrid};
use gui::{EyesGui, GuiCmd};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread, time,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// reset settings to defaults
    #[arg(short, long)]
    reset: bool,
    /// use performance test settings (with GUI - for comparison with bench test)
    #[arg(short, long)]
    performance: bool,
}

// Simulation speed control arrays.
// There is a tradeoff between how often we query the GUI and how long
// a delay there is in the main loop. Playing with these values can
// give a relatively smooth control of the speed of the simulation.
// These two arrays represent that tradeoff.
const SPEED_TICKS: [u64; 10] = [1, 1, 1, 1, 10, 50, 100, 500, 1000, 1000];
const SPEED_DELAY: [u64; 10] = [300, 10, 2, 1, 1, 1, 1, 1, 1, 0];

fn main() {
    let args = Args::parse();
    let settings = if args.performance {
        get_performance_settings()
    } else {
        get_settings(args.reset)
    };
    world_loop(settings);
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
    let mut paused = false;

    // outer loop continues until user quits or resets the world
    'outer: loop {
        let mut world = World::new(settings.clone(), restarts);

        world.populate();

        // inner loop runs until all creatures die
        'inner: loop {
            let tick_result = do_tick(&mut world, &tx_grid, &rx_gui_cmd, &mut paused);
            match tick_result {
                Err(TickActions::Reset) => break 'inner,
                Err(TickActions::Quit) => break 'outer,
                Ok(()) => {}
            };

            if world.creature_count() == 0 {
                break 'inner;
            }
        }
        // copy variable config to the next world
        settings.grass_rate = world.grid.grass_rate;
        settings.speed = world.grid.speed;
        restarts += 1;
    }
}

enum TickActions {
    Reset,
    Quit,
}

fn do_tick(
    world: &mut World,
    tx_grid: &Sender<WorldGrid>,
    rx_gui_cmd: &Receiver<GuiCmd>,
    paused: &mut bool,
) -> Result<(), TickActions> {
    if (world.grid.ticks % SPEED_TICKS[world.grid.speed as usize - 1]) == 0 {
        // Gui loop sends a command or GuiCmd::None every 100ms
        let next_cmd = rx_gui_cmd.try_recv();

        if next_cmd.is_ok() {
            match next_cmd.unwrap() {
                GuiCmd::Reset => return Err(TickActions::Reset),
                GuiCmd::Quit => return Err(TickActions::Quit),
                GuiCmd::Pause => *paused = !*paused,
                GuiCmd::SpeedUp => world.grid.increment_speed(true),
                GuiCmd::SpeedDown => world.grid.increment_speed(false),
                GuiCmd::GrassUp => world.grid.increment_grass_rate(true),
                GuiCmd::GrassDown => world.grid.increment_grass_rate(false),
                GuiCmd::Save => save_world(&world),
                _ => {}
            };
            tx_grid.send(world.grid.clone()).unwrap();
        }

        thread::sleep(time::Duration::from_millis(
            SPEED_DELAY[world.grid.speed as usize - 1],
        ));
    }
    if !*paused {
        world.tick();
    }
    Ok(())
}

fn get_settings(reset: bool) -> Settings {
    match reset {
        true => Settings::reset(),
        false => Settings::load(),
    }
}

fn get_performance_settings() -> Settings {
    // for performance testing, we use 50 'random' creatures which survive indefinitely
    // because there are no energy costs. Plus a typical amount of grass which also
    // eats some processing time.
    Settings {
        size: 40,
        grass_count: 1000,
        grass_rate: 85,
        creature_move_energy: 0,
        creature_idle_energy: 0,
        creature_move_rate: 0.001,
        grass_energy: 0,
        speed: 10,
        creatures: vec![("random".to_string(), 50)],
        ..Settings::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn performance_test(bencher: &mut Bencher) {
        let settings = get_performance_settings();

        let mut world = World::new(settings, 0);

        world.populate();

        let world_ref = &mut world;

        bencher.iter(|| {
            black_box({
                for _ in 0..10000 {
                    world_ref.tick()
                }
            });
        });
    }
}
