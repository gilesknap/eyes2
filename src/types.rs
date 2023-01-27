//! Simple shared structures that all implement Copy and Clone
//!
//!

use direction::Coord;

/// represent a change to the world
#[derive(Debug, Copy, Clone)]
pub enum Update {
    AddCreature(Coord),
    MoveCreature(u64, Coord),
    AddGrass(Coord),
    RemoveCreature(u64),
    RemoveGrass(u64),
}
