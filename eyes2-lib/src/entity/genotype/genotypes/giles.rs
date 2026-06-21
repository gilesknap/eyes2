//! A faithful re-implementation of the evolving "RISC" genome that drove the
//! herbivores in my original `eyes` project from 1999.
//!
//! The original source is here: <https://github.com/gilesknap/eyes> (see
//! `engine.cpp` / `guts.cpp`). Each creature carries a block of random bytes
//! (`CODE_SIZE` of them) which is interpreted as a tiny byte-code program for a
//! virtual machine with a single accumulator register `r`, five general purpose
//! I/O registers and an instruction pointer `ip`. One instruction is executed
//! per world tick, exactly as the original executed one instruction per creature
//! turn.
//!
//! When a creature reproduces the genome is copied to the child. With a
//! probability driven by the (itself evolvable) `mutation_rate`, the copy is
//! mutated: individual bytes flip and the copy point can jump around the parent
//! genome, which gives gene duplication / rearrangement. The breeding threshold
//! and the mutation rate are part of the genome too, so they evolve alongside
//! the code. Out of this random soup useful survival strategies emerge.
//!
//! # Adaptation to eyes2
//!
//! The original VM read its surroundings synchronously from the global universe,
//! up to four cells deep in each of the eight directions. eyes2 instead exposes
//! a single adjacent cell in each direction via the asynchronous Look action
//! ([`GenotypeActions::Look`]). We therefore cache the last vision returned by
//! the world and refresh it after every move. The eight vision variables
//! (`V1`..`V8`) read from that cache.

use super::{Genotype, GenotypeActions};
use crate::utils::int_to_dir;
use crate::{entity::Vision, Cell, Settings};
use direction::Direction;
use serde::{Deserialize, Serialize};

/// number of bytes in a genome (matches the original `CODE_SIZE`)
const CODE_SIZE: usize = 1000;

/// number of distinct instructions in the VM
const NUMBER_OF_INSTRUCTIONS: u8 = 12;
/// total number of readable variables (vision + state + registers)
const NUMBER_OF_VARS: u8 = 18;
/// number of writable I/O registers (`I1`..`I5`)
const NUMBER_OF_IO_VARS: usize = 5;

// the instruction set, values must match the byte interpreted by the VM
const LOADC: u8 = 0; // load a constant into the accumulator
const LOADV: u8 = 1; // load a variable into the accumulator
const ANDV: u8 = 2; // bitwise AND the accumulator with a variable
const ORV: u8 = 3; // bitwise OR the accumulator with a variable
const JZ: u8 = 4; // jump if the accumulator is zero
const JNZ: u8 = 5; // jump if the accumulator is non-zero
const MOVV: u8 = 6; // move in the direction held in a variable
const MOVC: u8 = 7; // move in a constant direction
const NOP: u8 = 8; // do nothing
const SAVEV: u8 = 9; // save the accumulator into an I/O register
const ADDV: u8 = 10; // add a variable to the accumulator
const SUBV: u8 = 11; // subtract a variable from the accumulator

// the readable variable indices (V1..V8 are the eight vision directions)
const VAR_V1: u8 = 0; // first vision direction
const VAR_V8: u8 = 7; // last vision direction
const VAR_E: u8 = 8; // energy
const VAR_X: u8 = 9; // x position (internal dead-reckoning)
const VAR_Y: u8 = 10; // y position (internal dead-reckoning)
const VAR_B: u8 = 11; // breed threshold
const VAR_M: u8 = 12; // mutation rate
const VAR_I1: u8 = 13; // first I/O register

// bounds the evolving breed threshold and mutation rate are clamped to so that
// the population can never freeze (mutation_rate of 0) or breed for free
const MIN_MUTATION_RATE: u32 = 1;
const MAX_MUTATION_RATE: u32 = 100;
const DEFAULT_MUTATION_RATE: u32 = 5;

