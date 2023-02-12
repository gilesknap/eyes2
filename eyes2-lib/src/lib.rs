// Just declare the submodules

pub mod settings;
// represent individual entities in the world
pub mod entity;
// represent the state of the world as a grid of cells
pub mod world;
// standalone utility functions
pub mod utils;

// these are the public API structures
pub use crate::settings::Settings;
pub use crate::world::{Cell, World, WorldGrid};
