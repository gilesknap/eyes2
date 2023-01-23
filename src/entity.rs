// a trait to declare that a type is an entity that can be stored in EntityMap
pub mod creature;
pub mod grass;

pub trait Entity {
    fn new() -> Self;
}
