use std::collections::HashMap;

use direction::Coord;

use crate::{
    entity::{creature::Creature, entity::Cell},
    settings::Settings,
};

// The world is a 2D grid of cells
pub type WorldGrid = Vec<Cell>;

// a queue of updates to the world to be applied at the end of the tick
// Note I did not use queues crate because it clones the objects in the
// Queue and we specifically want to pass object ownership for e.g.
// AddCreature(Creature)
pub type UpdateQueue = Vec<Update>;

// a world is a 2D grid of Cell plus a HashMap of creatures and grass blocks
pub struct World {
    // the grid of cells
    pub(super) grid: WorldGrid,
    // the list of creatures in the world
    pub(super) creatures: HashMap<u64, Creature>,
    // queue of updates to the world to be applied at the end of the tick
    pub(super) updates: UpdateQueue,
    // record of the number of ticks that have passed in the world
    pub(super) ticks: u64,
    // the settings for the world
    pub(super) config: Settings,
    // the interval between grass growth events
    pub(super) grass_rate: u64,
    // track when we will next call grass tick
    pub(super) next_grass_tick: u64,
    // a random number generator
    pub(super) rng: rand::rngs::StdRng,
    // next unique id to assign to an Entity
    pub(super) next_id: u64,
}

/// Represent the possible world update service requests that
/// Entities can place on the update queue.
pub enum Update {
    AddCreature(Creature),
    MoveCreature(u64, Coord, Coord),
    RemoveCreature(u64, Coord),
}
