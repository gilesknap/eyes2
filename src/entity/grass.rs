//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::{Position, Update};
use crate::utils::{move_pos, random_direction};
use crate::world::UpdateQueue;
use queues::*;

pub struct Grass {
    id: u64,
    position: Position,
    config: Settings,
}

impl Entity for Grass {
    fn new(id: u64, position: Position, config: Settings) -> Grass {
        Grass {
            id,
            position,
            config,
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
        let new_pos = move_pos(self.position, random_direction(), self.config.size);
        queue.add(Update::AddGrass(new_pos)).ok();
    }
}
