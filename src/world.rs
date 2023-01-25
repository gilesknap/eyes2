//! The world is a 2D grid of cells. Each cell can contain a creature or grass.
//! The world is responsible for updating the state of the world each tick.
//!
pub mod entity_map;
use crate::entity::creature::Creature;
use crate::entity::grass::Grass;
use crate::entity::Cell;
use crate::types::{Position, Update};
use crate::world::entity_map::EntityMap;
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
    // the size of the 2D world (width and height are the same)
    size: u16,
    // the grid of cells - inner Vec is a row, outer Vec is a column
    // it is wrapped in a reference counted pointer so that it can be shared
    grid: WorldGrid,
    // the list of creatures in the world
    creatures: EntityMap<Creature>,
    // the list of all the grass blocks in the world
    grass: EntityMap<Grass>,
    // queue of updates to the world to be applied at the end of the tick
    updates: Queue<Update>,
    // record of the number of ticks that have passed in the world
    ticks: u64,
}

// public methods
impl World {
    pub fn new(size: u16) -> World {
        // create a square 2d vector of empty cells
        let grid = Rc::new(RefCell::new(vec![
            vec![Cell::Empty; size as usize];
            size as usize
        ]));

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            size,
            grid: grid.clone(),
            creatures: EntityMap::<Creature>::new(grid.clone()),
            grass: EntityMap::<Grass>::new(grid.clone()),
            updates: UpdateQueue::new(),
            ticks: 0,
        };

        println!("Created a new world of size {} square", world.size);
        world
    }

    pub fn get_size(&self) -> u16 {
        self.size
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

    pub fn populate(&mut self, grass_count: u16, creature_count: u16) {
        self.grass.populate(grass_count);
        self.creatures.populate(creature_count);

        println!(
            "Added {} grass and {} creatures to the world",
            grass_count, creature_count
        );
    }

    // give each creature one clock cycle of processing
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

    fn apply_updates(&mut self) {
        while self.updates.size() > 0 {
            let update = self.updates.remove().unwrap();
            match update {
                Update::AddCreature(position) => {
                    self.creatures.add_entity(position).ok();
                }
                Update::MoveCreature(_id, _position) => {
                    // self.creatures.get_entity(&id).move_to(position);
                }
                Update::AddGrass(position) => {
                    self.grass.add_entity(position).ok();
                }
                Update::RemoveCreature(id) => {
                    self.creatures.remove_entity(&id);
                }
                Update::RemoveGrass(id) => {
                    self.grass.remove_entity(&id);
                }
            }
        }
    }

    // TODO maybe make this private - instead expose HashMap iterator for
    // creatures and grass EntityMaps
    pub fn get_cell(&self, position: Position) -> Cell {
        return self.grid.borrow()[position.x as usize][position.y as usize];
    }
}

#[cfg(test)]
mod tests;
