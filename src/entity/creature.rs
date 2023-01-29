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
use std::rc::Rc;

#[derive(Debug)]
pub struct Creature {
    id: u64,
    coord: Coord,
    code: Processor,
    config: Settings,
    rng: rand::rngs::StdRng,
}

impl Entity for Creature {
    fn new(id: u64, coord: Coord, config: Settings) -> Creature {
        let mut rng = StdRng::from_entropy();
        let (b, e) = config.creature_initial_energy;
        let energy = rng.gen_range(b..e);
        Creature {
            id,
            coord,
            code: Processor::new(energy),
            rng,
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
        self.code.energy -= self.config.creature_idle_energy;

        self.code.tick();

        if self.code.energy == 0 {
            queue
                .add(Update::RemoveCreature(self.id, self.coord()))
                .ok();
        } else if self.rng.gen_range(0.0..1.0) <= self.config.creature_move_rate {
            let direction: Direction = self.rng.sample(Standard);
            let new_pos = move_pos(self.coord, direction, self.config.size);
            queue
                .add(Update::MoveCreature(self.id(), self.coord(), new_pos))
                .ok();
        }
    }

    pub fn eat(&mut self, amount: u32) {
        self.code.energy += amount;
    }

    // I'm pretty sure this gives us reproduction capability - now we need a
    // way to call this from the genome code ...
    pub fn _reproduce(&mut self, queue: &mut UpdateQueue) {
        let child = Creature::new(0, self.coord, self.config);
        queue.add(Update::AddCreature(Rc::new(child))).ok();
    }
}
