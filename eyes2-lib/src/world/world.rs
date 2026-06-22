use crate::entity::{look_world, new_genotype, Creature, Update};
use crate::settings::Settings;
use crate::utils;
use direction::{Coord, Direction};
use fastrand::Rng as FastRng;
use rayon::prelude::*;
use std::collections::HashMap;

use super::grid::{Cell, WorldGrid};

// a world is a 2D WorldGrid of Cell plus the creatures and grass blocks
//
// Creatures are stored in a contiguous `Vec` so that the per-creature "think"
// phase of a tick can be run across all CPU cores with rayon's `par_iter_mut`
// (each worker gets a disjoint `&mut Creature`). The grid still refers to
// creatures by their unique id; `id_index` maps an id back to its slot in the
// `Vec` in O(1) during the serial "resolve" phase. See DESIGN_MULTITHREAD.md.
pub struct World {
    // the grid of cells
    pub grid: WorldGrid,
    // the creatures in the world, in no particular order
    creatures: Vec<Creature>,
    // map from a creature's unique id to its slot in `creatures`
    id_index: HashMap<u64, usize>,
    // scratch buffer used to seed creatures when populating / loading a world
    updates: Vec<Update>,
    // per-creature intent slots for the hot path: the parallel think phase
    // writes each creature's intent into its own slot (disjoint, so lock-free
    // and allocation-free), and the serial resolve phase drains them. Reused
    // across ticks to avoid reallocation.
    intents: Vec<Option<Update>>,
    // the settings for the world
    config: Settings,
    // track when we will next call grass tick
    next_grass_tick: u64,
    // a random number generator
    rng: fastrand::Rng,
    // run the think phase in parallel only once there are at least this many
    // creatures. Below it the rayon fork/join cost outweighs the work and a
    // plain serial loop is much faster - see DESIGN_MULTITHREAD.md.
    parallel_threshold: usize,
}

// Default creature count at/above which the per-tick "think" phase is run in
// parallel. Tuned empirically: below a few hundred creatures the per-tick work
// is smaller than rayon's fork/join overhead so the serial path wins.
pub const DEFAULT_PARALLEL_THRESHOLD: usize = 512;

// public static methods
impl World {
    pub fn new(config: Settings, restarts: u64) -> World {
        // create a square 2d vector of empty cells
        let grid = WorldGrid::new(config.size, config.grass_rate, config.speed, restarts);

        World {
            grid,
            creatures: Vec::new(),
            id_index: HashMap::new(),
            updates: Vec::new(),
            intents: Vec::new(),
            config,
            next_grass_tick: 0,
            rng: FastRng::new(),
            parallel_threshold: DEFAULT_PARALLEL_THRESHOLD,
        }
    }

    pub fn load(config: Settings, grid: WorldGrid) -> World {
        let next_grass_tick = grid.ticks + grid.grass_rate;

        World {
            grid,
            creatures: Vec::new(),
            id_index: HashMap::new(),
            updates: Vec::new(),
            intents: Vec::new(),
            config,
            next_grass_tick,
            rng: FastRng::new(),
            parallel_threshold: DEFAULT_PARALLEL_THRESHOLD,
        }
    }
}

// public instance methods
impl World {
    pub fn get_size(&self) -> u16 {
        self.config.size
    }

    pub fn creature_count(&self) -> u64 {
        self.creatures.len() as u64
    }

    /// Set the creature-count threshold at or above which the per-tick think
    /// phase runs in parallel. Set to `usize::MAX` to force serial execution,
    /// or `0` to always go parallel. Mainly useful for benchmarking.
    pub fn set_parallel_threshold(&mut self, threshold: usize) {
        self.parallel_threshold = threshold;
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.grass_count as usize {
            let x = self.rng.i32(0..self.config.size as i32 - 1);
            let y = self.rng.i32(0..self.config.size as i32 - 1);
            self.grid.add_grass(Coord { x, y });
        }
        for creature in self.config.creatures.iter() {
            for _ in 0..creature.1 {
                let x = self.rng.i32(0..self.config.size as i32);
                let y = self.rng.i32(0..self.config.size as i32);

                let genotype = new_genotype(creature.0.as_str(), self.config.clone());

                let creature =
                    Creature::new(Box::new(genotype).unwrap(), Coord { x, y }, self.config.clone());
                self.updates.push(Update::AddEntity(creature));
            }
        }

        self.apply_updates();
    }

