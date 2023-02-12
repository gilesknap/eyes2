/// A few simple types used for communicating with the world
use crate::entity::creature::Creature;
use direction::Coord;

// a queue of updates to the world to be applied at the end of the tick
// Note I did not use queues crate because it clones the objects in the
// Queue and we specifically want to pass object ownership for e.g.
// AddCreature(Creature)
pub type UpdateQueue = Vec<Update>;

/// Represent the possible world update service requests that
/// Entities can place on the update queue.
pub enum Update {
    AddCreature(Creature),
    MoveCreature(u64, Coord, Coord),
    RemoveCreature(u64, Coord),
}
