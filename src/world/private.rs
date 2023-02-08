use crate::entity::{entity::Cell, entity::Entity, grass::Grass};
use direction::Coord;
use rand::prelude::*;
use std::cmp;
use std::f64::MAX_EXP;

use super::types::{Update, World};

/// This is where world services requests from Entities to make changes to
/// the world.
impl World {
    pub(super) fn get_next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub(super) fn eat_grass(&mut self, grass_id: u64, id: u64) {
        self.grass.remove(&grass_id);
        self.creatures
            .get_mut(&id)
            .unwrap()
            .eat(self.config.grass_energy);
    }

    pub(super) fn validate_creature(&self, id: u64, coord: Coord) {
        // TODO How do I pass the type to use in the match so this can be
        // used for Cell::Grass too ??
        let cell = self.grid[coord.x as usize][coord.y as usize];
        match cell {
            // TODO I'm going to treat these as panic for now. But maybe once we go multithread there may
            // be requests from creatures that have not yet responded to deletion
            Cell::Creature(match_id) => {
                if match_id != id {
                    panic!("creature id does not match world grid");
                }
            }
            _ => panic!("no creature in world at grid coordinate"),
        };
    }

    pub(super) fn do_tick(&mut self) {
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
                // when there is no grass left never call grass tick again
                0 => MAX_EXP as u64,
                // Otherwise calculate the next tick on which we will call grass tick.
                // We calculate it as between 1000 ticks and 100,000 ticks per grass block
                // inversely proportional to grass_rate of 1-100
                _ => {
                    let total_ticks = (101 - self.grass_rate) * 1000;
                    let div = cmp::min(
                        self.grass.len(),
                        self.config.max_grass_per_interval as usize,
                    ) as u64;

                    self.ticks + total_ticks / div
                }
            }
        }

        self.apply_updates();
        self.ticks += 1;
    }

    /// process the updates to the world that have been queued in the previous tick
    pub(super) fn apply_updates(&mut self) {
        // TODO is this the best way to iterate over all items in a queue?
        while self.updates.len() > 0 {
            let update = self.updates.remove(0);
            match update {
                Update::AddCreature(creature) => {
                    let coord = creature.coord();
                    let id = creature.id();
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Empty => {
                            self.creatures.insert(id, creature);
                        }
                        Cell::Grass(grass_id) => {
                            self.creatures.insert(id, creature); // TODO consider factoring out this repetition
                            self.eat_grass(grass_id, id);
                        }
                        _ => continue, // skip add if there is already a creature in the cell
                    };
                    self.grid[coord.x as usize][coord.y as usize] = Cell::Creature(id);
                }
                Update::AddGrass(_id, coord) => {
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Empty => {
                            // TODO should call grow here (using id to get grass to grow)
                            // ANS: In fact we can do that in the grass tick and pass the new
                            // grass object using the pattern tested in AddCreature
                            let id = self.get_next_id();
                            let grass = Grass::new(id, coord, self.config);
                            self.grass.insert(id, grass);
                            self.grid[coord.x as usize][coord.y as usize] = Cell::Grass(id)
                        }
                        _ => continue,
                    };
                }
                Update::RemoveCreature(id, coord) => {
                    self.validate_creature(id, coord);
                    self.creatures.remove(&id);
                    self.grid[coord.x as usize][coord.y as usize] = Cell::Empty;
                }
                Update::RemoveGrass(id, coord) => {
                    let cell = self.grid[coord.x as usize][coord.y as usize];
                    match cell {
                        Cell::Grass(grass_id) => {
                            if grass_id == id {
                                self.grass.remove(&id);
                                self.grid[coord.x as usize][coord.y as usize] = Cell::Empty;
                            }
                        }
                        _ => panic!("no grass in world at grid coordinate"),
                    };
                }
                Update::MoveCreature(id, old_coord, new_coord) => {
                    self.validate_creature(id, old_coord);
                    let cell = self.grid[new_coord.x as usize][new_coord.y as usize];
                    match cell {
                        Cell::Empty => {}
                        Cell::Grass(grass_id) => self.eat_grass(grass_id, id),
                        // skip move if there is already a creature in the cell
                        Cell::Creature(_) => continue,
                    }
                    self.creatures.get_mut(&id).unwrap().move_to(new_coord);
                    let grid = &mut self.grid;
                    grid[old_coord.x as usize][old_coord.y as usize] = Cell::Empty;
                    grid[new_coord.x as usize][new_coord.y as usize] = Cell::Creature(id);
                }
            }
        }
    }
}
