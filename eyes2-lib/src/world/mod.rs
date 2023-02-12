// re-export API structures to the world module root
pub use self::grid::{Cell, WorldGrid};
pub use self::world::World;

pub mod grid;
pub mod world;

#[cfg(test)]
mod tests;
