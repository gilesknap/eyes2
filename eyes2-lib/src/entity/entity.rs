//! Define the Entity trait, which is implemented by all types of entities
//! that can be stored in the EntityMap
//!
use direction::Coord;

use crate::world::types::UpdateQueue;
use crate::Settings;
// a trait to declare that a type is an entity that can be stored in EntityMap

pub trait Entity {
    // static methods
    fn new(coord: Coord, config: Settings) -> Self;

    // property getters
    fn id(&self) -> u64;
    fn coord(&self) -> Coord;

    // property setters
    fn move_to(&mut self, coord: Coord);
    fn set_id(&mut self, id: u64);

    // instance methods
    fn tick(&mut self, queue: &mut UpdateQueue);
}

// represent the contents of a single cell in the world
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    // the cell is empty
    Empty,

    // the cell is occupied by a Creature (with a unique number)
    Entity(u64),

    // the cell is occupied by a block of grass
    Grass,
}
