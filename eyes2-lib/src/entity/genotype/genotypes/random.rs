//! Implement the Random genotype, which is a creature that moves randomly and reproduces
//! when it has enough energy.

use super::{Genotype, GenotypeActions};
use crate::utils::random_direction;
use crate::Settings;
use direction::Direction;
use fastrand::Rng as FastRng;

#[derive(Clone, Debug)]
pub struct RandomGenomeType {
    config: Settings,
    energy: i32,
    rng: FastRng,
    direction: Direction,
}

impl Genotype for RandomGenomeType {
    fn tick(&mut self) -> GenotypeActions {
        if self.energy >= self.config.creature_reproduction_energy {
            return GenotypeActions::Reproduce(Box::new(self.reproduce()));
        }

        if self.rng.f32() <= self.config.creature_move_rate {
            self.direction = random_direction(&self.rng);
            return GenotypeActions::Move(self.direction);
        }

        GenotypeActions::None
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }

    fn get_sigil(&self) -> char {
        'R'
    }

    fn get_name(&self) -> String {
        "random".to_string()
    }
}

impl RandomGenomeType {
    pub fn new(config: Settings) -> RandomGenomeType {
        RandomGenomeType {
            config,
            energy: 0,
            rng: FastRng::new(),
            direction: Direction::North,
        }
    }

    pub fn reproduce(&mut self) -> Self {
        self.energy -= self.config.creature_reproduction_energy;
        self.clone()
    }
}
