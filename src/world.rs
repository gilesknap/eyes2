use crate::creature::Creature;
use crate::types::Position;
use std::collections::HashMap;

// A world is a 2D grid but instead of storing the grid as a 2D array we store
// the creatures and grass blocks it contains in hashmaps with the position
// as the key. All positions that are not in a hashmap are empty.
pub struct World {
    // the size of the world (width and height are the same)
    size: u16,
    // the set of creatures in the world
    creatures: HashMap<Position, Creature>,
    // the set of grass blocks in the world
    grass: HashMap<Position, Creature>,
}

impl World {
    pub fn new(size: u16) -> World {
        World {
            size,
            creatures: HashMap::new(),
            grass: HashMap::new(),
        }
    }

    pub fn add_creature(&mut self, creature: Creature) {
        self.creatures.insert(creature.position, creature);
    }
}
