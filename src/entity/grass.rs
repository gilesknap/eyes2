use super::{Cell, Entity};

pub struct Grass {}

impl Entity for Grass {
    fn new() -> Grass {
        Grass {}
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Grass(id)
    }
}
