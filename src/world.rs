use crate::creature::Creature;
use std::collections::HashMap;

// represent the contents of a single cell in the world
enum Cell {
    // the entry is empty
    Empty,
    // the entry is occupied by a Creature
    Occupied { creature_num: u64 },
    // the entry is occupied by a block of grass (food for Herbivorous Creatures)
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
}

impl World {
    pub fn new(size: u16) -> World {
        let mut cells = Vec::new();
        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                row.push(Cell::Empty);
            }
            cells.push(row)
        }
        World {
            size,
            cells,
            creatures: HashMap::new(),
        }
    }

    pub fn add_creature(&mut self, creature: Creature) {
        let creature_num = creature.num;
        let (x, y) = creature.position;
        self.cells[x as usize][y as usize] = Cell::Occupied { creature_num };
        self.creatures.insert(creature_num, creature);
    }
}
