use super::{Cell, Entity};

pub enum Status {
    Alive,
    Dead,
}

#[derive(Debug)] // TODO I'd like to avoid making this copyable
pub struct Creature {
    // the creature's current energy level
    pub energy: u32,
}

impl Entity for Creature {
    fn new() -> Creature {
        Creature { energy: 1000 }
    }
    fn cell_type(id: u64) -> Cell {
        Cell::Creature(id)
    }
}

impl Creature {
    pub fn tick(&mut self) -> Status {
        self.energy -= 1;
        // TODO: execute the next instruction in the creature's program
        match self.energy {
            0 => Status::Dead,
            _ => Status::Alive,
        }
    }
}
