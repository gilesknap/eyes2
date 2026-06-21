//! Headless evolution sanity experiment for the `giles` creature controller.
//!
//! These tests drive the simulation directly through `eyes2-lib` (no curses
//! GUI / binary involved) and are marked `#[ignore]` so they do not slow down
//! normal CI. They are sanity / observation runs rather than strict pass/fail
//! tests of emergent evolution: the assertions are deliberately modest and
//! robust (no panics, population survives at a sustainable grass rate).
//!
//! Run them (and see the periodic stdout printouts) with:
//!
//! ```sh
//! cargo test -p eyes2-lib --test evolution -- --ignored --nocapture
//! ```
//!
//! Use `--release` if it feels slow:
//!
//! ```sh
//! cargo test --release -p eyes2-lib --test evolution -- --ignored --nocapture
//! ```

use eyes2_lib::{Settings, World};

/// Build a Settings populated with ONLY `giles` creatures.
///
/// `grass_rate` is in 1..=100 where *higher* means grass grows *faster*
/// (see `World::ticks_per_grass`), so a lower value applies more selection
/// pressure.
fn giles_settings(size: u16, num_creatures: u16, grass_rate: u64) -> Settings {
    Settings {
        size,
        // seed a reasonable amount of grass so the world is not barren at t=0
        grass_count: (size as u32 * size as u32 / 4) as u16,
        grass_rate,
        creatures: vec![("giles".to_string(), num_creatures)],
        ..Settings::default()
    }
}

/// Run a simulation for `ticks` ticks, printing population & grass periodically.
///
/// Returns the `(population, grass)` recorded at the end of the run.
fn run_experiment(label: &str, mut settings: Settings, ticks: u64, samples: u64) -> (u64, usize) {
    // clamp like Settings::load() would, just to stay in valid ranges
    settings.grass_rate = settings.grass_rate.clamp(1, 100);

    let mut world = World::new(settings, 0);
    world.populate();

    let report_every = (ticks / samples).max(1);

    println!(
        "\n=== {label} (grass_rate={}) ===",
        world.grid.grass_rate
    );
    println!(
        "{:>10} {:>12} {:>10}",
        "tick", "population", "grass"
    );
    println!(
        "{:>10} {:>12} {:>10}",
        0,
        world.creature_count(),
        world.grid.grass_count()
    );

    for t in 1..=ticks {
        world.tick();
        if t % report_every == 0 || t == ticks {
            println!(
                "{:>10} {:>12} {:>10}",
                world.grid.ticks,
                world.creature_count(),
                world.grid.grass_count()
            );
        }
        // nothing left to evolve; stop early to keep the run short
        if world.creature_count() == 0 {
            println!("  (population extinct at tick {})", world.grid.ticks);
            break;
        }
    }

    let result = (world.creature_count(), world.grid.grass_count());
    println!(
        "--- {label}: final population={} grass={} ---",
        result.0, result.1
    );
    result
}

/// Sustainable-rate run: a world of only `giles` creatures at a high grass rate
/// where grass is abundant enough that an evolving lineage can keep up. With
/// `giles`, the byte-code genome must *evolve* a viable reproduction strategy,
/// so the population only reliably persists when grass is plentiful (empirically
/// grass_rate ~99-100 on a 40x40 world with 50 starting creatures). We assert
/// only that the simulation runs without panicking and that *some* population
/// survives to the end -- a robust sanity check, not a test of evolution.
#[ignore = "headless evolution experiment; run with --ignored --nocapture"]
#[test]
fn giles_survives_at_sustainable_grass_rate() {
    let settings = giles_settings(40, 50, 99);
    let (population, _grass) =
        run_experiment("sustainable", settings, 20_000, 20);

    assert!(
        population > 0,
        "expected the giles population to survive at a sustainable grass rate, \
         but it died out"
    );
}

/// Comparison run at a harsher grass rate to show the difference in dynamics.
/// At this lower rate grass regrows too slowly for the lineage to keep up and
/// the population trends towards extinction. We deliberately make NO assertion
/// on the outcome (it may decline, fluctuate or die out) -- this run exists
/// purely to print a contrasting trajectory; we only require no panic.
#[ignore = "headless evolution experiment; run with --ignored --nocapture"]
#[test]
fn giles_under_harsh_grass_rate_comparison() {
    let settings = giles_settings(40, 50, 80);
    let (_population, _grass) =
        run_experiment("harsh", settings, 20_000, 20);
    // No assertion on population -- this is an observational comparison run.
}
