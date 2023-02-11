use crate::entity::{creature::Creature, entity::Entity};
use crate::settings::Settings;
use direction::Coord;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};
use std::collections::HashMap;

use super::{
    grid::WorldGrid,
    types::{Update, UpdateQueue, World},
};

// public static methods
impl World {
    pub fn new(config: Settings) -> World {
        // create a square 2d vector of empty cells
        let grid = WorldGrid::new(config.size, config.grass_rate, config.speed);

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            grid,
            creatures: HashMap::<u64, Creature>::new(),
            updates: UpdateQueue::new(),
            config,
            next_grass_tick: 0,
            rng: StdRng::from_entropy(),
            next_id: 0,
        };

        world
    }
}

// public instance methods
impl World {
    pub fn get_size(&self) -> u16 {
        self.config.size
    }

    pub fn creature_count(&self) -> u64 {
        self.creatures.len() as u64
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.grass_count as usize {
            let x = self.rng.gen_range(0..self.config.size - 1) as i32;
            let y = self.rng.gen_range(0..self.config.size - 1) as i32;
            self.grid.add_grass(Coord { x, y });
        }
        for _ in 0..self.config.creature_count as usize {
            let x = self.rng.gen_range(0..self.config.size) as i32;
            let y = self.rng.gen_range(0..self.config.size) as i32;

            let id = self.get_next_id();
            let creature = Creature::new(id, Coord { x, y }, self.config.clone());
            self.updates.push(Update::AddCreature(creature));
        }
        self.apply_updates();
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        self.do_tick();
    }
}