    pub fn tick(&mut self) {
        self.think_phase();
        self.resolve_phase();
    }

    /// The parallel half of a tick: run every creature's `think` and collect
    /// the intents, plus the (read-only-ish) grass growth scan. Exposed
    /// separately so benchmarks can time the parallel vs serial split; normal
    /// callers use [`tick`].
    pub fn think_phase(&mut self) {
        self.grid.ticks += 1;

        // PHASE 1 - "think": every creature advances itself by one tick and
        // yields the intent it would like applied to the world. This touches
        // only per-creature state, never the grid. Above `parallel_threshold`
        // creatures we spread it across all cores with rayon (order-preserving,
        // so conflict resolution stays stable); below it the serial loop is
        // faster because there is too little work to amortise fork/join.
        // split the borrow: the think phase reads the grid immutably while
        // mutating the (disjoint) creature store and its intent slots
        let grid = &self.grid;
        if self.creatures.len() >= self.parallel_threshold {
            // give every creature its own intent slot and fill them in parallel.
            // Each thread writes only the slots for the creatures it owns, so
            // there is no contention, no locking and no allocation per tick.
            self.intents.clear();
            self.intents.resize_with(self.creatures.len(), || None);
            self.creatures
                .par_iter_mut()
                .zip(self.intents.par_iter_mut())
                .for_each(|(creature, slot)| *slot = creature.think(grid));
        } else {
            let updates = &mut self.updates;
            for creature in self.creatures.iter_mut() {
                if let Some(update) = creature.think(grid) {
                    updates.push(update);
                }
            }
        }

        // limit calls to grass tick relative to grass_rate
        if self.grid.ticks >= self.next_grass_tick {
            self.grow_grass();
            self.next_grass_tick = self.grid.ticks + self.ticks_per_grass();
        }
    }

    /// The serial half of a tick: apply the collected intents to the grid,
    /// resolving conflicts (e.g. two creatures wanting the same cell). The hot
    /// path fills `intents` (slot per creature); the serial small-world path
    /// and the world-seeding code fill `updates`. Drain whichever holds work.
    pub fn resolve_phase(&mut self) {
        self.apply_intents();
        self.apply_updates();
        self.grid.ticks += 1;
    }
}

/// internal implementation details of the World struct
impl World {
    /// Apply the per-creature intents produced by the hot-path "think" phase.
    /// They were written into disjoint slots in parallel; here we walk them
    /// serially (this is the inherently serial "resolve" half of a tick).
    fn apply_intents(&mut self) {
        // take the slot buffer out of `self` so we can mutate the rest of
        // `self` while draining it, then hand the emptied buffer back for reuse
        let mut intents = std::mem::take(&mut self.intents);
        for slot in intents.drain(..) {
            if let Some(update) = slot {
                self.apply_one(update);
            }
        }
        self.intents = intents;
    }

    /// Apply any intents sitting in the `updates` queue (used to seed creatures
    /// when populating or loading a world, rather than the per-tick hot path).
    fn apply_updates(&mut self) {
        let mut updates = std::mem::take(&mut self.updates);
        for update in updates.drain(..) {
            self.apply_one(update);
        }
        self.updates = updates;
    }

    /// Apply a single intent to the world, mutating the grid and creature store
    /// and resolving conflicts (a move/spawn into an occupied cell is dropped).
    fn apply_one(&mut self, update: Update) {
        match update {
            Update::AddEntity(mut creature) => {
                let coord = creature.coord();
                let cell = self.grid.get_cell(coord);
                // skip the spawn entirely if the target cell is blocked
                match cell {
                    Cell::Empty | Cell::Grass => {}
                    Cell::Entity(_, _) | Cell::Wall => return,
                }
                let sigil = creature.get_sigil();
                // assign a fresh id only to brand new creatures; a creature
                // restored from a saved world keeps the id it was saved with
                // (set_id is a no-op once an id is set), so re-read the id
                // back from the creature and use it for *both* the index and
                // the grid cell to keep them in lock-step.
                creature.set_id(self.get_next_id());
                let id = creature.id();
                self.add_creature(creature);
                if let Cell::Grass = cell {
                    self.eat_grass(coord, id);
                }
                self.grid.set_cell(coord, Cell::Entity(id, sigil));
            }
            Update::RemoveEntity(id, coord) => {
                self.validate_creature(id, coord);
                self.remove_creature(id);
                self.grid.set_cell(coord, Cell::Empty);
            }
            Update::MoveEntity(id, old_coord, new_coord) => {
                self.validate_creature(id, old_coord);
                let cell = self.grid.get_cell(new_coord);
                match cell {
                    Cell::Empty => {}
                    Cell::Grass => self.eat_grass(new_coord, id),
                    // skip move if there is already a creature in the cell
                    // TODO this needs to change for carnivores
                    Cell::Entity(_, _) => return,
                    Cell::Wall => return,
                }
                let creature = self.get_creature_mut(id);
                creature.move_to(new_coord);
                let sigil = creature.get_sigil();
                self.grid.set_cell(old_coord, Cell::Empty);
                self.grid.set_cell(new_coord, Cell::Entity(id, sigil));
            }
            Update::Look(id) => {
                let coord = self.get_creature_mut(id).coord();
                let vision = look_world(coord, &self.grid);
                // Send the list of adjacent cells back to the requesting creature
                self.get_creature_mut(id).vision(vision);
            }
        }
    }

