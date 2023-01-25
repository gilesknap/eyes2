//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::types::{Position, Update};
use crate::utils::{move_pos, random_direction};
use crate::world::UpdateQueue;
use queues::*;
use rand::Rng;

pub struct Grass {
    id: u64,
    position: Position,
}

impl Entity for Grass {
    fn new(id: u64, position: Position) -> Grass {
        Grass { id, position }
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

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        if rand::thread_rng().gen_range(0..500) == 0 {
            // grow a new grass block
            let new_pos = move_pos(self.position, random_direction());
            queue.add(Update::AddGrass(new_pos)).ok();
        }
    }
}
