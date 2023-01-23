pub mod entity_map;
use crate::entity::creature::Creature;
use crate::entity::grass::Grass;
use crate::types::Position;
use crate::world::entity_map::EntityMap;

// represent the contents of a single cell in the world
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    // the cell is empty
    Empty,

    // TODO -  this should be a reference to a Creature structure
    // but I assume the borrow checker would not let me make changes to the creature
    // if it is referenced both here and in the world.creatures list.
    //
    // Possible solutions:
    //   https://doc.rust-lang.org/std/cell/index.html
    //   https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
    //   https://doc.rust-lang.org/std/rc/struct.Rc.html

    // the cell is occupied by a Creature (with a unique number)
    Creature(u64),

    // the cell is occupied by a block of grass (food for Herbivorous Creatures)
    Grass(u64),
}

// a world is a 2D grid of Cell
pub struct World {
    // the size of the 2D world (width and height are the same)
    size: u16,
    // the grid of cells - inner Vec is a row, outer Vec is a column
    cells: Vec<Vec<Cell>>,
    // the list of creatures in the world
    creatures: EntityMap<Creature>,
    // the list of all the grass blocks in the world
    grass: EntityMap<Grass>,
}

// public methods
impl World {
    pub fn new(size: u16) -> World {
        // create a square 2d vector of empty cells
        let cells = vec![vec![Cell::Empty; size as usize]; size as usize];
        let world = World {
            size,
            cells,
            creatures: EntityMap::<Creature>::new(),
            grass: EntityMap::<Grass>::new(),
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
