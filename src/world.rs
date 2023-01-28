//! The world is a 2D grid of cells. Each cell can contain a creature or grass.
//! The world is responsible for updating the state of the world each tick.
//!
pub mod entity_map;
pub mod world_api;
use crate::entity::{creature::Creature, grass::Grass, Cell};
use crate::settings::Settings;
use direction::Coord;
use entity_map::EntityMap;
use queues::*;
use std::cell::RefCell;
use std::rc::Rc;

// a reference counted pointer to Reference Cell of a 2d vector of cells
// TODO replace RefCell with RwLock when we go multi-threaded (I think
// its a RwLock because only World should make changes. TODO TODO
// but wait - maybe only World accesses it at all? and every interaction
// goes via the update queue?)
//
// The outer Rc allows us to share the RefCell between multiple owners.
// The RefCell allows us to mutate the contents of the Vec from any of
// these owners. At present this is safe as we are single threaded.
pub type WorldGrid = Rc<RefCell<Vec<Vec<Cell>>>>;

// a queue of updates to the world to be applied at the end of the tick
pub type UpdateQueue = Queue<Update>;

// a world is a 2D grid of Cell
pub struct World {
    // the grid of cells
    grid: WorldGrid,
    // the list of creatures in the world
    creatures: EntityMap<Creature>,
    // the list of all the grass blocks in the world
    grass: EntityMap<Grass>,
    // queue of updates to the world to be applied at the end of the tick
    updates: Queue<Update>,
    // record of the number of ticks that have passed in the world
    ticks: u64,
    // the settings for the world
    config: Settings,
    // track when we will next call grass tick
    next_grass_tick: u64,
    // a random number generator
    rng: rand::rngs::StdRng,
}

#[derive(Clone)]
pub enum Update {
    AddCreature(Creature),
    MoveCreature(u64, Coord),
    AddGrass(Grass),
    RemoveCreature(u64),
    RemoveGrass(u64),
}

impl World {
    /// process the updates to the world that have been queued in the previous tick
    fn apply_updates(&mut self) {
        while self.updates.size() > 0 {
            let update = self.updates.remove().unwrap();
            match update {
                Update::AddCreature(creature) => {
                    self.creatures.add_entity(creature).ok();
                }
                Update::AddGrass(grass) => {
                    self.grass.add_entity(grass).ok();
                }
                Update::RemoveCreature(id) => {
                    self.creatures.remove_entity(&id);
                }
                Update::RemoveGrass(id) => {
                    self.grass.remove_entity(&id);
                }
                Update::MoveCreature(id, position) => {
                    let cell = self.grid.borrow()[position.x as usize][position.y as usize];
                    match cell {
                        Cell::Empty => {}
                        Cell::Grass(grass_id) => {
                            self.grass.remove_entity(&grass_id);
                            self.creatures.get_entity(&id).eat(self.config.grass_energy);
                        }
                        // skip move if there is already a creature in the cell
                        Cell::Creature(_) => continue,
                    }
                    self.creatures.move_entity(&id, position);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests;
