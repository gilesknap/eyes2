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
pub use crate::world::{save_world, Cell, World, WorldGrid};

/// Tooling for the `giles` genome: its instruction set ([`giles::isa`]) and a
/// disassembler / assembler ([`giles::asm`]) for converting between the raw
/// genome bytes and a human readable listing.
pub mod giles {
    pub use crate::entity::genotype::genotypes::giles::{asm, isa, GilesGenotype};
}
