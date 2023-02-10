use crate::{
    entity::{entity::Cell, entity::Entity},
    utils::rotate_direction,
};
use direction::{Coord, Direction};

use super::types::{Update, World};

/// This is where world services requests from Entities to make changes to
/// the world.
impl World {
    pub(super) fn get_next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub(super) fn validate_creature(&self, id: u64, coord: Coord) {
        // TODO How do I pass the type to use in the match so this can be
        // used for Cell::Grass too ??
        let cell = self.grid.get_cell(coord);
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
        // TODO divide by number of grass
        if self.ticks >= self.next_grass_tick {
            self.grow_grass();
            self.next_grass_tick += self.ticks_per_grass();
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
                    let cell = self.grid.get_cell(coord);
                    match cell {
                        Cell::Empty => {
                            self.creatures.insert(id, creature);
                        }
                        Cell::Grass => {
                            self.creatures.insert(id, creature); // TODO consider factoring out this repetition
                            self.eat_grass(coord, id);
                        }
                        _ => continue, // skip add if there is already a creature in the cell
                    };
                    self.grid.set_cell(coord, Cell::Creature(id));
                }
                Update::RemoveCreature(id, coord) => {
                    self.validate_creature(id, coord);
                    self.creatures.remove(&id);
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
}

// grass methods
impl World {
    fn ticks_per_grass(&self) -> u64 {
        // ticks per grass growth is between 100 to 1,000,000 in inverse
        // logarithmic proportion to grass
        (101 - self.grass_rate as u64).pow(2) * 100
    }
    pub(super) fn grow_grass(&mut self) {
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
                        grow_dir = rotate_direction(grow_dir);
                    }
                    _ => {}
                }
            }
        }

        for coord in new_grass {
            self.grid.add_grass(coord);
        }
    }

    pub(super) fn eat_grass(&mut self, coord: Coord, id: u64) {
        self.grid.remove_grass(coord);
        self.creatures
            .get_mut(&id)
            .unwrap()
            .eat(self.config.grass_energy);
    }
}
