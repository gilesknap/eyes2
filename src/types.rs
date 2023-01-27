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

static DIRECTIONS: [(&str, i8, i8); 8] = [
    ("North", 0, -1),
    ("NorthEast", 1, -1),
    ("East", 1, 0),
    ("SouthEast", 1, 1),
    ("South", 0, 1),
    ("SouthWest", -1, 1),
    ("West", -1, 0),
    ("NorthWest", -1, -1),
];
