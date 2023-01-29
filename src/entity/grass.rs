//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::settings::Settings;
use crate::utils::{move_pos, rotate_direction};
use crate::world::Update;
use crate::world::UpdateQueue;
use direction::{Coord, Direction};
use queues::*;

pub struct Grass {
    id: u64,
    coord: Coord,
    next_grow_dir: Direction,
    config: Settings,
}

impl Entity for Grass {
    fn new(id: u64, coord: Coord, config: Settings) -> Grass {
        Grass {
            id,
            coord,
            config,
            next_grow_dir: Direction::North,
        }
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

    fn set_id(&mut self, id: u64) {
        // id is immutable once set
        if self.id == 0 {
            self.id = id;
        }
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    // Grass tick causes it to grow every grow_interval ticks. Note that the
    // algorithm goes to pains not to call rand every tick, because this
    // gets called a lot. Instead we loop over directions and interval counters
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        let new_coord = move_pos(self.coord, self.next_grow_dir, self.config.size);

        queue.add(Update::AddGrass(self.id, new_coord)).ok();

        self.next_grow_dir = rotate_direction(self.next_grow_dir);
    }

    pub fn _grow(&mut self, coord: Coord) -> Grass {
        Grass {
            id: 0,
            coord,
            config: self.config,
            next_grow_dir: self.next_grow_dir,
        }
    }
}
