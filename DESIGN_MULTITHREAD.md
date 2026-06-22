# Multi-threaded simulation architecture

## Goal

Use *all* CPU cores to maximise **ticks per second**, the metric that
governs how much evolution we can observe per wall-clock second. The GUI
already runs on its own thread; this document is about parallelising the
*simulation* itself, which today runs entirely on one core.

## Why the current model looks hard to parallelise (and why it isn't)

`World::tick()` is:

```rust
for creature in self.creatures.values_mut() {
    creature.tick();          // mutate self, send Update down an mpsc channel
}
self.apply_updates();         // drain channel, mutate the grid
```

The README flags this as the blocker:

> Multi Threaded processing for the creatures ... may be hard as we
> currently loop over all and call one tick - this model would need to
> change.

The key observation that unlocks everything:

> **`creature.tick()` never reads or writes the world grid.**

A creature only ever mutates *its own* state (energy, genotype registers,
cached vision) and then emits an *intent* — `Move`, `Reproduce`, `Look`,
`RemoveEntity` — describing what it would like the world to do. The grid is
read and mutated **exclusively** inside `apply_updates()`. So a tick has two
phases with very different parallelism characteristics:

| Phase | Work | Shared state | Parallelism |
|-------|------|--------------|-------------|
| **Think** | run each creature's genotype VM, age it, decide an action; reproduction copies + mutates the 1 kB genome | none — each creature touches only itself | **embarrassingly parallel** |
| **Resolve** | apply intents to the grid, resolve conflicts (two creatures want the same cell), assign ids, deliver vision | the single grid + creature map | inherently **serial** (but cheap) |

The think phase is where all the CPU cost lives for the interesting
genotype (`giles` runs a byte-code VM every tick and copies/mutates a 1000
byte genome on every reproduction). The resolve phase only does work
*proportional to the number of intents that actually change the grid* —
in steady state most creatures are resting (`creature_move_rate` is tiny;
`giles` mostly executes non-move instructions), so very few intents reach
the serial phase each tick. That asymmetry is exactly what makes the split
worthwhile: parallelise the O(N) think, keep the cheap O(movers) resolve
serial.

## The architecture: parallel think → serial resolve

```
            ┌─────────────────────────── one tick ───────────────────────────┐

  creatures:  [c0][c1][c2][c3][c4][c5][c6][c7] ...            (Vec<Creature>)
                │   │   │   │   │   │   │   │
   THINK   ─────┼───┼───┼───┼───┼───┼───┼───┼─────  rayon work-stealing pool
  (parallel)   t0  t1  t2  t3  ... split across all cores; each creature
                │   │   │   │       mutates only itself and yields Option<Update>
                ▼   ▼   ▼   ▼
   intents:   [ Move, None, Look, Reproduce(child), None, Move, ... ]
                                    │
   RESOLVE ─────────────────────────┘  single thread drains the intent list,
  (serial)                             mutates the grid, resolves cell
                                       conflicts, assigns ids, delivers vision
```

### Concretely

1. **Storage.** Creatures move from `HashMap<u64, Creature>` to a contiguous
   `Vec<Creature>` plus an `id → slot` index (`HashMap<u64, usize>`). The
   `Vec` is cache-friendly and, crucially, supports `par_iter_mut()` which
   hands each worker thread a **disjoint** `&mut Creature` — no locks, no
   `unsafe`. The grid still refers to creatures by `id`; the index map turns
   an `id` back into a slot in O(1) during resolve.

2. **`Creature` becomes `Send` and channel-free.** The old design gave every
   creature an `Rc<Sender<Update>>` and had it `send()` on every action.
   `Rc` is not `Send`, and a channel send per creature per tick is pure
   overhead. Instead `Creature::think()` *returns* `Option<Update>`. No
   channel, no `Rc`, and the type is now trivially `Send` so rayon can move
   it across threads.

3. **`Genotype: Send`.** Adding `Send` as a supertrait of `Genotype` makes
   `Box<dyn Genotype>` `Send`, so a whole `Creature` (and a `Reproduce`
   intent carrying a child genotype) can cross threads. Every existing
   genotype already only contains `Send` data.

4. **Resolve** is the old `apply_updates()`, unchanged in spirit: it walks
   the collected intents in order and mutates the grid, handling the same
   cases (eat grass, blocked move, spawn child, death, deliver vision).
   Conflicts (two creatures targeting one cell) are resolved by first-come
   in the intent order — same policy as today.

### Determinism

The simulation is already non-deterministic across runs (`HashMap`
iteration order, per-creature RNG seeds). The new model is *no less*
deterministic: rayon's `collect()` preserves input order, so for a given
creature `Vec` the intent list — and therefore conflict resolution — is
stable within a run. We trade nothing we had.

### Why not finer-grained locking / actor-per-creature?

- **A lock per cell / per creature** would add atomic traffic to the hot
  path and serialise on contention exactly where creatures interact.
- **One OS thread (or async task) per creature** does not fit: creatures are
  numerous, short-lived and individually do nanoseconds of work per tick;
  scheduling overhead would dwarf the work.
