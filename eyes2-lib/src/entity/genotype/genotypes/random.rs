//! Implement the Random genotype, which is a creature that moves randomly and reproduces
//! when it has enough energy.

use super::{Genotype, GenotypeActions};
use crate::utils::random_direction;
use crate::Settings;
use fastrand::Rng as FastRng;

#[derive(Clone, Debug)]
pub struct RandomGenomeType {
    config: Settings,
    energy: i32,
    rng: FastRng,
}

impl Genotype for RandomGenomeType {
    fn new(config: Settings) -> RandomGenomeType {
        RandomGenomeType {
            config,
            energy: 0,
            rng: FastRng::new(),
        }
    }

    fn tick(&mut self) -> GenotypeActions<Self> {
        if self.energy >= self.config.creature_reproduction_energy {
            return GenotypeActions::Reproduce(self.reproduce());
        }

        if self.rng.f32() <= self.config.creature_move_rate {
            let direction = random_direction(&self.rng);
            return GenotypeActions::Move(direction);
        }

        GenotypeActions::None
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }
}

impl RandomGenomeType {
    pub fn reproduce(&mut self) -> Self {
        self.energy -= self.config.creature_reproduction_energy;
        self.clone()
    }
}
