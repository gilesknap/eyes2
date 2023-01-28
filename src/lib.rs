// Just declare the submodules

pub mod settings;
// represent individual entities in the world
mod entity;
// inspect the world and render it (using curses?) also handle user input
pub mod gui;
// inspect the world and render it (using curses?) also handle user input
pub mod tui;
// represent the state of the world as a grid of cells
pub mod world;
// standalone utility functions
pub mod utils;
