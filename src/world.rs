mod thing;
use crate::creature::{Creature, Status};
use crate::types::Position;
use rand;
use std::collections::{HashMap, HashSet};

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
    Grass,
}

// a world is a 2D grid of Cell
pub struct World {
    // the size of the world (width and height are the same)
    size: u16,
    // the grid of cells - inner Vec is a row, outer Vec is a column
    cells: Vec<Vec<Cell>>,
    // the list of creatures in the world
    creatures: HashMap<u64, Creature>,
    // the list of all the grass blocks in the world
    grass: HashSet<Position>,
}

// public methods
impl World {
    pub fn new(size: u16) -> World {
        // create a square 2d vector of empty cells
        let cells = vec![vec![Cell::Empty; size as usize]; size as usize];
        let grass = HashSet::new();
        let world = World {
            size,
            cells,
            creatures: HashMap::new(),
            grass,
        };

        println!("Created a new world of size {} square", world.size);
        world
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn grass_count(&self) -> usize {
        self.grass.len()
    }

    pub fn creature_count(&self) -> usize {
        self.creatures.len()
    }

    pub fn populate(&mut self, grass_count: u16, creature_count: u16, energy: u32) {
        // add grass
        for _ in 0..grass_count {
            loop {
                let x = rand::random::<u16>() % self.size;
                let y = rand::random::<u16>() % self.size;
                // use of concise flow control (see Chapter 6)
                if let Ok(()) = self.add_grass(Position { x, y }) {
                    break;
                }
            }
        }

        // add creatures
        for _ in 0..creature_count {
            loop {
                let x = rand::random::<u16>() % self.size;
                let y = rand::random::<u16>() % self.size;
                let creature = Creature::new(Position { x, y }, energy);
                if let Ok(()) = self.add_creature(creature) {
                    break;
                }
            }
        }

        println!(
            "Added {} grass and {} creatures to the world",
            grass_count, creature_count
        );
    }

    // get creature by its number
    pub fn creature(&self, num: u64) -> Option<&Creature> {
        self.creatures.get(&num)
    }

    // get creature by its position
    pub fn creature2(&self, x: u16, y: u16) -> Option<&Creature> {
        match self.get_cell(Position { x, y }) {
            Cell::Creature(num) => self.creatures.get(&num),
            _ => None,
        }
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

// private methods
impl World {
    fn set_cell(&mut self, position: Position, cell: Cell) {
        self.cells[position.x as usize][position.y as usize] = cell;
    }

    fn add_creature(&mut self, creature: Creature) -> Result<(), ()> {
        match self.get_cell(creature.position) {
            Cell::Empty => {
                self.set_cell(creature.position, Cell::Creature(creature.num));
                self.creatures.insert(creature.num, creature);
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn remove_creature(&mut self, creature: &Creature) -> Result<(), ()> {
        match self.get_cell(creature.position) {
            Cell::Creature(num) => {
                if num == creature.num {
                    self.set_cell(creature.position, Cell::Empty);
                    self.creatures.remove(&creature.num);
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }

    fn add_grass(&mut self, position: Position) -> Result<(), ()> {
        match self.get_cell(position) {
            Cell::Empty => {
                self.set_cell(position, Cell::Grass);
                self.grass.insert(position);
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn remove_grass(&mut self, position: Position) -> Result<(), ()> {
        match self.get_cell(position) {
            Cell::Grass => {
                self.set_cell(position, Cell::Empty);
                self.grass.remove(&position);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
#[cfg(test)]
mod tests;
