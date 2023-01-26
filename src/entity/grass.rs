//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::types::{Position, Update};
use crate::utils::random_direction;
use crate::world::UpdateQueue;
use queues::*;

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

    fn move_to(&mut self, pos: Position) {
        self.position = pos;
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        queue
            .add(Update::AddGrass(self.position, random_direction()))
            .ok();
    }
}
