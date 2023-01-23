pub mod entity_map;
use crate::entity::creature::{Creature, Status};
use crate::entity::grass::Grass;
use crate::entity::Cell;
use crate::types::{Position, WorldGrid};
use crate::world::entity_map::EntityMap;
use std::cell::RefCell;
use std::rc::Rc;

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
        // between multiple owners:
        // - the world
        // - the creatures EntityMap
        // - the grass EntityMap

        let world = World {
            size,
            grid: grid.clone(),
            creatures: EntityMap::<Creature>::new(grid.clone()),
            grass: EntityMap::<Grass>::new(grid.clone()),
        };

        println!("Created a new world of size {} square", world.size);
        world
    }

    pub fn get_size(&self) -> u16 {
        self.size
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

    // get reference to mutable creature by its id
    pub fn creature(&mut self, id: u64) -> Rc<&mut Creature> {
        let creature = self.creatures.get_entity(&id).unwrap();
        Rc::new(creature)
    }

    // give each creature one clock cycle of processing
    pub fn tick(&mut self) {
        let mut remove_me = Vec::new();
        let ids: Vec<u64> = self.creatures.keys();

        for id in ids {
            // use unwrap here because we know the id is valid
            match self.creatures.get_entity(&id).unwrap().tick() {
                Status::Alive => {}
                Status::Dead => remove_me.push(id),
            }
        }

        for id in remove_me {
            self.creatures.remove_entity(&id).unwrap();
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
