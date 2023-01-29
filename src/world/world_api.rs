use super::entity_map::EntityMap;
use super::World;
use super::{Update, UpdateQueue};
use crate::entity::{creature::Creature, grass::Grass, Cell, Entity};
use crate::settings::Settings;
use direction::Coord;
use queues::*;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng};
use std::cmp;
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
            creatures: EntityMap::<Creature>::new(config),
            grass: EntityMap::<Grass>::new(config),
            updates: UpdateQueue::new(),
            ticks: 0,
            config,
            next_grass_tick: 0,
            rng: StdRng::from_entropy(),
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
        self.grass.count()
    }

    pub fn creature_count(&self) -> usize {
        self.creatures.count()
    }

    pub fn populate(&mut self) {
        for _ in 0..self.config.grass_count as usize {
            let x = rand::thread_rng().gen_range(0..self.config.size) as i32;
            let y = rand::thread_rng().gen_range(0..self.config.size) as i32;

            self.updates.add(Update::AddGrass(0, Coord { x, y })).ok();
        }
        for _ in 0..self.config.creature_count as usize {
            let x = rand::thread_rng().gen_range(0..self.config.size) as i32;
            let y = rand::thread_rng().gen_range(0..self.config.size) as i32;

            let creature = Creature::new(0, Coord { x, y }, self.config.clone());
            self.updates.add(Update::AddCreature(creature)).ok();
        }
        self.apply_updates();

        println!(
            "Added {} grass and {} creatures to the world",
            self.config.grass_count, self.config.creature_count
        );
    }

    pub fn tick(&mut self) {
        let ids: Vec<u64> = self.creatures.keys();

        for id in ids {
            self.creatures.get_entity(&id).tick(&mut self.updates);
        }

        // limit calls to grass tick relative to grass_interval
        if self.ticks >= self.next_grass_tick {
            let ids = self.grass.keys();

            // pick a random grass block to grow
            let which = self.rng.gen_range(0..ids.len());
            // let which = ids.len() - 1;
            self.grass.get_entity(&ids[which]).tick(&mut self.updates);

            self.next_grass_tick = match self.grass.count() {
                0 => MAX_EXP as u64,
                _ => {
                    self.ticks
                        + self.config.grass_interval
                            / cmp::min(
                                self.grass.count(),
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
        return self.grid[position.x as usize][position.y as usize];
    }
}
