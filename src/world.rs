//! The world is a 2D grid of cells. Each cell can contain a creature or grass.
//! The world is responsible for updating the state of the world each tick.
//!
pub mod world_api;
use std::collections::HashMap;

use crate::entity::Entity;
use crate::entity::{creature::Creature, grass::Grass, Cell};
use crate::settings::Settings;
use direction::Coord;

// The world is a 2D grid of cells
pub type WorldGrid = Vec<Vec<Cell>>;

// a queue of updates to the world to be applied at the end of the tick
pub type UpdateQueue = Vec<Update>;

// a world is a 2D grid of Cell plus a HashMap of creatures and grass blocks
pub struct World {
    // the grid of cells
    grid: WorldGrid,
    // the list of creatures in the world
    creatures: HashMap<u64, Creature>,
    // the list of all the grass blocks in the world
    grass: HashMap<u64, Grass>,
    // queue of updates to the world to be applied at the end of the tick
    updates: UpdateQueue,
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
pub enum Update {
    AddCreature(Creature),
    MoveCreature(u64, Coord, Coord),
    AddGrass(u64, Coord),
    RemoveCreature(u64, Coord),
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
        // TODO is this the best way to iterate over all items in a queue?
        while self.updates.len() > 0 {
            let update = self.updates.remove(0);
            match update {
                Update::AddCreature(creature) => {
                    let coord = creature.coord();
                    let id = creature.id();
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Empty => {
                            self.creatures.insert(id, creature);
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
                            // ANS: In fact we can do that in the grass tick and pass the new
                            // grass object using the pattern tested in AddCreature
                            let id = self.get_next_id();
                            let grass = Grass::new(id, coord, self.config);
                            self.grass.insert(id, grass);
                            self.grid[coord.x as usize][coord.y as usize] = Cell::Grass(id)
                        }
                        _ => continue,
                    };
                }
                Update::RemoveCreature(id, coord) => {
                    self.creatures.remove(&id);
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
                Update::MoveCreature(id, old_coord, new_coord) => {
                    let cell = self.grid[new_coord.x as usize][new_coord.y as usize];
                    match cell {
                        Cell::Empty => {}
                        Cell::Grass(grass_id) => self.eat_grass(grass_id, id),
                        // skip move if there is already a creature in the cell
                        Cell::Creature(_) => continue,
                    }
                    self.creatures.get_mut(&id).unwrap().move_to(new_coord);
                    let grid = &mut self.grid;
                    grid[old_coord.x as usize][old_coord.y as usize] = Cell::Empty;
                    grid[new_coord.x as usize][new_coord.y as usize] = Cell::Creature(id);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests;
