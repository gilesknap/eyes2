//! represents a creature in the world that can eat grass and reproduce
//!
use super::{Cell, Entity};
use crate::types::{Position, Update};
use crate::utils::{move_pos, random_direction};
use crate::world::UpdateQueue;
use queues::*;
use rand::Rng;

#[derive(Debug)] // TODO I'd like to avoid making this copyable
pub struct Creature {
    id: u64,
    position: Position,
    // the creature's current energy level
    pub energy: u32,
}

impl Entity for Creature {
    fn new(id: u64, position: Position) -> Creature {
        Creature {
            id,
            position,
            energy: rand::thread_rng().gen_range(10000..20000),
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
        self.energy -= 1;
        // TODO: execute the next instruction in the creature's program

        if self.energy == 0 {
            queue.add(Update::RemoveCreature(self.id)).ok();
        } else if rand::thread_rng().gen_range(0..500) == 0 {
            // random creature movement for now
            let new_pos = move_pos(self.position, random_direction());
            queue.add(Update::MoveCreature(self.id, new_pos)).ok();
        }
    }

    pub fn eat(&mut self, amount: u32) {
        self.energy += amount;
    }
}
