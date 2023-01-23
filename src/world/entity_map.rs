// Define a generic structure to hold the entities in the world. One of these
// would be created for each type of entity (creatures, grass, possibly others).
// The entity is held in a hash map, with unique id as key,
// this is for fast lookup and iteration, the hash map owns the entity.
//
// An entity id is also held in the world grid, so that the
// inspecting a cell in the world can quickly find and entity. This
// is held using a Cell enum, which contains the unique id of the
// entity. This approach means there is only one reference to the entity
// and that can be borrowed from the hashmap as needed.
//
// We are storing the entity id instead of an Entity reference because I want
// to eventually make this multi process, The world grid will be shared between
// processes with transactional changes allowed, but the entities will be
// owned by the process that created them.

use rand::Rng;

use crate::entity::{Cell, Entity};
use crate::types::{Position, WorldGrid};
use std::collections::HashMap;

pub struct EntityItem<T> {
    pub id: u64,
    pub position: Position,
    pub entity: T,
}

pub struct EntityMap<T> {
    entities: HashMap<u64, T>,
    next_id: u64,
    grid: WorldGrid,
    grid_size: u16,
}

impl<T> EntityMap<T>
where
    T: Entity,
{
    pub fn new(grid: WorldGrid) -> EntityMap<T> {
        let grid_size = (grid.len() as f64).sqrt() as u16;
        EntityMap {
            entities: HashMap::new(),
            next_id: 0,
            grid,
            grid_size,
        }
    }

    pub fn populate(&mut self, count: u16) {
        for _ in 0..count {
            loop {
                let entity = T::new();
                // find a random empty cell to place the entity
                let x = rand::thread_rng().gen_range(0..self.grid_size);
                let y = rand::thread_rng().gen_range(0..self.grid_size);
                if let Ok(()) = self.add_entity(entity, Position { x, y }) {
                    break;
                };
            }
        }
    }

    pub fn get_entity(&self, id: u64) -> Option<&T> {
        self.entities.get(&id)
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }

    pub fn add_entity(&mut self, entity: T, position: Position) -> Result<(), ()> {
        match self.grid[position.x as usize][position.y as usize] {
            Cell::Empty => {
                let id = self.next_id;
                self.next_id += 1;
                // TODO this fails as Rc is not mutable
                // Looks like I need to use RefCell but it would be nice to design
                // a pattern that does not require that.
                self.grid[position.x as usize][position.y as usize] = T::cell_type(id);
                self.entities.insert(id, entity);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn remove_entity(&mut self, id: u64) -> Result<(), ()> {
        // propagate an error if entity not found
        match self.entities.remove(&id) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }
}
