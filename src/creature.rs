use std::sync::atomic::{AtomicU64, Ordering};

pub struct Creature {
    // the creature's unique number
    pub num: u64,
    // the creature's position in the world
    pub position: (u32, u32),
    // the creature's current energy level
    pub energy: u32,
    // the creature's current direction
}

impl Creature {
    pub fn new(position: (u32, u32), energy: u32) -> Creature {
        static NEXT_NUM: AtomicU64 = AtomicU64::new(1);
        Creature {
            num: NEXT_NUM.fetch_add(1, Ordering::Relaxed),
            position,
            energy,
        }
    }
}
