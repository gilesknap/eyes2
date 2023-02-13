/// Specification of the communication protocol between the Creatures
/// and the World.
use super::{creature::Creature, genotype::genotypes::random::RandomGenomeType};
use direction::{Coord, Direction};

/// Represent the possible world update service requests that
/// Creatures can place on the update queue.
pub enum Update {
    AddEntity(Creature<RandomGenomeType>),
    MoveEntity(u64, Coord, Coord),
    RemoveEntity(u64, Coord),
    Look(u64, Direction),
}
