//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::Update;
use crate::utils::move_pos;
use crate::world::UpdateQueue;
use direction::{Coord, Direction};
use queues::*;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};

pub struct Grass {
    id: u64,
    coord: Coord,
    config: Settings,
}

impl Entity for Grass {
    fn new(id: u64, coord: Coord, config: Settings) -> Grass {
        Grass { id, coord, config }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Grass(id)
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
        self.tick(queue);
    }
}

impl Grass {
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        let direction: Direction = StdRng::from_entropy().sample(Standard);
        let new_pos = move_pos(self.coord, direction, self.config.size);
        queue.add(Update::AddGrass(new_pos)).ok();
    }
}
