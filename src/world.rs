pub mod entity_map;
use crate::entity::creature::Creature;
use crate::entity::grass::Grass;
use crate::types::{Cell, Position, WorldGrid};
use crate::world::entity_map::EntityMap;
use std::rc::Rc;

// a world is a 2D grid of Cell
pub struct World {
    // the size of the 2D world (width and height are the same)
    size: u16,
    // the grid of cells - inner Vec is a row, outer Vec is a column
    // it is wrapped in a reference counted pointer so that it can be shared
    cells: WorldGrid,
    // the list of creatures in the world
    creatures: EntityMap<Creature>,
    // the list of all the grass blocks in the world
    grass: EntityMap<Grass>,
}

// public methods
impl World {
    pub fn new(size: u16) -> World {
        // create a square 2d vector of empty cells
        let grid = Rc::new(vec![vec![Cell::Empty; size as usize]; size as usize]);
        let world = World {
            size,
            cells: grid.clone(),
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

    // get creature by its number
    pub fn creature(&self, num: u64) -> Option<&Creature> {
        self.creatures.get_entity(num)
    }

    // // TODO TODO TODO TODO
    // // This is the crux of the ownership problem
    // // resolve this and all will be good right? :)
    // pub fn run(&mut self) {
    //     loop {
    //         for (_, creature) in &self.creatures {
    //             match creature.tick() {
    //                 Status::Alive => {}
    //                 Status::Dead => {
    //                     self.set_cell(creature.position, Cell::Empty);

    //                     self.creatures.remove(&creature.num);
    //                 }
    //             }
    //         }
    //     }
    // }

    // TODO maybe make this private - instead expose HashMap iterator for
    // creatures and grass
    pub fn get_cell(&self, position: Position) -> Cell {
        return self.cells[position.x as usize][position.y as usize];
    }
}

#[cfg(test)]
mod tests;