    /// look up a creature by its unique id (panics if absent - the grid and the
    /// id index are kept in lock-step so this should never happen)
    fn get_creature_mut(&mut self, id: u64) -> &mut Creature {
        let index = self.id_index[&id];
        &mut self.creatures[index]
    }

    /// immutable lookup of a creature by id (used when serialising the world)
    fn get_creature(&self, id: u64) -> &Creature {
        let index = self.id_index[&id];
        &self.creatures[index]
    }

    /// queue an intent to be applied to the world. Used to seed creatures when
    /// loading a saved world; the hot per-tick path fills the same buffer
    /// directly from the parallel think phase.
    fn queue_update(&mut self, update: Update) {
        self.updates.push(update);
    }

    /// add a creature to the store, recording its id -> slot mapping. The
    /// creature must already have its id assigned.
    fn add_creature(&mut self, creature: Creature) {
        let index = self.creatures.len();
        self.id_index.insert(creature.id(), index);
        self.creatures.push(creature);
        self.grid.creature_count = self.creature_count();
    }

    /// remove a creature by id using swap-remove and fix up the index of the
    /// creature that was moved into the vacated slot (O(1))
    fn remove_creature(&mut self, id: u64) {
        let index = self.id_index.remove(&id).expect("removing unknown creature");
        self.creatures.swap_remove(index);
        // swap_remove moved the last creature into `index` (unless we removed
        // the last one) - repoint its id at its new slot
        if index < self.creatures.len() {
            let moved_id = self.creatures[index].id();
            self.id_index.insert(moved_id, index);
        }
        self.grid.creature_count = self.creature_count();
    }

    fn get_next_id(&mut self) -> u64 {
        self.grid.next_id += 1;
        self.grid.next_id
    }

    fn validate_creature(&self, id: u64, coord: Coord) {
        let cell = self.grid.get_cell(coord);
        match cell {
            // TODO I'm going to treat these as panic for now. But maybe once we go multithread there may
            // be requests from creatures that have not yet realized they were deleted
            Cell::Entity(match_id, _) => {
                if match_id != id {
                    panic!("creature id does not match world grid");
                }
            }
            _ => panic!("no creature in world at grid coordinate"),
        };
    }

    fn ticks_per_grass(&self) -> u64 {
        // ticks per grass growth is between 100 to 1,000,000 in inverse
        // logarithmic proportion to grass_rate parameter of 1 - 100
        (101 - self.grid.grass_rate as u64).pow(2) * 100
    }

    fn grow_grass(&mut self) {
        // walk through all the cells in the grid except the edges and grow grass
        // adjacent to cells that already have grass
        let mut grow_dir = Direction::North;
        let mut new_grass: Vec<Coord> = Vec::new();

        for x in 1..self.config.size as i32 - 2 {
            for y in 1..self.config.size as i32 - 2 {
                let coord = Coord::new(x, y);
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Grass => {
                        new_grass.push(coord + grow_dir.coord());
                        grow_dir = utils::rotate_direction(grow_dir);
                    }
                    _ => {}
                }
            }
        }

        for coord in new_grass {
            self.grid.add_grass(coord);
        }
    }

    fn eat_grass(&mut self, coord: Coord, id: u64) {
        self.grid.remove_grass(coord);
        let energy = self.config.grass_energy;
        self.get_creature_mut(id).eat(energy);
    }
}

#[path = "world_test.rs"]
#[cfg(test)]
mod test;

#[path = "store.rs"]
pub mod store;
