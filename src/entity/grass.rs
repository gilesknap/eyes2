//! represents a grass block in the world that can grow new grass blocks
//! nearby at random depending on light levels
//!
use super::{Cell, Entity};
use crate::settings::Settings;
use crate::types::Update;
use crate::utils::move_pos;
use crate::world::UpdateQueue;
use direction::{Coord, DirectionIter};
use queues::*;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};

pub struct Grass {
    id: u64,
    coord: Coord,
    next_grow_dir: DirectionIter,
    grow_interval: u16,
    config: Settings,
}

impl Entity for Grass {
    fn new(id: u64, coord: Coord, config: Settings) -> Grass {
        let mut rng = StdRng::from_entropy();
        // first gen grass has random interval between 50% and 100% of config.grass_interval
        let interval = rng.gen_range(config.grass_interval / 2..config.grass_interval);

        Grass {
            id,
            coord,
            config,
            next_grow_dir: DirectionIter::new(),
            grow_interval: interval,
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

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    // Grass tick causes it to grow every grow_interval ticks. Note that the
    // algorithm goes to pains not to call rand every tick, because this
    // gets called a lot. Instead we loop over directions and interval counters
    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        if self.grow_interval > 0 {
            self.grow_interval -= 1;
        } else {
            self.grow_interval = self.config.grass_interval;

            // TODO is this ugly or is it elegant? I'm on the fence on this one.
            let grow_dir = match self.next_grow_dir.next() {
                Some(dir) => dir,
                None => {
                    self.next_grow_dir = DirectionIter::new();
                    self.next_grow_dir.next().unwrap()
                }
            };

            let new_coord = move_pos(self.coord, grow_dir, self.config.size);
            let _new_grass = self.grow(new_coord);
            queue.add(Update::AddGrass(new_coord)).ok();
        }
    }

    pub fn grow(&mut self, coord: Coord) -> Grass {
        Grass {
            id: 0,
            coord,
            config: self.config,
            next_grow_dir: self.next_grow_dir.clone(),
            grow_interval: self.config.grass_interval,
        }
    }
}
