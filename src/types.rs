use std::rc::Rc;

// represent a coordinate in the world
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

// represent a direction in the 2d world
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// represent the contents of a single cell in the world
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    // the cell is empty
    Empty,

    // the cell is occupied by a Creature (with a unique number)
    Creature(u64),

    // the cell is occupied by a block of grass (with a unique number)
    Grass(u64),
}

// a reference counted pointer to a 2d vector of cells
pub type WorldGrid = Rc<Vec<Vec<Cell>>>;
