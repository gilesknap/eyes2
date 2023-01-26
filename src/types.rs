//! Simple shared structures that all implement Copy and Clone
//!
//!

/// represent a coordinate in the world
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// represent a change to the world
#[derive(Debug, Copy, Clone)]
pub enum Update {
    AddCreature(Position),
    MoveCreature(u64, Position),
    AddGrass(Position),
    RemoveCreature(u64),
    RemoveGrass(u64),
}

/// represent a direction in the 2d world
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
