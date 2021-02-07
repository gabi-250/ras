use super::instruction::{Operand, Operands};
use super::register::Register;
use std::str::FromStr;

/// REX bits: 0100WRXB
const REX: u8 = 0b1000000;
const REX_W: u8 = 0b1001000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RexPrefix {
    None,
    W,
    R,
    X,
    B,
}

impl Into<u8> for RexPrefix {
    fn into(self) -> u8 {
        match self {
            RexPrefix::W => REX | REX_W,
            RexPrefix::None => REX,
            _ => unimplemented!(),
        }
    }
}

impl FromStr for RexPrefix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "REX" => Ok(RexPrefix::None),
            "REX.W" => Ok(RexPrefix::W),
            "REX.R" => Ok(RexPrefix::R),
            "REX.X" => Ok(RexPrefix::X),
            "REX.B" => Ok(RexPrefix::B),
            s => Err(format!("failed to parse REX prefix: {}", s)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct InstructionRepr {
    pub opcode: u8,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub opcode_extension: Option<u8>,
    pub operand_encodings: [Option<OperandEncoding>; 4],
    //pub operand_encoding2: Option<OperandEncoding>,
    //pub operand_encoding3: Option<OperandEncoding>,
    //pub operand_encoding4: Option<OperandEncoding>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OperandEncoding {
    pub kind: OperandKind,
    pub size: usize,
}

impl OperandEncoding {
    pub fn new(kind: OperandKind, size: usize) -> Self {
        Self { kind, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperandKind {
    /// ModRM:reg
    ModRmReg,
    /// ModRM:r/m
    ModRmRegMem,
    /// imm8/16/32
    Imm,
    /// Moffs
    MemoryOffset,
    /// AL/AX/EAX/RAX
    Al,
    /// opcode + rd
    OpcodeRd,
    /// 1
    One,
    /// CL
    Cl,
}

impl InstructionRepr {
    pub(crate) fn emit_instr(&self, operands: Operands) -> Vec<u8> {
        let operand1 = operands.0[0].as_ref();
        let operand2 = operands.0[1].as_ref();

        let mut out = vec![];

        if let Some(rex_prefix) = self.rex_prefix {
            out.push(rex_prefix.into());
        }

        out.push(self.opcode);

        if self.has_modrm() {
            // XXX which operand goes into RM? which operand goes into REG? it depends on the
            // instruction operand encoding
            let modrm_reg = if let Some(opcode_ext) = self.opcode_extension {
                opcode_ext
            } else {
                operand2.unwrap().reg_num()
            };

            out.push(modrm(
                0b11, // XXX todo
                modrm_reg,
                operand1.unwrap().reg_num(),
            ))
        }

        out
    }

    fn has_modrm(&self) -> bool {
        for op in &self.operand_encodings {
            if matches!(
                op.map(|enc| enc.kind),
                Some(OperandKind::ModRmReg) | Some(OperandKind::ModRmRegMem)
            ) {
                return true;
            }
        }

        false
    }

    pub fn matches(&self, operands: &Operands) -> bool {
        operands
            .0
            .iter()
            .zip(self.operand_encodings.iter())
            .all(|(op1, op2)| operands_match(op1, op2))
    }
}

fn operands_match(op1: &Option<Operand>, op2: &Option<OperandEncoding>) -> bool {
    let (op1, op2) = match (op1, op2) {
        (Some(op1), Some(op2)) => (op1, op2),
        (None, None) => return true,
        _ => return false,
    };

    if op1.size() > op2.size() {
        return false;
    }

    return matches!(
        (op1, op2.kind),
        (Operand::Memory, OperandKind::ModRmRegMem) |
        (Operand::Register(_), OperandKind::ModRmRegMem) |
        (Operand::Register(_), OperandKind::ModRmReg) |
        (Operand::Immediate(_), OperandKind::Imm)
    );
}

/// The value of the ModR/M byte.
pub(crate) fn modrm(md: u8, rm: u8, reg: u8) -> u8 {
    ((md & 0b11) << 6) + ((rm & 0b111) << 3) + reg
}

/// The scale used in a SIB expression.
#[allow(unused)]
pub(crate) enum Scale {
    Byte = 0,
    Word,
    Double,
    Quad,
}

/// The value of the SIB byte. From the Intel manual:
///   * The scale field specifies the scale factor.
///   * The index field specifies the register number of the index register.
///   * The base field specifies the register number of the base register.
#[allow(unused)]
pub(crate) fn sib(scale: Option<Scale>, index: Register, base: Register) -> u8 {
    // Table 2-3. 32-Bit Addressing Forms with the SIB Byte
    let scale = match scale {
        Some(Scale::Byte) | None => 0,
        Some(Scale::Word) => 0b01,
        Some(Scale::Double) => 0b10,
        Some(Scale::Quad) => 0b11,
    };

    let index = index.reg_num();
    let base = base.reg_num();

    // XXX is this right?
    ((scale & 0b11) << 6) + ((index & 0b111) << 3) + base
}
