use super::Entity;

pub struct Grass {}

impl Entity for Grass {
    fn new() -> Grass {
        Grass {}
    }
}
