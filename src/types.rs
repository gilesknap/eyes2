// represent a coordinate in the world
#[derive(Debug, Copy, Clone, PartialEq)]
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
