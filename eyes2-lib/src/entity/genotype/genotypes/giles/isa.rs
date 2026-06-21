//! The instruction set architecture (ISA) of the `giles` virtual machine.
//!
//! These definitions are the single source of truth shared by the VM
//! ([`super::genotype`]), the disassembler and the assembler ([`super::asm`]).
//! The numeric values must not change without regenerating any saved genomes,
//! as they are the meaning of the raw genome bytes.

/// number of bytes in a genome (matches the original `CODE_SIZE`)
pub const CODE_SIZE: usize = 1000;

/// number of distinct instructions in the VM
pub const NUMBER_OF_INSTRUCTIONS: u8 = 12;
/// total number of readable variables (vision + state + registers)
pub const NUMBER_OF_VARS: u8 = 18;
/// number of writable I/O registers (`I1`..`I5`)
pub const NUMBER_OF_IO_VARS: usize = 5;

// the instruction set, values must match the byte interpreted by the VM
pub const LOADC: u8 = 0; // load a constant into the accumulator
pub const LOADV: u8 = 1; // load a variable into the accumulator
pub const ANDV: u8 = 2; // bitwise AND the accumulator with a variable
pub const ORV: u8 = 3; // bitwise OR the accumulator with a variable
pub const JZ: u8 = 4; // jump if the accumulator is zero
pub const JNZ: u8 = 5; // jump if the accumulator is non-zero
pub const MOVV: u8 = 6; // move in the direction held in a variable
pub const MOVC: u8 = 7; // move in a constant direction
pub const NOP: u8 = 8; // do nothing
pub const SAVEV: u8 = 9; // save the accumulator into an I/O register
pub const ADDV: u8 = 10; // add a variable to the accumulator
pub const SUBV: u8 = 11; // subtract a variable from the accumulator

// the readable variable indices (V1..V8 are the eight vision directions)
pub const VAR_V1: u8 = 0; // first vision direction
pub const VAR_V8: u8 = 7; // last vision direction
pub const VAR_E: u8 = 8; // energy
pub const VAR_X: u8 = 9; // x position (internal dead-reckoning)
pub const VAR_Y: u8 = 10; // y position (internal dead-reckoning)
pub const VAR_B: u8 = 11; // breed threshold
pub const VAR_M: u8 = 12; // mutation rate
pub const VAR_I1: u8 = 13; // first I/O register

/// mnemonic for each instruction, indexed by opcode
pub const MNEMONICS: [&str; NUMBER_OF_INSTRUCTIONS as usize] = [
    "LOADC", "LOADV", "ANDV", "ORV", "JZ", "JNZ", "MOVV", "MOVC", "NOP", "SAVEV", "ADDV", "SUBV",
];

/// name for each readable variable, indexed by variable number
pub const VAR_NAMES: [&str; NUMBER_OF_VARS as usize] = [
    "V1", "V2", "V3", "V4", "V5", "V6", "V7", "V8", "E", "X", "Y", "B", "M", "I1", "I2", "I3", "I4",
    "I5",
];

/// The kind of operand (if any) that follows an instruction in the genome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    /// no operand byte(s)
    None,
    /// a 16 bit little-endian constant (two bytes)
    Constant,
    /// a one byte index selecting one of the readable variables
    Variable,
    /// a one byte index selecting one of the writable I/O registers
    IoVariable,
    /// a 16 bit little-endian value added to the instruction's own address to
    /// give a (wrapped) absolute jump target
    Jump,
}

/// The number of operand bytes that follow a given opcode in the genome.
pub fn operand_size(opcode: u8) -> usize {
    match operand_kind(opcode) {
        Operand::None => 0,
        Operand::Variable | Operand::IoVariable => 1,
        Operand::Constant | Operand::Jump => 2,
    }
}

/// Return the operand kind for a given opcode. The opcode must already be
/// reduced modulo [`NUMBER_OF_INSTRUCTIONS`].
pub fn operand_kind(opcode: u8) -> Operand {
    match opcode {
        LOADC | MOVC => Operand::Constant,
        JZ | JNZ => Operand::Jump,
        LOADV | ANDV | ORV | MOVV | ADDV | SUBV => Operand::Variable,
        SAVEV => Operand::IoVariable,
        NOP => Operand::None,
        // opcode is always reduced % NUMBER_OF_INSTRUCTIONS so this is unreachable
        _ => Operand::None,
    }
}

/// Look up an opcode by (case-insensitive) mnemonic.
pub fn opcode_from_mnemonic(name: &str) -> Option<u8> {
    MNEMONICS
        .iter()
        .position(|m| m.eq_ignore_ascii_case(name))
        .map(|i| i as u8)
}

/// Look up a readable variable index by (case-insensitive) name (`V1`..`I5`).
pub fn variable_from_name(name: &str) -> Option<u8> {
    VAR_NAMES
        .iter()
        .position(|v| v.eq_ignore_ascii_case(name))
        .map(|i| i as u8)
}

/// Look up an I/O register index (0..[`NUMBER_OF_IO_VARS`]) by name (`I1`..`I5`).
pub fn io_variable_from_name(name: &str) -> Option<u8> {
    variable_from_name(name).and_then(|v| {
        if v >= VAR_I1 {
            Some(v - VAR_I1)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_match_counts() {
        assert_eq!(MNEMONICS.len(), NUMBER_OF_INSTRUCTIONS as usize);
        assert_eq!(VAR_NAMES.len(), NUMBER_OF_VARS as usize);
    }

    #[test]
    fn mnemonic_round_trips() {
        for (i, m) in MNEMONICS.iter().enumerate() {
            assert_eq!(opcode_from_mnemonic(m), Some(i as u8));
            assert_eq!(opcode_from_mnemonic(&m.to_lowercase()), Some(i as u8));
        }
        assert_eq!(opcode_from_mnemonic("NOPE"), None);
    }

    #[test]
    fn variable_lookups() {
        assert_eq!(variable_from_name("V1"), Some(VAR_V1));
        assert_eq!(variable_from_name("e"), Some(VAR_E));
        assert_eq!(variable_from_name("I5"), Some(VAR_I1 + 4));
        assert_eq!(io_variable_from_name("I1"), Some(0));
        assert_eq!(io_variable_from_name("I5"), Some(4));
        // E is readable but not a writable I/O register
        assert_eq!(io_variable_from_name("E"), None);
    }
}
