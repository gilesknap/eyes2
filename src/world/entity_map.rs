// Define a structure to hold the entities in the world. One of these
// would be created for each type of entity (creatures, grass, possibly others).
// The entity is held in a hash map, with unique id as key,
// this is for fast lookup and iteration, the hash map owns the entity.
//
// An entity reference is also held in the world grid, so that the
// inspecting a cell in the world can quickly find and entity. This
// is held using a Cell enum, which contains the unique id of the
// entity. This approach means there is only one reference to the entity
// and that can be borrowed from the hashmap as needed.
//
// TODO storing a the entity id in the world grid means that we need a HashMap
// lookup to find the entity. Consider storing a reference to the entity in the
// world grid instead. But this would require a reference counted pointer to the
// entity or some other mechanism.
// TODO worth remembering that I want to make this multi-process one day
// and the grid and entity map will need to be shared between processes.
// so above TODO gets more complicated.

use crate::entity::Entity;
use std::collections::HashMap;

pub struct EntityMap<T> {
    entities: HashMap<u64, T>,
    next_id: u64,
}

impl<T> EntityMap<T>
where
    T: Entity,
{
    pub fn new() -> EntityMap<T> {
        EntityMap {
            entities: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn populate(&mut self, count: u16) {
        for _ in 0..count {
            loop {
                let entity = T::new();
                if let Ok(()) = self.add_entity(entity) {
                    break;
                }
            }
        }
    }

    pub fn get_entity(&self, id: u64) -> Option<&T> {
        self.entities.get(&id)
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }

    pub fn add_entity(&mut self, entity: T) -> Result<(), ()> {
        // TODO check if the grid is empty at the entity's position
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, entity);
        Ok(())
    }

    pub fn remove_entity(&mut self, id: u64) -> Result<(), ()> {
        // propagate an error if entity not found
        match self.entities.remove(&id) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }
}
