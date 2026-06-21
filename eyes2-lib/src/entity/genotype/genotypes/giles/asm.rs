//! Disassembler and assembler for the `giles` virtual machine genome.
//!
//! [`disassemble`] turns a raw genome (a block of bytes) into a human readable
//! listing, decoding each instruction the way the VM does (every opcode and
//! operand selector is reduced modulo its range, so any random byte block is a
//! valid - if nonsensical - program). [`assemble`] is the inverse: it parses a
//! listing back into genome bytes. Together they are the counterpart of the
//! original project's `DisAssemble` / `Assemble`.
//!
//! The two are exact inverses on canonical programs: for any genome `g`,
//! `disassemble(assemble(disassemble(g))) == disassemble(g)`.

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

/// read a 16 bit little-endian constant starting at `index`
fn constant_at(code: &[u8], index: usize) -> u16 {
    let lo = code[index] as u16;
    let hi = code[index + 1] as u16;
    lo.wrapping_add(hi.wrapping_mul(256))
}

/// Disassemble a genome into a list of decoded instructions.
///
/// Instructions are decoded sequentially from the start. Any trailing bytes
/// that are too few to form a complete instruction (with its operand) are left
/// off the listing, so the listing always describes whole instructions.
pub fn disassemble(code: &[u8]) -> Vec<DisasmLine> {
    let mut listing = Vec::new();
    let mut ip = 0usize;

    while ip < code.len() {
        let addr = ip;
        let opcode = code[ip] % NUMBER_OF_INSTRUCTIONS;
        // stop if this instruction's operand would run off the end
        if ip + 1 + operand_size(opcode) > code.len() {
            break;
        }
        ip += 1;
        let mnemonic = MNEMONICS[opcode as usize];

        let operand = match operand_kind(opcode) {
            Operand::None => None,
            Operand::Variable => {
                let var = code[ip] % NUMBER_OF_VARS;
                ip += 1;
                Some(VAR_NAMES[var as usize].to_string())
            }
            Operand::IoVariable => {
                let var = code[ip] as usize % NUMBER_OF_IO_VARS;
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

/// An error encountered while assembling a listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssembleError {
    /// an unrecognised instruction mnemonic
    UnknownMnemonic { line: usize, token: String },
    /// an unrecognised variable / register name
    UnknownVariable { line: usize, token: String },
    /// a numeric operand that could not be parsed as a (hex) number
    BadNumber { line: usize, token: String },
    /// an instruction that requires an operand but none was given
    MissingOperand { line: usize, mnemonic: String },
    /// an explicit address that could not be parsed as a (hex) number
    BadAddress { line: usize, token: String },
    /// the assembled program does not fit within a genome
    ProgramTooLong { line: usize },
}

/// parse a hex number, tolerating a leading `0x`
fn parse_hex(token: &str) -> Option<usize> {
    usize::from_str_radix(token.trim_start_matches("0x").trim_start_matches("0X"), 16).ok()
}

/// Assemble a listing into genome bytes.
///
/// Each non-empty line is `[ADDR] MNEMONIC [OPERAND]`, where the optional
/// leading `ADDR` (hex, as emitted by [`disassemble`]) places the instruction
/// at that genome offset; otherwise instructions are laid down sequentially.
/// Text after `;` or `#` is treated as a comment. Gaps between addressed
/// instructions are filled with `NOP`. The returned vector is exactly long
/// enough to hold the assembled program (it is not padded to [`CODE_SIZE`]);
/// use [`super::GilesGenotype::from_genome`] to turn it into a full genome.
pub fn assemble(source: &str) -> Result<Vec<u8>, AssembleError> {
    let mut code = vec![NOP; 0];
    let mut ip = 0usize;

    for (i, raw_line) in source.lines().enumerate() {
        let line = i + 1;
        // strip comments and surrounding whitespace
        let text = raw_line.split([';', '#']).next().unwrap_or("").trim();
        if text.is_empty() {
            continue;
        }

        let parts: Vec<&str> = text.split_whitespace().collect();

        // A line is `[ADDR] MNEMONIC [OPERAND]`. The first token is an explicit
        // address only when it is followed by a recognised mnemonic; otherwise
        // the first token is taken to be the mnemonic (so a typo is reported as
        // an unknown mnemonic rather than a bad address).
        let rest: &[&str] = if opcode_from_mnemonic(parts[0]).is_some() {
            &parts
        } else if parts.len() >= 2 && opcode_from_mnemonic(parts[1]).is_some() {
            ip = parse_hex(parts[0].trim_end_matches(':')).ok_or_else(|| {
                AssembleError::BadAddress {
                    line,
                    token: parts[0].to_string(),
                }
            })?;
            &parts[1..]
        } else {
            &parts
        };

        let mnemonic_token = rest[0];
        let operand_token = rest.get(1).copied();

        let opcode = opcode_from_mnemonic(mnemonic_token).ok_or_else(|| {
            AssembleError::UnknownMnemonic {
                line,
                token: mnemonic_token.to_string(),
            }
        })?;

        // grow the buffer (with NOP fill for any gap) so we can write at ip
        let end = ip + 1 + operand_size(opcode);
        if end > CODE_SIZE {
            return Err(AssembleError::ProgramTooLong { line });
        }
        if end > code.len() {
            code.resize(end, NOP);
        }

        let addr = ip;
        code[ip] = opcode;
        ip += 1;

        match operand_kind(opcode) {
            Operand::None => {}
            Operand::Variable => {
                let token = operand_token.ok_or_else(|| AssembleError::MissingOperand {
                    line,
                    mnemonic: mnemonic_token.to_string(),
                })?;
                let var = variable_from_name(token).ok_or_else(|| {
                    AssembleError::UnknownVariable {
                        line,
                        token: token.to_string(),
                    }
                })?;
                code[ip] = var;
                ip += 1;
            }
            Operand::IoVariable => {
                let token = operand_token.ok_or_else(|| AssembleError::MissingOperand {
                    line,
                    mnemonic: mnemonic_token.to_string(),
                })?;
                let var = io_variable_from_name(token).ok_or_else(|| {
                    AssembleError::UnknownVariable {
                        line,
                        token: token.to_string(),
                    }
                })?;
                code[ip] = var;
                ip += 1;
            }
            Operand::Constant => {
                let token = operand_token.ok_or_else(|| AssembleError::MissingOperand {
                    line,
                    mnemonic: mnemonic_token.to_string(),
                })?;
                let value = parse_hex(token).ok_or_else(|| AssembleError::BadNumber {
                    line,
                    token: token.to_string(),
                })? as u16;
                code[ip] = (value & 0xff) as u8;
                code[ip + 1] = (value >> 8) as u8;
                ip += 2;
            }
            Operand::Jump => {
                let token = operand_token.ok_or_else(|| AssembleError::MissingOperand {
                    line,
                    mnemonic: mnemonic_token.to_string(),
                })?;
                let target = parse_hex(token).ok_or_else(|| AssembleError::BadNumber {
                    line,
                    token: token.to_string(),
                })?;
                // store the offset relative to this instruction's own address,
                // the inverse of how disassemble() computes the target
                let offset = ((target + CODE_SIZE - (addr % CODE_SIZE)) % CODE_SIZE) as u16;
                code[ip] = (offset & 0xff) as u8;
                code[ip + 1] = (offset >> 8) as u8;
                ip += 2;
            }
        }
    }

    Ok(code)
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
    fn trailing_partial_instruction_is_dropped() {
        // LOADC needs two operand bytes but only one is present
        let code = vec![NOP, LOADC, 0x01];
        let listing = disassemble(&code);
        assert_eq!(listing.len(), 1);
        assert_eq!(listing[0].mnemonic, "NOP");
    }

    #[test]
    fn assembles_and_disassembles_a_hand_written_program() {
        let source = "
            ; head east while there is grass ahead
            LOADV V3      # look east
            JNZ 0x0       # (re)start if something is there
            MOVC 0x2      ; move east
        ";
        let code = assemble(source).expect("assembles");
        let listing = disassemble(&code);
        assert_eq!(listing[0].to_string(), "0000  LOADV V3");
        assert_eq!(listing[1].mnemonic, "JNZ");
        assert_eq!(listing[1].operand.as_deref(), Some("0000"));
        assert_eq!(listing[2].to_string(), "0005  MOVC  0x2");
    }

    #[test]
    fn round_trips_random_genomes() {
        // disassemble -> assemble -> disassemble is stable for any genome
        for _ in 0..200 {
            let code: Vec<u8> = (0..CODE_SIZE).map(|_| fastrand::u8(..)).collect();
            let listing1 = disassemble(&code);
            let text = disassemble_to_string(&code);
            let reassembled = assemble(&text).expect("reassembles");
            let listing2 = disassemble(&reassembled);
            assert_eq!(listing1, listing2);
        }
    }

    #[test]
    fn jump_targets_survive_round_trip() {
        // a backward jump and a forward jump
        let source = "0010 JZ 0x4\n0004 JNZ 0x20\n";
        let code = assemble(source).unwrap();
        let listing = disassemble(&code);
        let jz = listing.iter().find(|l| l.addr == 0x10).unwrap();
        let jnz = listing.iter().find(|l| l.addr == 0x04).unwrap();
        assert_eq!(jz.operand.as_deref(), Some("0004"));
        assert_eq!(jnz.operand.as_deref(), Some("0020"));
    }

    #[test]
    fn reports_errors() {
        assert!(matches!(
            assemble("BOGUS V1"),
            Err(AssembleError::UnknownMnemonic { line: 1, .. })
        ));
        assert!(matches!(
            assemble("LOADV ZZ"),
            Err(AssembleError::UnknownVariable { line: 1, .. })
        ));
        assert!(matches!(
            assemble("LOADC xyz"),
            Err(AssembleError::BadNumber { line: 1, .. })
        ));
        assert!(matches!(
            assemble("MOVC"),
            Err(AssembleError::MissingOperand { line: 1, .. })
        ));
    }

    #[test]
    fn never_panics_on_random_genomes() {
        for _ in 0..100 {
            let code: Vec<u8> = (0..CODE_SIZE).map(|_| fastrand::u8(..)).collect();
            let listing = disassemble(&code);
            assert!(!listing.is_empty());
            let _ = disassemble_to_string(&code);
        }
    }
}
