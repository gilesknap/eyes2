use crate::world::UpdateQueue;
// a trait to declare that a type is an entity that can be stored in EntityMap
pub mod creature;
pub mod grass;

pub trait Entity {
    fn new(id: u64) -> Self;
    fn cell_type(id: u64) -> Cell;

    fn id(&self) -> u64;
    fn tick(&mut self, queue: &mut UpdateQueue);
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
