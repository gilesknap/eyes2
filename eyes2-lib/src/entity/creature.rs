//! The representation of a creature in the world
//!
//! This module implements the generic behaviour of a creature and enforces
//! the rules of the world. The rules are:-
//!
//! 1. A creature can move one cell in any of the 8 directions (including diagonals)
//! 2. A herbivore can eat grass if it is in the same cell as the grass
//! 3. A carnivore can eat another creature if it is in the same cell as the other creature
//! 4. A creature can reproduce if it has enough energy
//! 5. A creature dies if it has no energy
//! 6. A creature can request the value adjacent cells (i.e. the vision in 'eyes)
//!
//! The rules are implemented in the tick() method which is called once per tick
//! of the world. Global settings control the energy costs and rewards of each action.
//!
//! The specific behaviour of an individual is determined by its genotype.
//!
//! genotypes must implement the Genotype trait and are registered in the
//! new_genotype() function.
//!
//! genotypes call back into the creature to perform actions such as moving
//! 'looking' via the GenotypeCallback trait.
//!

use crate::utils::move_pos;

use super::genotype::genotype::GenotypeActions;
use super::vision::{look_world, Vision};
use super::Genotype;
use super::Update;
use crate::world::WorldGrid;
use crate::Settings;
use direction::{Coord, Direction};
use fastrand::Rng as FastRng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Creature {
    // the unique id of the creature used to identify it in the world
    id: u64,
    // the position of the creature in the world for reverse lookup
    #[serde(skip)]
    coord: Coord,
    // the amount of energy the creature has
    energy: i32,
    // global settings for the world which include generic creature settings
    #[serde(skip)]
    config: Settings,
    // the world rules are different for herbivores and carnivores
    _herbivore: bool,
    // the genotype of the creature which determines its behaviour
    genotype: Box<dyn Genotype>,
    // the sigil used to represent the creature in the world
    sigil: char,
}

// The representation of a creature in the world
impl Creature {
    pub fn new(genotype: Box<dyn Genotype>, coord: Coord, config: Settings) -> Creature {
        let (b, e) = config.creature_initial_energy;

        // TODO maybe pass a pre-created rng around to avoid creating a new one each time
        let rng = FastRng::new();
        let energy = rng.i32(b..e);
        let sigil = genotype.get_sigil();

        Creature {
            id: 0,
            coord,
            energy,
            config,
            _herbivore: true,
            genotype,
            sigil: sigil,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn move_to(&mut self, pos: Coord) {
        self.coord = pos;
    }

    pub fn set_id(&mut self, id: u64) {
        // id is immutable once set
        if self.id == 0 {
            self.id = id;
        }
    }

    pub fn set_config(&mut self, config: Settings) {
        self.config = config;
    }

    pub fn eat(&mut self, amount: i32) {
        self.energy += amount;
        self.genotype.set_energy(self.energy);
    }

    /// Advance this creature by one tick of the world clock and return the
    /// single world `Update` it would like applied (if any).
    ///
    /// This is the parallel "think" phase. It is given a **read-only** view of
    /// the grid (which is never mutated during this phase) and mutates only
    /// this creature - energy, genotype registers and cached vision. Because it
    /// touches no shared mutable state it is safe to run for every creature
    /// concurrently across all cores. Any change to the *grid* is deferred to a
    /// world `Update` that is applied later, serially. See DESIGN_MULTITHREAD.md.
    ///
    /// A `Look` is resolved here rather than as a deferred update: reading the
    /// surroundings is read-only, so it can be done in parallel, and it keeps
    /// the expensive O(N) vision work out of the serial resolve phase. The genome
    /// still only *uses* the new vision on its next tick (it gates on its own
    /// `pending_look`), so the one-tick perception latency is preserved.
    pub fn think(&mut self, grid: &WorldGrid) -> Option<Update> {
        self.energy -= self.config.creature_idle_energy;

        let coord = self.coord();
        // check for death
        if self.energy <= 0 {
            return Some(Update::RemoveEntity(self.id, coord));
        }

        // call the genotype specific tick method
        match self.genotype.tick() {
            GenotypeActions::Move(direction) => Some(self.move_dir(direction)),
            GenotypeActions::Reproduce(genotype) => Some(self.reproduce(genotype)),
            GenotypeActions::Look => {
                // resolve the look immediately from the read-only grid and
                // deliver it to the genotype; nothing to defer to the world
                let vision = look_world(coord, grid);
                self.genotype.vision(vision);
                None
            }
            GenotypeActions::None => None,
        }
    }

    pub fn get_sigil(&self) -> char {
        self.sigil
    }

    pub fn vision(&mut self, vision: Vision) {
        self.genotype.vision(vision);
    }
}

// private instance methods
impl Creature {
    /// Build the child creature and return the `AddEntity` intent for it. The
    /// world splits energy authoritatively when the intent is applied; we halve
    /// our own estimate here so the child inherits a sensible value.
    fn reproduce(&mut self, genotype: Box<dyn Genotype>) -> Update {
        let mut child = Creature::new(genotype, self.coord, self.config.clone());
        self.energy /= 2;
        child.energy = self.energy;
        // child is spawned to the left unless we are against the left wall
        if self.coord.x == 0 {
            child.coord.x += 1;
        } else {
            child.coord.x -= 1
        }
        Update::AddEntity(child)
    }

    fn move_dir(&mut self, direction: Direction) -> Update {
        let new_pos = move_pos(self.coord, direction, self.config.size);

        let (id, coord) = (self.id(), self.coord());
        self.energy -= self.config.creature_move_energy;
        Update::MoveEntity(id, coord, new_pos)
    }
}
