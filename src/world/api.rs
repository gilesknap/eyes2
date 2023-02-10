use crate::entity::{
    creature::Creature,
    entity::{Cell, Entity},
};
use crate::settings::Settings;
use direction::Coord;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};
use std::collections::HashMap;

use super::types::{Update, UpdateQueue, World};

// public static methods
impl World {
    pub fn new(config: Settings) -> World {
        // create a square 2d vector of empty cells
        let grid = vec![Cell::Empty; config.size.pow(2) as usize];
        config.size as usize;

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            grid,
            creatures: HashMap::<u64, Creature>::new(),
            updates: UpdateQueue::new(),
            ticks: 0,
            config,
            grass_rate: config.grass_interval,
            next_grass_tick: 0,
            grass_count: 0,
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

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn grass_count(&self) -> usize {
        self.grass_count as usize
    }

    pub fn creature_count(&self) -> usize {
        self.creatures.len()
    }

    pub fn grass_rate(&self) -> u64 {
        self.grass_rate
    }

    pub fn increment_grass_rate(&mut self, up: bool) {
        if up {
            self.grass_rate += 1;
        } else {
            self.grass_rate -= 1;
        }
        self.grass_rate = self.grass_rate.clamp(1, 100)
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.grass_count as usize {
            let x = self.rng.gen_range(0..self.config.size - 1) as i32;
            let y = self.rng.gen_range(0..self.config.size - 1) as i32;
            self.add_grass(Coord { x, y });
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

    /// read a cell from the grid - used for rendering the world
    #[inline(always)]
    pub fn get_cell(&self, position: Coord) -> Cell {
        // TODO Currently using Copy to return this - maybe should switch to
        // using a Box? Then could remove the Copy trait from Cell
        self.grid[(position.x + position.y * self.config.size as i32) as usize]
    }

    #[inline(always)]
    pub fn set_cell(&mut self, position: Coord, value: Cell) {
        self.grid[(position.x + position.y * self.config.size as i32) as usize] = value;
    }
}
