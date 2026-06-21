//! Disassembler for the `giles` virtual machine genome.
//!
//! [`disassemble`] turns a raw genome (a block of bytes) into a human readable
//! listing, decoding each instruction exactly as the VM does: the opcode and
//! every operand selector are reduced modulo their range, so any random byte
//! block is a valid (if nonsensical) program. This is the counterpart to the
//! original project's `DisAssemble`.
//!
//! An assembler (the inverse direction) is provided in a later change.

use super::isa::*;

/// One decoded instruction in a disassembly listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisasmLine {
    /// the address (genome offset) of the instruction's opcode byte
    pub addr: usize,
    /// the instruction mnemonic, e.g. `MOVC`
    pub mnemonic: &'static str,
    /// the rendered operand, if the instruction takes one (e.g. `V1`, `0x2a`)
    pub operand: Option<String>,
}

impl std::fmt::Display for DisasmLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.operand {
            Some(operand) => write!(f, "{:04x}  {:<6}{}", self.addr, self.mnemonic, operand),
            None => write!(f, "{:04x}  {}", self.addr, self.mnemonic),
        }
    }
}

/// read a genome byte, wrapping the index exactly as the VM does so that an
/// instruction whose operand runs off the end of the genome decodes the same
/// way the VM would execute it
fn byte_at(code: &[u8], index: usize) -> u8 {
    code[index % code.len()]
}

/// read a 16 bit little-endian constant starting at `index`
fn constant_at(code: &[u8], index: usize) -> u16 {
    let lo = byte_at(code, index) as u16;
    let hi = byte_at(code, index + 1) as u16;
    lo.wrapping_add(hi.wrapping_mul(256))
}

/// Disassemble a genome into a list of decoded instructions.
pub fn disassemble(code: &[u8]) -> Vec<DisasmLine> {
    assert!(!code.is_empty(), "cannot disassemble an empty genome");

    let mut listing = Vec::new();
    let mut ip = 0usize;

    while ip < code.len() {
        let addr = ip;
        let opcode = byte_at(code, ip) % NUMBER_OF_INSTRUCTIONS;
        ip += 1;
        let mnemonic = MNEMONICS[opcode as usize];

        let operand = match operand_kind(opcode) {
            Operand::None => None,
            Operand::Variable => {
                let var = byte_at(code, ip) % NUMBER_OF_VARS;
                ip += 1;
                Some(VAR_NAMES[var as usize].to_string())
            }
            Operand::IoVariable => {
                let var = byte_at(code, ip) as usize % NUMBER_OF_IO_VARS;
                ip += 1;
                Some(VAR_NAMES[VAR_I1 as usize + var].to_string())
            }
            Operand::Constant => {
                let value = constant_at(code, ip);
                ip += 2;
                // MOVC only uses the constant modulo 8 (the eight directions)
                let value = if opcode == MOVC { value % 8 } else { value };
                Some(format!("{:#x}", value))
            }
            Operand::Jump => {
                let target = (addr + constant_at(code, ip) as usize) % CODE_SIZE;
                ip += 2;
                Some(format!("{:04x}", target))
            }
        };

        listing.push(DisasmLine {
            addr,
            mnemonic,
            operand,
        });
    }

    listing
}

/// Disassemble a genome into a newline separated string listing.
pub fn disassemble_to_string(code: &[u8]) -> String {
    disassemble(code)
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_each_operand_kind() {
        // LOADC 0x1234 ; LOADV V1 ; SAVEV I1 ; MOVC East(2) ; NOP ; JZ ...
        let code = vec![
            LOADC, 0x34, 0x12, // LOADC 0x1234
            LOADV, VAR_V1, // LOADV V1
            SAVEV, 0, // SAVEV I1
            MOVC, 2, 0,    // MOVC 0x2  (East)
            NOP,  // NOP
            JZ, 5, 0, // JZ -> (addr 11 + 5) = 0x10
        ];
        let listing = disassemble(&code);

        assert_eq!(listing[0].to_string(), "0000  LOADC 0x1234");
        assert_eq!(listing[1].to_string(), "0003  LOADV V1");
        assert_eq!(listing[2].to_string(), "0005  SAVEV I1");
        assert_eq!(listing[3].to_string(), "0007  MOVC  0x2");
        assert_eq!(listing[4].to_string(), "000a  NOP");
        assert_eq!(listing[5].mnemonic, "JZ");
        assert_eq!(listing[5].operand.as_deref(), Some("0010"));
    }

    #[test]
    fn opcode_and_selectors_are_reduced_modulo() {
        // a byte of 8 + opcode aliases to the same opcode; 8 == NOP here
        let code = vec![NUMBER_OF_INSTRUCTIONS + NOP, 0, 0];
        assert_eq!(disassemble(&code)[0].mnemonic, "NOP");
    }

    #[test]
    fn never_panics_on_random_genomes() {
        for _ in 0..100 {
            let code: Vec<u8> = (0..CODE_SIZE).map(|_| fastrand::u8(..)).collect();
            let listing = disassemble(&code);
            assert!(!listing.is_empty());
            // rendering must also not panic
            let _ = disassemble_to_string(&code);
        }
    }
}
