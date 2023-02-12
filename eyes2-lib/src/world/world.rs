use crate::entity::{Cell, Creature, Entity};
use crate::settings::Settings;
use crate::utils;
use direction::{Coord, Direction};
use fastrand::Rng as FastRng;
use std::collections::HashMap;

use super::{
    grid::WorldGrid,
    types::{Update, UpdateQueue},
};

// a world is a 2D grid of Cell plus a HashMap of creatures and grass blocks
// using fields to give visibility to the rest of the world module
pub struct World {
    // the grid of cells
    pub grid: WorldGrid,
    // the list of creatures in the world
    creatures: HashMap<u64, Creature>,
    // queue of updates to the world to be applied at the end of the tick
    updates: UpdateQueue,
    // the settings for the world
    config: Settings,
    // track when we will next call grass tick
    next_grass_tick: u64,
    // a random number generator
    rng: fastrand::Rng,
    // next unique id to assign to an Entity
    next_id: u64,
}

// public static methods
impl World {
    pub fn new(config: Settings, restarts: u64) -> World {
        // create a square 2d vector of empty cells
        let grid = WorldGrid::new(config.size, config.grass_rate, config.speed, restarts);

        // the grid is wrapped in a RefCell so that we can mutate it
        // this in turn is wrapped in an Rc so that we can share it
        // between multiple owners
        let world = World {
            grid,
            creatures: HashMap::<u64, Creature>::new(),
            updates: UpdateQueue::new(),
            config,
            next_grass_tick: 0,
            rng: FastRng::new(),
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
            let x = self.rng.i32(0..self.config.size as i32 - 1);
            let y = self.rng.i32(0..self.config.size as i32 - 1);
            self.grid.add_grass(Coord { x, y });
        }
        for _ in 0..self.config.creature_count as usize {
            let x = self.rng.i32(0..self.config.size as i32);
            let y = self.rng.i32(0..self.config.size as i32);

            let creature = Creature::new(Coord { x, y }, self.config.clone());
            self.updates.push(Update::AddCreature(creature));
        }
        self.apply_updates();
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        self.do_tick();
    }
}

/// internal implementation details of the World object
impl World {
    fn do_tick(&mut self) {
        for creature in self.creatures.values_mut() {
            creature.tick(&mut self.updates);
        }

        // limit calls to grass tick relative to grass_rate
        if self.grid.ticks >= self.next_grass_tick {
            self.grow_grass();
            self.next_grass_tick += self.ticks_per_grass();
        }

        self.apply_updates();
        self.grid.ticks += 1;
    }

    /// process the updates to the world that have been queued in the previous tick
    fn apply_updates(&mut self) {
        while self.updates.len() > 0 {
            let update = self.updates.pop().unwrap();
            match update {
                Update::AddCreature(mut creature) => {
                    let coord = creature.coord();
                    let id = self.get_next_id();
                    creature.set_id(id);
                    let cell = self.grid.get_cell(coord);
                    match cell {
                        Cell::Empty => {
                            self.creatures.insert(id, creature);
                            self.grid.creature_count = self.creature_count();
                        }
                        Cell::Grass => {
                            self.creatures.insert(id, creature); // TODO how to out this repetition?
                            self.grid.creature_count = self.creature_count();
                            self.eat_grass(coord, id);
                        }
                        _ => continue, // skip add if there is already a creature in the cell
                    };
                    self.grid.set_cell(coord, Cell::Creature(id));
                }
                Update::RemoveCreature(id, coord) => {
                    self.validate_creature(id, coord);
                    self.creatures.remove(&id);
                    self.grid.creature_count = self.creature_count();
                    self.grid.set_cell(coord, Cell::Empty);
                }
                Update::MoveCreature(id, old_coord, new_coord) => {
                    self.validate_creature(id, old_coord);
                    let cell = self.grid.get_cell(new_coord);
                    match cell {
                        Cell::Empty => {}
                        Cell::Grass => self.eat_grass(new_coord, id),
                        // skip move if there is already a creature in the cell
                        Cell::Creature(_) => continue,
                    }
                    self.creatures.get_mut(&id).unwrap().move_to(new_coord);
                    self.grid.set_cell(old_coord, Cell::Empty);
                    self.grid.set_cell(new_coord, Cell::Creature(id));
                }
            }
        }
    }

    fn get_next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    fn validate_creature(&self, id: u64, coord: Coord) {
        let cell = self.grid.get_cell(coord);
        match cell {
            // TODO I'm going to treat these as panic for now. But maybe once we go multithread there may
            // be requests from creatures that have not yet realized they were deleted
            Cell::Creature(match_id) => {
                if match_id != id {
                    panic!("creature id does not match world grid");
                }
            }
            _ => panic!("no creature in world at grid coordinate"),
        };
    }
}

// grass methods
impl World {
    fn ticks_per_grass(&self) -> u64 {
        // ticks per grass growth is between 100 to 1,000,000 in inverse
        // logarithmic proportion to grass_rate parameter
        (101 - self.grid.grass_rate as u64).pow(2) * 100
    }

    fn grow_grass(&mut self) {
        // walk through all the cells in the grid except the edges and grow grass
        // adjacent to cells that already have grass
        let mut grow_dir = Direction::North;
        let mut new_grass: Vec<Coord> = Vec::new();

        for x in 1..self.config.size as i32 - 2 {
            for y in 1..self.config.size as i32 - 2 {
                let coord = Coord::new(x, y);
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Grass => {
                        new_grass.push(coord + grow_dir.coord());
                        grow_dir = utils::rotate_direction(grow_dir);
                    }
                    _ => {}
                }
            }
        }

        for coord in new_grass {
            self.grid.add_grass(coord);
        }
    }

    fn eat_grass(&mut self, coord: Coord, id: u64) {
        self.grid.remove_grass(coord);
        self.creatures
            .get_mut(&id)
            .unwrap()
            .eat(self.config.grass_energy);
    }
}