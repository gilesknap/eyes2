//! Simple shared structures that all implement Copy and Clone
//!
//!

use direction::Coord;

use crate::entity::{creature::Creature, grass::Grass};

/// represent a change to the world
#[derive(Clone)]
pub enum Update {
    AddCreature(Creature),
    MoveCreature(u64, Coord),
    AddGrass(Grass),
    RemoveCreature(u64),
    RemoveGrass(u64),
}
