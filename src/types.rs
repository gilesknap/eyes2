use crate::entity::Cell;
use std::{cell::RefCell, rc::Rc};

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

// a reference counted pointer to Reference Cell of a 2d vector of cells
// TODO replace RefCell with Arc when we have multiple threads
//
// The outer Rc allows us to share the RefCell between multiple owners.
// The RefCell allows us to mutate the contents of the Vec from any of
// these owners. At present this is safe as we are single threaded.
pub type WorldGrid = Rc<RefCell<Vec<Vec<Cell>>>>;
