/// Specification of the communication protocol between the Creatures
/// and the World.
use super::creature::Creature;
use direction::{Coord, Direction};

/// a queue of updates to the world to be applied at the end of the tick
pub type UpdateQueue = Vec<Update>;

/// Represent the possible world update service requests that
/// Creatures can place on the update queue.
pub enum Update {
    AddEntity(Creature),
    MoveEntity(u64, Coord, Coord),
    RemoveEntity(u64, Coord),
    Look(u64, Direction),
}