#[derive(Serialize, Deserialize, Clone)]
pub struct GilesGenotype {
    // global settings, used for the world size and the seed breed threshold
    #[serde(skip)]
    config: Settings,
    // the genome: a block of bytes interpreted as VM byte-code
    code: Vec<u8>,
    // instruction pointer into `code`
    ip: usize,
    // the accumulator register
    r: u16,
    // the five writable I/O registers (I1..I5)
    vars: [u16; NUMBER_OF_IO_VARS],
    // a dead-reckoning estimate of our position, used as an evolvable input
    // signal. It does not track the true world coordinate (the genotype is not
    // told that) but gives the genome a consistent sense of where it has moved.
    x: i32,
    y: i32,
    // the amount of energy we believe we have (refreshed by the world on eat)
    energy: i32,
    // energy threshold at which we reproduce (evolvable)
    breed_after: i32,
    // percentage chance of mutating the genome on reproduction (evolvable)
    mutation_rate: u32,
    // the last view of the world returned by a Look, encoded one value per
    // direction. Refreshed after every move.
    vision: [u16; 8],
    // when true we spend the next tick looking to refresh `vision`
    pending_look: bool,
}

#[typetag::serde(name = "giles_genotype")]
impl Genotype for GilesGenotype {
    fn tick(&mut self) -> GenotypeActions {
        // refresh our perception of the world after moving (or on the very
        // first tick) - the world's Look is asynchronous so it costs a tick
        if self.pending_look {
            self.pending_look = false;
            return GenotypeActions::Look;
        }

        // reproduce once we have banked enough energy
        if self.energy >= self.breed_after {
            return GenotypeActions::Reproduce(Box::new(self.reproduce()));
        }

        // otherwise execute exactly one instruction of our genome
        match self.execute() {
            Some(direction) => {
                // having moved, our surroundings have changed - look again
                self.pending_look = true;
                GenotypeActions::Move(direction)
            }
            None => GenotypeActions::None,
        }
    }

    fn vision(&mut self, vision: Vision) {
        for (i, cell) in vision.iter().enumerate() {
            self.vision[i] = encode_cell(cell);
        }
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }

    fn get_sigil(&self) -> char {
        'G'
    }
}

impl GilesGenotype {
    pub fn new(config: Settings) -> GilesGenotype {
        let breed_after = config.creature_reproduction_energy;
        let code = (0..CODE_SIZE).map(|_| fastrand::u8(..)).collect();

        GilesGenotype {
            config,
            code,
            ip: 0,
            r: 0,
            vars: [0; NUMBER_OF_IO_VARS],
            x: 0,
            y: 0,
            energy: 0,
            breed_after,
            mutation_rate: DEFAULT_MUTATION_RATE,
            vision: [0; 8],
            // look before we leap
            pending_look: true,
        }
    }

    /// Produce a child genotype, mutating its genome with probability
    /// `mutation_rate`%. The world is the authority on creature energy and
    /// splits it between parent and child in [`Creature::reproduce`]; we halve
    /// our internal estimate to mirror that so the parent drops back below the
    /// breed threshold and does not immediately try to breed again (the same
    /// purpose served by `random.rs` subtracting its reproduction energy).
    fn reproduce(&mut self) -> Self {
        self.energy /= 2;

        // the child inherits our (halved) energy and genome via the clone
        let mut child = self.clone();
        // but starts its program from the beginning with clear registers
        child.ip = 0;
        child.r = 0;
        child.vars = [0; NUMBER_OF_IO_VARS];
        child.pending_look = true;

        if fastrand::u32(0..100) < self.mutation_rate {
            child.mutate();
        }

        child
    }

