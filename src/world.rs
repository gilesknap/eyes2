//! The world is a 2D grid of cells. Each cell can contain a creature or grass.
//! The world is responsible for updating the state of the world each tick.
//!
pub mod world_api;
use std::collections::HashMap;

use crate::entity::Entity;
use crate::entity::{creature::Creature, grass::Grass, Cell};
use crate::settings::Settings;
use direction::Coord;
use queues::*;

// a reference counted pointer to Reference Cell of a 2d vector of cells
// TODO replace RefCell with RwLock when we go multi-threaded (I think
// its a RwLock because only World should make changes. TODO TODO
// but wait - maybe only World accesses it at all? and every interaction
// goes via the update queue?)
//
// The outer Rc allows us to share the RefCell between multiple owners.
// The RefCell allows us to mutate the contents of the Vec from any of
// these owners. At present this is safe as we are single threaded.
pub type WorldGrid = Vec<Vec<Cell>>;

// a queue of updates to the world to be applied at the end of the tick
pub type UpdateQueue = Queue<Update>;

// a world is a 2D grid of Cell plus a HashMap of creatures and grass blocks
pub struct World {
    // the grid of cells
    grid: WorldGrid,
    // the list of creatures in the world
    creatures: HashMap<u64, Creature>,
    // the list of all the grass blocks in the world
    grass: HashMap<u64, Grass>,
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
    // next unique id to assign to an Entity
    next_id: u64,
}

/// Represent the possible world update service requests that
/// Entities can place on the update queue.
#[derive(Clone)]
pub enum Update {
    AddCreature(Creature),
    MoveCreature(Creature, Coord),
    AddGrass(u64, Coord),
    RemoveCreature(Creature),
    RemoveGrass(u64, Coord),
}

/// This is where world services requests from Entities to make changes to
/// the world.
impl World {
    /// process the updates to the world that have been queued in the previous tick
    fn eat_grass(&mut self, grass_id: u64, id: u64) {
        self.grass.remove(&grass_id);
        self.creatures
            .get_mut(&id)
            .unwrap()
            .eat(self.config.grass_energy);
    }

    fn get_next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    fn apply_updates(&mut self) {
        while self.updates.size() > 0 {
            let update = self.updates.remove().unwrap();
            match update {
                Update::AddCreature(creature) => {
                    let coord = creature.coord();
                    let id = creature.id();
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Empty => {
                            self.creatures.insert(id, creature);
                            () // TODO REALLY??
                        }
                        Cell::Grass(grass_id) => {
                            self.creatures.insert(id, creature);
                            self.eat_grass(grass_id, id);
                        }
                        _ => continue, // skip add if there is already a creature in the cell
                    };
                    self.grid[coord.x as usize][coord.y as usize] = Cell::Creature(id);
                }
                Update::AddGrass(_id, coord) => {
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Empty => {
                            // TODO should call grow here (using id to get grass to grow)
                            let id = self.get_next_id();
                            let grass = Grass::new(id, coord, self.config);
                            self.grass.insert(id, grass);
                            self.grid[coord.x as usize][coord.y as usize] = Cell::Grass(id)
                        }
                        _ => continue,
                    };
                }
                Update::RemoveCreature(creature) => {
                    let coord = creature.coord();
                    self.creatures.remove(&creature.id());
                    self.grid[coord.x as usize][coord.y as usize] = Cell::Empty;
                }
                Update::RemoveGrass(id, coord) => {
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Grass(grass_id) => {
                            if grass_id == id {
                                self.grass.remove(&id);
                                self.grid[coord.x as usize][coord.y as usize] = Cell::Empty;
                            }
                        }
                        _ => continue,
                    };
                }
                Update::MoveCreature(creature, new_coord) => {
                    let old_coord = creature.coord();
                    let cell = self.grid[new_coord.x as usize][new_coord.y as usize];
                    match cell {
                        Cell::Empty => {}
                        Cell::Grass(grass_id) => self.eat_grass(grass_id, creature.id()),
                        // skip move if there is already a creature in the cell
                        Cell::Creature(_) => continue,
                    }
                    self.creatures
                        .get_mut(&creature.id())
                        .unwrap()
                        .move_to(new_coord);

                    let grid = &mut self.grid;
                    grid[old_coord.x as usize][old_coord.y as usize] = Cell::Empty;
                    grid[new_coord.x as usize][new_coord.y as usize] =
                        Cell::Creature(creature.id());
                }
            }
        }
    }
}
#[cfg(test)]
mod tests;
