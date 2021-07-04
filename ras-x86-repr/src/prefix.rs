use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// REX bits: 0100WRXB
const REX: u8 = 0b0100_0000;
const REX_W: u8 = 0b0000_1000;
#[allow(unused)]
const REX_R: u8 = 0b0000_0100;
#[allow(unused)]
const REX_X: u8 = 0b0000_0010;
#[allow(unused)]
const REX_B: u8 = 0b0000_0001;

pub const OPERAND_SIZE_PREFIX: u8 = 0x66;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RexPrefix {
    None,
    /// Use a 64-bit operand size instead of the default operand size (which is usually 32-bit in
    /// long mode).
    W,
    /// Extend the ModR/M.reg field.
    ///
    /// This effectively prepends 0b1 to the register number.
    R,
    /// Extend the SIB.index field.
    ///
    /// This effectively prepends 0b1 to the register number.
    X,
    /// Extend the ModR/M.rm field or the SIB.base field (or the register operands coded in the
    /// opcode byte).
    ///
    /// This effectively prepends 0b1 to the register number.
    B,
}

impl Into<u8> for RexPrefix {
    fn into(self) -> u8 {
        match self {
            RexPrefix::W => REX | REX_W,
            RexPrefix::None => REX,
            prefix => unimplemented!("prefix={:?}", prefix),
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
