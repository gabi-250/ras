use super::instruction::{Operands, Scale};
use super::register::Register;
use super::Mode;
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

pub(crate) struct InstructionRepr {
    pub opcode: u8,
    pub modrm: bool,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub modes: Vec<Mode>,
}

impl InstructionRepr {
    pub(crate) fn emit_instr(&self, operands: Operands) -> Vec<u8> {
        let Operands {
            mode,
            operand1,
            operand2,
            operand3,
        } = operands;

        let mut out = vec![];

        if let Some(rex_prefix) = self.rex_prefix {
            out.push(rex_prefix.into());
        }

        out.push(self.opcode);

        if self.modrm {
            out.push(modrm(
                0b11, // XXX todo
                operand2.unwrap().reg_num(),
                operand1.unwrap().reg_num(),
            ))
        }

        out
    }
}

/// The value of the ModR/M byte.
pub(crate) fn modrm(md: u8, rm: u8, reg: u8) -> u8 {
    ((md & 0b11) << 6) + ((rm & 0b111) << 3) + reg
}

/// The value of the SIB byte. From the Intel manual:
///   * The scale field specifies the scale factor.
///   * The index field specifies the register number of the index register.
///   * The base field specifies the register number of the base register.
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