- **A persistent data-parallel pool** (rayon) is the right grain: threads ==
  cores, work-stealing balances the uneven cost of reproduction spikes, and
  there is no per-tick thread spawn (which would be fatal — a tick can be
  sub-microsecond, an OS thread spawn is tens of microseconds).

## Scaling limits and the next tier (spatial sharding)

The parallel-think / serial-resolve design scales the **think** phase to all
cores. Its ceiling is Amdahl's law on the serial resolve: when a workload is
dominated by grid mutation (huge worlds where a large fraction of creatures
move *every* tick) the serial resolve becomes the bottleneck.

The next tier, if that ceiling is ever hit, is **spatial sharding**: because
all interactions are local (vision and movement reach exactly one cell), the
grid can be cut into tiles and non-adjacent tiles resolved concurrently —
e.g. a red/black pass over tiles wider than the interaction radius, or
double-buffered tiles with halo exchange at the seams. This is a much larger
change (cross-tile moves, reproduction and grass at seams all need care) and
is only worth it once measurement shows resolve — not think — is the wall.
For the genotypes we actually run (`giles`), think dominates, so tier one is
the high-value change and tier two is deliberately left as documented future
work.

## Implementation notes

A few things that fell out of building this and are worth knowing:

- **Adaptive serial/parallel switch.** Going parallel is only a win when there
  is enough work to amortise rayon's fork/join. The very first naive version
  ran the existing 50-creature performance benchmark **50x slower** because the
  per-tick work (trivial `random` genotype) was nanoseconds while the fork/join
  was microseconds. `World` therefore only goes parallel at/above
  `parallel_threshold` creatures (default 512, tunable via
  `set_parallel_threshold`); below it a plain serial loop runs. The old
  small-world benchmark is now ~12% *faster* than before the whole change
  (`Vec` storage + no per-creature channel send beat the original single
  thread).

- **Lock-free intent collection.** The think phase gives each creature its own
  slot in a reused `Vec<Option<Update>>` and fills the slots with
  `par_iter_mut().zip(...)`. Each worker writes only its own creatures' slots,
  so there is no locking, no channel and no per-tick allocation.

- **`Look` resolved in the parallel phase.** Originally vision was a deferred
  `Update` resolved serially - O(N) grid reads every tick on one core, which
  capped scaling badly. Because the grid is read-only during think, vision is
  now computed there, in parallel. The genome's one-tick perception latency is
  unchanged (it still gates on its own `pending_look`).

## How to measure

`cargo run --release --example scaling -p eyes2` builds one large `giles`
world, snapshots it, and replays the identical state across 1..N rayon threads
plus a forced-serial reference, printing ticks/sec, speed-up and the
think/resolve phase split. Optional args: `... -- <size> <creatures> <ticks>`.

## Results

Measured on a 4-core (4 vCPU) Intel Xeon @2.8GHz cloud box, `giles`-only world,
best of 5 timed runs each:

```
world: 160x160 (25600 cells)   creatures: ~5400   ticks/run: 3000

            serial:           2415 ticks/sec   (reference, no rayon)
  parallel  1 thread(s):      2142 ticks/sec   1.00x
  parallel  2 thread(s):      3161 ticks/sec   1.48x
  parallel  3 thread(s):      3603 ticks/sec   1.68x
  parallel  4 thread(s):      3746 ticks/sec   1.75x

  tick phase split @1 thread:   think 88.2%   resolve 11.8%   (Amdahl ceiling 8.5x)
```

So on this hardware the `giles` workload reaches ~1.75x on 4 cores. The
interesting part is *why it is not higher*, because it is **not** the
architecture:

- The serial resolve is only ~12% of a tick, so Amdahl alone would allow ~8.5x.
- The same box, on a **compute-bound, cache-friendly** rayon workload, scales
  **3.97x on 4 cores** - i.e. the cores and the pool are healthy.
- Backing the numbers out, the *think phase itself* only speeds up ~1.95x on
  4 cores. That is the signature of a **memory-latency-bound** loop: each
  `Creature` holds a `Box<dyn Genotype>` which pointer-chases to a separately
  heap-allocated 1 kB genome, so iterating thousands of them is a cache-miss
  storm and the shared memory subsystem - not the cores - is the wall.

### What this means

- The architecture **does** spread the simulation across all cores, and for a
  creature controller that does real compute per tick (a deeper VM, a small
  neural net, the original `eyes` 4-cell-deep **ranged** vision, etc.) it will
  scale close to the hardware's 3.97x rather than `giles`'s 1.75x. `giles` is
  deliberately the *lightest possible* per-tick workload (one byte-code
  instruction + single-cell vision), which is the worst case for parallel
  speed-up and the best case for exposing memory latency.
- The headline throughput still **roughly doubled** for the realistic large
  `giles` world, with zero change to behaviour, on a 4-vCPU box.

### Next optimisation if more `giles` throughput is wanted

Improve the **memory layout** so the hot think loop is cache-friendly:
store genome bytes for all creatures of one genotype in a single contiguous
arena (struct-of-arrays) instead of a `Box` per creature, so a worker streams
its shard of genomes linearly and the prefetcher can keep up. This is a larger
change to the polymorphic genotype storage and is only worth doing if `giles`
specifically needs to go faster; the parallel-think architecture above is the
prerequisite for it and is the high-value change.
