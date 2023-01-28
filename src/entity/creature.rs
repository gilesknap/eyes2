//! represents a creature in the world that can eat grass and reproduce
//!
mod code;

use super::{Cell, Entity};
use crate::settings::Settings;
use crate::utils::move_pos;
use crate::world::{Update, UpdateQueue};
use code::Processor;
use direction::{Coord, Direction};
use queues::*;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};

#[derive(Clone)]
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
        self.code.energy -= 1;

        self.code.tick();

        if self.code.energy == 0 {
            // TODO not happy about this clone here just to get a callback to remove
            queue.add(Update::RemoveCreature(self.clone())).ok();
        } else if self.rng.gen_range(0.0..1.0) <= self.config.creature_move_rate {
            let direction: Direction = self.rng.sample(Standard);
            let new_pos = move_pos(self.coord, direction, self.config.size);
            // TODO this also has clone
            queue.add(Update::MoveCreature(self.clone(), new_pos)).ok();
        }
    }

    pub fn eat(&mut self, amount: u32) {
        self.code.energy += amount;
    }

    // TODO: looks like we have a reproduction capability - we need a way to call
    // this from the genome code ...
    // (this passing of a Entity via the Queue has already been proven for Grass)
    pub fn _reproduce(&mut self, queue: &mut UpdateQueue) {
        let child = Creature::new(0, self.coord, self.config);
        queue.add(Update::AddCreature(child)).ok();
    }
}
