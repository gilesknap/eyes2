//! Implement the 'RISC Instruction Set' for 'creatures'
//!

// TODO I've noticed that the original eyes creature does not have any RAM other than
// its 5 registers - might be nice to add some?

use crate::entity::Genotype;
use crate::utils::random_direction;
use crate::Settings;
use fastrand::Rng as FastRng;

#[derive(Debug)]
pub struct RandomGenotype {
    config: Settings,
    energy: i32,
    rng: FastRng,
}

impl Genotype for RandomGenotype {
    fn tick(&mut self) {
        if self.energy >= self.config.creature_reproduction_energy {
            self.reproduce();
        } else if self.rng.f32() <= self.config.creature_move_rate {
            let direction = random_direction(&self.rng);
        }
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }
}

impl RandomGenotype {
    pub fn new(config: Settings) -> RandomGenotype {
        RandomGenotype {
            config,
            energy: 0,
            rng: FastRng::new(),
        }
    }

    pub fn reproduce(&mut self) {
        self.energy -= self.config.creature_reproduction_energy;
        // TODO
    }
}
