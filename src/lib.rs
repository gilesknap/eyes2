extern crate pancurses;
extern crate queues;
extern crate rand;
// Just declare the submodules

// implement the assembler disassembler and executor for the RISC `processor`
mod code;
// represent individual entities in the world
mod entity;
// inspect the world and render it (using curses?) also handle user input
pub mod gui;
// some common types used throughout the project
pub mod types;
// represent the state of the world as a grid of cells
pub mod world;
// standalone utility functions
pub mod utils;
