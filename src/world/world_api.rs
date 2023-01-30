use super::World;
use super::{Update, UpdateQueue};
use crate::entity::{creature::Creature, grass::Grass, Cell, Entity};
use crate::settings::Settings;
use direction::Coord;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};
use std::cmp;
use std::collections::HashMap;
use std::f64::MAX_EXP;

// public static methods
impl World {
    pub fn new(config: Settings) -> World {
        // create a square 2d vector of empty cells
        let grid = vec![vec![Cell::Empty; config.size as usize]; config.size as usize];

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            grid,
            creatures: HashMap::<u64, Creature>::new(),
            grass: HashMap::<u64, Grass>::new(),
            updates: UpdateQueue::new(),
            ticks: 0,
            config,
            next_grass_tick: 0,
            rng: StdRng::from_entropy(),
            next_id: 0,
        };

        println!("Created a new world of size {} square", world.config.size);
        world
    }
}

// public instance methods
impl World {
    pub fn get_size(&self) -> i32 {
        self.config.size
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn grass_count(&self) -> usize {
        self.grass.len()
    }

    pub fn creature_count(&self) -> usize {
        self.creatures.len()
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.grass_count as usize {
            let x = rand::thread_rng().gen_range(0..self.config.size) as i32;
            let y = rand::thread_rng().gen_range(0..self.config.size) as i32;

            let id = self.get_next_id();
            self.updates.push(Update::AddGrass(id, Coord { x, y }));
        }
        for _ in 0..self.config.creature_count as usize {
            let x = rand::thread_rng().gen_range(0..self.config.size) as i32;
            let y = rand::thread_rng().gen_range(0..self.config.size) as i32;

            let id = self.get_next_id();
            let creature = Creature::new(id, Coord { x, y }, self.config.clone());
            self.updates.push(Update::AddCreature(creature));
        }
        self.apply_updates();

        println!(
            "Added {} grass and {} creatures to the world",
            self.config.grass_count, self.config.creature_count
        );
    }

    pub fn tick(&mut self) {
        for creature in self.creatures.values_mut() {
            creature.tick(&mut self.updates);
        }

        // limit calls to grass tick relative to grass_interval
        if self.ticks >= self.next_grass_tick && !self.grass.is_empty() {
            // pick a random grass block to grow
            // TODO - need to work out how to do this without cloning the keys
            // TODO - and without traversing the entire map to get this one item
            // I believe that IndexMap might be the answer
            // https://users.rust-lang.org/t/random-entry-of-hashmap/26548/4
            let keys: Vec<u64> = self.grass.keys().cloned().collect();
            let which = self.rng.gen_range(0..self.grass.len());

            let grass = self.grass.get_mut(&keys[which]).unwrap();
            grass.tick(&mut self.updates);

            // Calculate the next tick for grass. It is grass interval
            // divided by the number of grass blocks, but capped at
            // max_grass_per_interval
            self.next_grass_tick = match self.grass.len() {
                0 => MAX_EXP as u64,
                _ => {
                    self.ticks
                        + self.config.grass_interval
                            / cmp::min(
                                self.grass.len(),
                                self.config.max_grass_per_interval as usize,
                            ) as u64
                }
            }
        }

        self.apply_updates();
        self.ticks += 1;
    }

    /// read a cell from the grid - used for rendering the world
    pub fn get_cell(&self, position: Coord) -> Cell {
        // TODO Currently using Copy to return this - maybe should switch to
        // using a Box? Then could remove the Copy trait from Cell
        return self.grid[position.x as usize][position.y as usize];
    }
}
