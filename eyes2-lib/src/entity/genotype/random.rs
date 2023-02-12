//! Implement the 'RISC Instruction Set' for 'creatures'
//!

// TODO I've noticed that the original eyes creature does not have any RAM other than
// its 5 registers - might be nice to add some?

const GENOME: usize = 1000;

#[derive(Debug)]
pub struct RandomGenotype {
    pub energy: i32, // energy level
}

impl RandomGenotype {
    pub fn new(energy: i32) -> RandomGenotype {
        RandomGenotype { energy }
    }

    pub fn randomize() -> [u16; GENOME] {
        let mut genome = [0; GENOME];
        for i in 0..genome.len() {
            genome[i] = fastrand::u16(..);
        }
        genome
    }

    // totally dummy instruction set for now
    pub fn tick(&mut self) {}
}
