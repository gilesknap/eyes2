//! Implement the 'RISC Instruction Set' for 'creatures'
//!

// TODO I've noticed that the original eyes creature does not have any RAM other than
// its 5 registers - might be nice to add some?

use crate::entity::update::UpdateQueue;
use crate::entity::Genotype;

#[derive(Debug)]
pub struct RandomGenotype {
    pub energy: i32, // energy level
}

impl Genotype for RandomGenotype {
    fn tick(&mut self, _queue: &mut UpdateQueue) {
        self.energy -= 1;
    }
}

impl RandomGenotype {
    pub fn _new(energy: i32) -> RandomGenotype {
        RandomGenotype { energy }
    }
}
