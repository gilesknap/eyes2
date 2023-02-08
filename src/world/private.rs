use crate::entity::Entity;
use crate::entity::{grass::Grass, Cell};
use direction::Coord;

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
