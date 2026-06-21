//! The `giles` creature controller.
//!
//! A faithful port of the evolving RISC byte-code genome from the original 1999
//! `eyes` project, split across:
//!
//! - [`isa`] - the instruction set definitions shared by all parts
//! - [`genotype`] - the virtual machine / [`Genotype`](super::super::Genotype) implementation
//! - [`asm`] - the disassembler and assembler for the genome

pub mod asm;
pub mod genotype;
pub mod isa;

pub use genotype::GilesGenotype;
