//! represents a creature in the world that can eat grass and reproduce
//!
mod code;

use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::{Position, Update};
use crate::utils::{move_pos, random_direction};
use crate::world::UpdateQueue;
use code::Processor;
use queues::*;
use rand::Rng;

pub struct Creature {
    id: u64,
    position: Position,
    code: Processor,
    config: Settings,
    rng: rand::rngs::ThreadRng,
}

impl Entity for Creature {
    fn new(id: u64, position: Position, config: Settings) -> Creature {
        Creature {
            id,
            position,
            code: Processor::new(),
            rng: rand::thread_rng(),
            config,
        }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Creature(id)
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
        self.tick(queue)
    }
}

impl Creature {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        self.code.energy -= 1;

        self.code.tick();

        if self.code.energy == 0 {
            queue.add(Update::RemoveCreature(self.id)).ok();
        } else if self.rng.gen_range(0.0..1.0) <= self.config.creature_move_rate {
            // random creature movement for now

            let new_pos = move_pos(self.position, random_direction(), self.config.size);
            queue.add(Update::MoveCreature(self.id, new_pos)).ok();
        }
    }

    pub fn eat(&mut self, amount: u32) {
        self.code.energy += amount;
    }

    pub fn _reproduce(&mut self, _queue: &mut UpdateQueue) {
        let _child = Creature::new(self.id + 1, self.position, self.config);
        // TODO this is no good as we need to get next id from the world
        // how to do that and need a thread safe way to do it for the future
    }
}
