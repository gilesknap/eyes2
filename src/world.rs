//! The world is a 2D grid of cells. Each cell can contain a creature or grass.
//! The world is responsible for updating the state of the world each tick.
//!
pub mod entity_map;
use crate::entity::creature::Creature;
use crate::entity::grass::Grass;
use crate::entity::Cell;
use crate::settings::Settings;
use crate::types::Update;
use direction::Coord;
use entity_map::EntityMap;
use queues::*;
use std::cell::RefCell;
use std::rc::Rc;

// a reference counted pointer to Reference Cell of a 2d vector of cells
// TODO replace RefCell with Arc when we have multiple threads
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
}

// public static methods
impl World {
    pub fn new(config: Settings) -> World {
        // create a square 2d vector of empty cells
        let grid = Rc::new(RefCell::new(vec![
            vec![Cell::Empty; config.size as usize];
            config.size as usize
        ]));

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            grid: grid.clone(),
            creatures: EntityMap::<Creature>::new(grid.clone(), config),
            grass: EntityMap::<Grass>::new(grid.clone(), config),
            updates: UpdateQueue::new(),
            ticks: 0,
            config,
        };

        println!("Created a new world of size {} square", world.config.size);
        world
    }
}

// public instance methods
impl World {
    pub fn get_size(&self) -> i32 {
        self.config.size
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn grass_count(&self) -> usize {
        self.grass.count()
    }

    pub fn creature_count(&self) -> usize {
        self.creatures.count()
    }

    pub fn populate(&mut self) {
        self.grass.populate(self.config.grass_count);
        self.creatures.populate(self.config.creature_count);

        println!(
            "Added {} grass and {} creatures to the world",
            self.config.grass_count, self.config.creature_count
        );
    }

    pub fn tick(&mut self) {
        let ids: Vec<u64> = self.creatures.keys();

        for id in ids {
            self.creatures.get_entity(&id).tick(&mut self.updates);
        }

        let ids = self.grass.keys();
        for id in ids {
            self.grass.get_entity(&id).tick(&mut self.updates);
        }

        self.apply_updates();
        self.ticks += 1;
    }

    /// read a cell from the grid - used for rendering the world
    pub fn get_cell(&self, position: Coord) -> Cell {
        return self.grid.borrow()[position.x as usize][position.y as usize];
    }
}

// private methods
impl World {
    /// process the updates to the world that have been queued in the previous tick
    fn apply_updates(&mut self) {
        while self.updates.size() > 0 {
            let update = self.updates.remove().unwrap();
            match update {
                Update::AddCreature(position) => {
                    self.creatures.add_new_entity(position).ok();
                }
                Update::AddGrass(position) => {
                    self.grass.add_new_entity(position).ok();
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
