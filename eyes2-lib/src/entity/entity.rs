//! Define the Entity trait, which is implemented by all types of entities
//! that can be stored in the EntityMap
//!
use super::Creature;
use crate::Settings;
use direction::Coord;

/// each type of entity must implement this trait
pub trait Entity
where
    Self: Sized,
{
    // constructor
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

/// Each type of entity must use the an UpdateQueue to communicate with
/// the world. The world will process the queue at the end of each tick.

// a queue of updates to the world to be applied at the end of the tick
// Note I did not use queues crate because it clones the objects in the
// Queue and we specifically want to pass object ownership for e.g.
// AddEntity(Entity)
pub type UpdateQueue = Vec<Update>;

/// Represent the possible world update service requests that
/// Entities can place on the update queue.
pub enum Update {
    AddEntity(Creature),
    MoveEntity(u64, Coord, Coord),
    RemoveEntity(u64, Coord),
}
