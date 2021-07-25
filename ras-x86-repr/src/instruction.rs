use crate::operand::{OperandKind, OperandRepr};
use crate::prefix::RexPrefix;
use crate::Mode;

use serde::{Deserialize, Serialize};

use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionRepr {
    pub encoding: InstructionEncoding,
    pub operands: Vec<OperandRepr>,
    pub modes: Vec<Mode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionEncoding {
    pub bytecode: Vec<EncodingBytecode>,
    /// According to the "Intel 64 and IA-32 Architectures Software Developer's Manual": "Indicates
    /// the use of 66/F2/F3 prefixes (beyond those already part of the instructions opcode) are not
    /// allowed with the instruction. Such use will either cause an invalid-opcode exception (#UD)
    /// or result in the encoding for a different instruction."
    pub is_np: bool,
}

impl InstructionRepr {
    pub fn new(
        encoding: InstructionEncoding,
        operands: Vec<OperandRepr>,
        modes: Vec<Mode>,
    ) -> Self {
        Self {
            encoding,
            operands,
            modes,
        }
    }

    pub fn has_modrm(&self) -> bool {
        self.operands
            .iter()
            .any(|op| matches!(op.kind, OperandKind::ModRmReg | OperandKind::ModRmRegMem))
    }

    /// Return `true` if the data is full-sized.
    ///
    /// The data can be byte or full-sized, where full-sized is 16 or 32 bits. This information is
    /// extracted from the w bit of the opcode (if w = 0, the data is byte-sized).
    pub fn is_full_sized(&self) -> bool {
        self.encoding
            .bytecode
            .iter()
            .find_map(|code| match &code {
                EncodingBytecode::Opcode(op) => Some(op % 2 == 1),
                _ => None,
            })
            .unwrap_or_default()
    }
}

impl InstructionEncoding {
    pub fn new(bytecode: Vec<EncodingBytecode>, is_np: bool) -> Self {
        Self { bytecode, is_np }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum EncodingBytecode {
    Rex(RexPrefix),
    Prefix(u8),
    Opcode(u8),
    ModRm,
    ModRmWithReg(u8),
    Ib,
    Iw,
    Id,
    Cb,
    Cw,
    Cd,
    Cp,
    Co,
    Ct,
}

impl FromStr for EncodingBytecode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ib" => Ok(EncodingBytecode::Ib),
            "iw" => Ok(EncodingBytecode::Iw),
            "id" => Ok(EncodingBytecode::Id),
            "cb" => Ok(EncodingBytecode::Cb),
            "cw" => Ok(EncodingBytecode::Cw),
            "cd" => Ok(EncodingBytecode::Cd),
            "co" => Ok(EncodingBytecode::Co),
            "cp" => Ok(EncodingBytecode::Cp),
            "ct" => Ok(EncodingBytecode::Ct),
            _ => Err(format!("failed to parse EncodingBytecode: {}", s)),
        }
    }
}