    /// Mutate the child's evolvable parameters and genome in place, following
    /// the scheme from the original `MutateAnimal`.
    fn mutate(&mut self) {
        // nudge the breed threshold by +/- (mutation_rate / 2) percent
        self.breed_after += self.breed_after * self.signed_mutation_percent() / 100;
        // and likewise the mutation rate itself
        let new_rate = self.mutation_rate as i32
            + self.mutation_rate as i32 * self.signed_mutation_percent() / 100;
        self.mutation_rate = (new_rate.max(0) as u32).clamp(MIN_MUTATION_RATE, MAX_MUTATION_RATE);

        // keep the breed threshold within sane bounds relative to the configured
        // reproduction energy so a lineage can never reproduce for free
        let seed = self.config.creature_reproduction_energy.max(1);
        self.breed_after = self.breed_after.clamp(seed / 10, seed * 10);

        // copy the genome over itself with per-byte mutation and a roving copy
        // point that gives gene duplication / rearrangement
        let parent = self.code.clone();
        let mut copy_from = 0usize;
        for j in 0..CODE_SIZE {
            self.code[j] = parent[copy_from];
            copy_from += 1;
            // deliberately rare: scale the genome mutation chance down 200-fold
            if fastrand::u32(0..20000) < self.mutation_rate {
                self.code[j] = fastrand::u8(..);
            }
            if fastrand::u32(0..20000) < self.mutation_rate {
                copy_from = fastrand::usize(0..CODE_SIZE);
            }
            if copy_from >= CODE_SIZE {
                copy_from = 0;
            }
        }
    }

    /// A signed mutation amount in the range [-rate/2, rate/2) percent.
    fn signed_mutation_percent(&self) -> i32 {
        let rate = self.mutation_rate.max(1) as i32;
        fastrand::i32(0..rate) - rate / 2
    }

    /// Execute a single VM instruction. Returns `Some(direction)` if the
    /// instruction was a move, otherwise `None` (the creature rests this tick).
    fn execute(&mut self) -> Option<Direction> {
        // remember where this instruction started for relative jumps
        let instruction_addr = self.ip;
        let instruction = self.next_byte() % NUMBER_OF_INSTRUCTIONS;

        match instruction {
            LOADC => self.r = self.get_constant(),
            LOADV => self.r = self.get_variable(),
            ANDV => self.r &= self.get_variable(),
            ORV => self.r |= self.get_variable(),
            ADDV => self.r = self.r.wrapping_add(self.get_variable()),
            SUBV => self.r = self.r.wrapping_sub(self.get_variable()),
            SAVEV => self.set_variable(self.r),
            JZ => {
                let target = self.get_constant();
                if self.r == 0 {
                    self.ip = (instruction_addr + target as usize) % CODE_SIZE;
                }
            }
            JNZ => {
                let target = self.get_constant();
                if self.r != 0 {
                    self.ip = (instruction_addr + target as usize) % CODE_SIZE;
                }
            }
            MOVV => {
                let v = self.get_variable();
                return Some(self.move_in_direction(v));
            }
            MOVC => {
                let v = self.get_constant();
                return Some(self.move_in_direction(v));
            }
            NOP => {}
            // % NUMBER_OF_INSTRUCTIONS guarantees we never reach here
            _ => unreachable!(),
        }

        None
    }

    /// Read the next genome byte, advancing (and wrapping) the instruction
    /// pointer. The original wrapped once per instruction with a 3-byte guard;
    /// wrapping on every byte here is simpler and can never index out of
    /// bounds, at the cost of letting an instruction's operand straddle the end
    /// of the genome (a harmless behavioural difference at the boundary).
    fn next_byte(&mut self) -> u8 {
        let byte = self.code[self.ip % CODE_SIZE];
        self.ip = (self.ip + 1) % CODE_SIZE;
        byte
    }

    /// Read a little-endian 16 bit constant from the genome.
    fn get_constant(&mut self) -> u16 {
        let lo = self.next_byte() as u16;
        let hi = self.next_byte() as u16;
        lo.wrapping_add(hi.wrapping_mul(256))
    }

    /// Read the value of the variable selected by the next genome byte.
    fn get_variable(&mut self) -> u16 {
        let var = self.next_byte() % NUMBER_OF_VARS;
        match var {
            // vision directions V1..V8
            VAR_V1..=VAR_V8 => self.vision[(var - VAR_V1) as usize],
            VAR_E => self.energy as u16,
            VAR_X => self.x as u16,
            VAR_Y => self.y as u16,
            VAR_B => self.breed_after as u16,
            VAR_M => self.mutation_rate as u16,
            // I/O registers I1..I5
            _ => self.vars[(var - VAR_I1) as usize],
        }
    }

    /// Write the accumulator into the I/O register selected by the next byte.
    fn set_variable(&mut self, value: u16) {
        let var = self.next_byte() as usize % NUMBER_OF_IO_VARS;
        self.vars[var] = value;
    }

