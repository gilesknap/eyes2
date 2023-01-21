use crate::creature::Creature;
use crate::types::Position;
use rand;
use std::collections::HashMap;

// represent the contents of a single cell in the world
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    // the cell is empty
    Empty,
    // the cell is occupied by a Creature
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
    // TODO: make this private and complete the accessor methods
    pub creatures: HashMap<u64, Creature>,
}

impl World {
    pub fn new(size: u16) -> World {
        // create a square 2d vector of empty cells
        let cells = vec![vec![Cell::Empty; size as usize]; size as usize];
        let world = World {
            size,
            cells,
            creatures: HashMap::new(),
        };

        print!("Created a new world of size {} square\n", world.size);
        world
    }

    pub fn get_cell(&self, position: Position) -> Cell {
        return self.cells[position.x as usize][position.y as usize];
    }

    pub fn set_cell(&mut self, position: Position, cell: Cell) {
        self.cells[position.x as usize][position.y as usize] = cell;
    }

    pub fn add_creature(&mut self, creature: Creature) -> bool {
        match self.get_cell(creature.position) {
            Cell::Empty => {
                self.set_cell(creature.position, Cell::Creature(creature.num));
                self.creatures.insert(creature.num, creature);
                true
            }
            _ => false,
        }
    }

    pub fn add_grass(&mut self, position: Position) -> bool {
        match self.get_cell(position) {
            Cell::Empty => {
                self.set_cell(position, Cell::Grass);
                true
            }
            _ => false,
        }
    }

    pub fn populate(&mut self, grass_count: u16, creature_count: u16, energy: u32) {
        // add grass
        for _ in 0..grass_count {
            let x = rand::random::<u16>() % self.size;
            let y = rand::random::<u16>() % self.size;
            let position = Position { x, y };
            self.add_grass(position);
        }

        // add creatures
        for _ in 0..creature_count {
            let x = rand::random::<u16>() % self.size;
            let y = rand::random::<u16>() % self.size;
            let position = Position { x, y };
            let creature = Creature::new(position, energy);
            self.add_creature(creature);
        }

        print!(
            "Added {} grass and {} creatures to the world",
            grass_count, creature_count
        );
    }
}
