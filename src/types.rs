// represent a coordinate in the world
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

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