    /// Turn a raw value into a move in one of the eight directions, updating our
    /// dead-reckoning position estimate.
    fn move_in_direction(&mut self, value: u16) -> Direction {
        let index = (value % 8) as i32;
        let direction = int_to_dir(index);
        let size = self.config.size as i32;
        let step = direction.coord();
        self.x = (self.x + step.x).rem_euclid(size);
        self.y = (self.y + step.y).rem_euclid(size);
        direction
    }
}

/// Encode a single world cell into the numeric value the VM's vision variables
/// read. The values mirror the original universe ids: empty space is 0, grass is
/// the attractive 1, another creature 2 and an impassable wall 4.
fn encode_cell(cell: &Cell) -> u16 {
    match cell {
        Cell::Empty => 0,
        Cell::Grass => 1,
        Cell::Entity(_, _) => 2,
        Cell::Wall => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_genotype() -> GilesGenotype {
        GilesGenotype::new(Settings::default())
    }

    #[test]
    fn get_constant_is_little_endian() {
        let mut g = test_genotype();
        g.code[0] = 0x34;
        g.code[1] = 0x12;
        g.ip = 0;
        assert_eq!(g.get_constant(), 0x1234);
        assert_eq!(g.ip, 2);
    }

    #[test]
    fn movc_returns_a_move_and_advances_perception() {
        let mut g = test_genotype();
        // a genome that is a single MOVC instruction heading East (index 2)
        g.code[0] = MOVC;
        g.code[1] = 2;
        g.code[2] = 0;
        g.ip = 0;
        g.pending_look = false;
        g.energy = 0;
        g.breed_after = i32::MAX;

        match g.tick() {
            GenotypeActions::Move(dir) => assert_eq!(dir, Direction::East),
            _ => panic!("expected a Move action"),
        }
        // after moving we should want to look again next tick
        assert!(g.pending_look);
        assert!(matches!(g.tick(), GenotypeActions::Look));
    }

    #[test]
    fn nop_rests_without_moving() {
        let mut g = test_genotype();
        g.code.iter_mut().for_each(|b| *b = NOP);
        g.ip = 0;
        g.pending_look = false;
        g.breed_after = i32::MAX;
        assert!(matches!(g.tick(), GenotypeActions::None));
    }

    #[test]
    fn reproduce_when_energy_reaches_threshold() {
        let mut g = test_genotype();
        g.pending_look = false;
        g.breed_after = 1000;
        g.energy = 1000;
        match g.tick() {
            GenotypeActions::Reproduce(_) => {}
            _ => panic!("expected a Reproduce action"),
        }
        // reproducing should have halved our stored energy
        assert_eq!(g.energy, 500);
    }

    #[test]
    fn save_and_load_register_round_trips() {
        let mut g = test_genotype();
        // LOADC 0x002a ; SAVEV I1
        g.code[0] = LOADC;
        g.code[1] = 0x2a;
        g.code[2] = 0x00;
        g.code[3] = SAVEV;
        g.code[4] = 0; // I1
        g.ip = 0;
        g.execute(); // LOADC
        assert_eq!(g.r, 0x2a);
        g.execute(); // SAVEV I1
        assert_eq!(g.vars[0], 0x2a);
    }

    #[test]
    fn mutation_preserves_genome_length_and_bounds() {
        let mut g = test_genotype();
        g.mutation_rate = MAX_MUTATION_RATE;
        g.mutate();
        assert_eq!(g.code.len(), CODE_SIZE);
        // mutation rate must stay within bounds so evolution never freezes
        assert!(g.mutation_rate >= MIN_MUTATION_RATE);
        assert!(g.mutation_rate <= MAX_MUTATION_RATE);
    }

    #[test]
    fn random_genomes_never_panic() {
        // run many random genomes for many ticks to flush out any indexing or
        // arithmetic panics in the VM
        for _ in 0..200 {
            let mut g = test_genotype();
            g.pending_look = false;
            g.breed_after = i32::MAX;
            for _ in 0..2000 {
                g.execute();
            }
        }
    }
}
