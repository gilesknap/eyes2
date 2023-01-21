use std::sync::atomic::{AtomicU64, Ordering};

use crate::types::Position;

#[derive(Debug, Copy, Clone)] // TODO I'd like to avoid making this copyable
pub struct Creature {
    // the creature's unique number
    pub num: u64,
    // the creature's position in the world
    pub position: Position,
    // the creature's current energy level
    pub energy: u32,
    // the creature's current direction
}

impl Creature {
    pub fn new(position: Position, energy: u32) -> Creature {
        static NEXT_NUM: AtomicU64 = AtomicU64::new(1);
        Creature {
            num: NEXT_NUM.fetch_add(1, Ordering::Relaxed),
            position,
            energy,
        }
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }
}
