//! represents a creature in the world that can eat grass and reproduce
//!
mod code;
use super::entity::{Entity, Update, UpdateQueue};
use crate::settings::Settings;
use crate::utils::random_direction;
use code::Processor;
use direction::Direction;
use fastrand::Rng as FastRng;

#[derive(Debug)]
pub struct Creature {
    id: u64,
    code: Processor,
    config: Settings,
    rng: FastRng,
}

impl Entity for Creature {
    fn new(config: Settings) -> Creature {
        let rng = FastRng::new();
        let (b, e) = config.creature_initial_energy;
        let energy = rng.i32(b..e);
        Creature {
            id: 0,
            code: Processor::new(energy),
            rng,
            config,
        }
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        // id is immutable once set
        if self.id == 0 {
            self.id = id;
        }
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue)
    }
}

impl Creature {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        self.code.energy -= self.config.creature_idle_energy;

        self.code.tick();

        if self.code.energy <= 0 {
            queue.push(Update::RemoveEntity(self.id));
        } else if self.code.energy >= self.config.creature_reproduction_energy {
            self.reproduce(queue);
        } else if self.rng.f32() <= self.config.creature_move_rate {
            let direction = random_direction(&self.rng);

            self.code.energy -= self.config.creature_move_energy;
            queue.push(Update::MoveEntity(self.id(), direction));
        }
    }

    pub fn eat(&mut self, amount: i32) {
        self.code.energy += amount;
    }

    pub fn reproduce(&mut self, queue: &mut UpdateQueue) {
        let mut child = Creature::new(self.config);
        self.code.energy /= 2;
        child.code.energy = self.code.energy;

        // spawn a child to our left
        queue.push(Update::AddEntity(self.id, child, Direction::West));
    }
}
