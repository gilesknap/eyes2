//! Throughput / multi-core scaling benchmark for the eyes2 simulation.
//!
//! Builds one large `giles` world, snapshots it, then replays the *same*
//! starting state through `world.tick()` on 1..N rayon worker threads and on a
//! forced-serial reference path, reporting ticks/second and the speed-up.
//!
//! Run it with:
//!
//! ```sh
//! cargo run --release --example scaling -p eyes2
//! ```
//!
//! Optional args: `cargo run --release --example scaling -p eyes2 -- <size> <creatures> <ticks>`

use std::time::Instant;

use eyes2_lib::{Settings, World};

/// A `giles`-only world with no energy costs, so the population stays roughly
/// constant for the duration of a short benchmark and every creature runs a VM
/// instruction every tick - i.e. real, evenly spread "think" work.
fn bench_settings(size: u16, creatures: u16) -> Settings {
    Settings {
        size,
        grass_count: (size as u32 * size as u32 / 3) as u16,
        grass_rate: 90,
        // zero energy costs => nobody starves, population is stable across runs
        creature_move_energy: 0,
        creature_idle_energy: 0,
        grass_energy: 0,
        // a high reproduction threshold keeps the count steady over a short run
        creature_reproduction_energy: i32::MAX,
        creatures: vec![("giles".to_string(), creatures)],
        ..Settings::default()
    }
}

/// Number of timed repetitions per configuration. We report the *best* (fastest)
/// run, which best filters out noise from OS preemption / noisy cloud neighbours.
const REPEATS: u32 = 5;

/// Time `ticks` ticks of a fresh copy of `snapshot` on `threads` worker
/// threads, repeated `REPEATS` times. `threshold == usize::MAX` forces the
/// serial path regardless of population. Returns (population_at_end, best
/// ticks_per_second).
fn run(snapshot: &str, threads: usize, threshold: usize, ticks: u64) -> (u64, f64) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let mut best_tps = 0.0f64;
    let mut population = 0;
    for _ in 0..REPEATS {
        // deserialize an identical fresh world so every repetition races the
        // exact same starting state
        let mut world: World = serde_yaml::from_str(snapshot).unwrap();
        world.set_parallel_threshold(threshold);

        let start = Instant::now();
        pool.install(|| {
            for _ in 0..ticks {
                world.tick();
            }
        });
        let tps = ticks as f64 / start.elapsed().as_secs_f64();
        best_tps = best_tps.max(tps);
        population = world.creature_count();
    }

    (population, best_tps)
}

/// Measure how a tick's wall-time splits between the parallel "think" phase and
/// the serial "resolve" phase, on `threads` threads. This reveals the Amdahl
/// ceiling: if resolve is a large fraction of the tick, no number of cores can
/// speed the think phase up past it.
fn phase_split(snapshot: &str, threads: usize, ticks: u64) -> (f64, f64) {
    let mut world: World = serde_yaml::from_str(snapshot).unwrap();
    world.set_parallel_threshold(0);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let (mut think, mut resolve) = (0.0f64, 0.0f64);
    pool.install(|| {
        for _ in 0..ticks {
            let t0 = Instant::now();
            world.think_phase();
            let t1 = Instant::now();
            world.resolve_phase();
            let t2 = Instant::now();
            think += (t1 - t0).as_secs_f64();
            resolve += (t2 - t1).as_secs_f64();
        }
    });
    let total = think + resolve;
    (think / total * 100.0, resolve / total * 100.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(160);
    let creatures: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(6000);
    let ticks: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3000);

    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

    // build and populate the world once, then snapshot it to a YAML string so
    // every run below starts from byte-for-byte identical state
    let mut world = World::new(bench_settings(size, creatures), 0);
    world.populate();
    let population = world.creature_count();
    let snapshot = serde_yaml::to_string(&world).unwrap();

    println!(
        "\neyes2 scaling benchmark\n\
         -----------------------\n\
         world: {size}x{size} ({} cells)   creatures: {population}   ticks/run: {ticks}\n\
         detected cores: {cores}\n",
        size as u32 * size as u32,
    );

    // forced-serial reference (no rayon involvement at all)
    let (_pop, serial_tps) = run(&snapshot, 1, usize::MAX, ticks);
    println!("{:>18}: {:>14.0} ticks/sec   (reference)", "serial", serial_tps);

    // parallel path on 1..cores threads (threshold 0 == always parallel)
    println!();
    let mut base_tps = 0.0;
    for threads in 1..=cores {
        let (_pop, tps) = run(&snapshot, threads, 0, ticks);
        if threads == 1 {
            base_tps = tps;
        }
        println!(
            "  parallel {:>2} thread(s): {:>14.0} ticks/sec   speed-up vs 1: {:>5.2}x   vs serial: {:>5.2}x",
            threads,
            tps,
            tps / base_tps,
            tps / serial_tps,
        );
    }

    // phase split at 1 thread shows the Amdahl ceiling
    let (think1, resolve1) = phase_split(&snapshot, 1, ticks);
    let (think_n, resolve_n) = phase_split(&snapshot, cores, ticks);
    println!(
        "\n  tick phase split @1 thread:    think {:>5.1}%   resolve {:>5.1}%",
        think1, resolve1
    );
    println!(
        "  tick phase split @{} threads:   think {:>5.1}%   resolve {:>5.1}%",
        cores, think_n, resolve_n
    );
    let max_speedup = 100.0 / resolve1;
    println!(
        "  => Amdahl ceiling from serial resolve alone: {:.2}x\n",
        max_speedup
    );
}
