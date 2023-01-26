//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::{Position, Update};
use crate::utils::{int_to_direction, move_pos};
use crate::world::UpdateQueue;
use queues::*;
use rand::Rng;

pub struct Grass {
    id: u64,
    position: Position,
    config: Settings,
    rng: rand::rngs::ThreadRng,
}

impl Entity for Grass {
    fn new(id: u64, position: Position, config: Settings) -> Grass {
        Grass {
            id,
            position,
            config,
            rng: rand::thread_rng(),
        }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Grass(id)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn position(&self) -> Position {
        self.position
    }

    fn move_to(&mut self, pos: Position) {
        self.position = pos;
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        let dir = self.rng.gen_range(0..8);
        let new_pos = move_pos(self.position, int_to_direction(dir), self.config.size);
        queue.add(Update::AddGrass(new_pos)).ok();
    }
}
