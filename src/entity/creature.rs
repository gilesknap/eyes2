//! represents a creature in the world that can eat grass and reproduce
//!
mod code;

use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::Update;
use crate::utils::move_pos;
use crate::world::UpdateQueue;
use code::Processor;
use direction::{Coord, Direction};
use queues::*;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};

pub struct Creature {
    id: u64,
    coord: Coord,
    code: Processor,
    config: Settings,
    rng: rand::rngs::StdRng,
}

impl Entity for Creature {
    fn new(id: u64, coord: Coord, config: Settings) -> Creature {
        Creature {
            id,
            coord,
            code: Processor::new(),
            rng: StdRng::from_entropy(),
            config,
        }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Creature(id)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn coord(&self) -> Coord {
        self.coord
    }

    fn move_to(&mut self, pos: Coord) {
        self.coord = pos;
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
            let direction: Direction = self.rng.sample(Standard);
            let new_pos = move_pos(self.coord, direction, self.config.size);
            queue.add(Update::MoveCreature(self.id, new_pos)).ok();
        }
    }

    pub fn eat(&mut self, amount: u32) {
        self.code.energy += amount;
    }

    pub fn _reproduce(&mut self, _queue: &mut UpdateQueue) {
        let _child = Creature::new(self.id + 1, self.coord, self.config);
        // TODO this is no good as we need to get next id from the world
        // how to do that and need a thread safe way to do it for the future
    }
}
