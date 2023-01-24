use super::{Cell, Entity};
use crate::world::UpdateQueue;

pub struct Grass {
    id: u64,
}

impl Entity for Grass {
    fn new(id: u64) -> Grass {
        Grass { id }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Grass(id)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue);
    }
}

impl Grass {
    pub fn tick(&mut self, _queue: &mut UpdateQueue) {
        // TODO: grow the grass
    }
}
