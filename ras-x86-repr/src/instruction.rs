use crate::operand::{OperandKind, OperandRepr};
use crate::prefix::RexPrefix;
use crate::Mode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionRepr {
    pub opcode: u8,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub opcode_extension: Option<u8>,
    pub operands: Vec<OperandRepr>,
    /// According to the "Intel 64 and IA-32 Architectures Software Developer's Manual": "Indicates
    /// the use of 66/F2/F3 prefixes (beyond those already part of the instructions opcode) are not
    /// allowed with the instruction. Such use will either cause an invalid-opcode exception (#UD)
    /// or result in the encoding for a different instruction."
    pub is_np: bool,
    pub modes: Vec<Mode>,
}

impl InstructionRepr {
    pub fn has_modrm(&self) -> bool {
        self.operands
            .iter()
            .any(|op| matches!(op.kind, OperandKind::ModRmReg | OperandKind::ModRmRegMem))
    }

    /// Check the direction of data operation by looking at the d bit of the opcode.
    /// XXX this bit is actually the sign extension bit if the operand is an immediate value.
    pub fn direction(&self) -> OperationDirection {
        assert!(
            self.operands.len() == 2,
            "direction of operation only makes sense for two-operand instructions"
        );

        (self.opcode % 2 as u8).into()
    }

    /// Return `true` if the data is full-sized.
    ///
    /// The data can be byte or full-sized, where full-sized is 16 or 32 bits. This information is
    /// extracted from the w bit of the opcode (if w = 0, the data is byte-sized).
    pub fn is_full_sized(&self) -> bool {
        self.opcode % 2 == 1
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OperationDirection {
    SrcDst = 0,
    DstSrc = 1,
}

impl From<u8> for OperationDirection {
    fn from(dir: u8) -> Self {
        match dir {
            0 => Self::SrcDst,
            1 => Self::DstSrc,
            _ => panic!("invalid operand direction"),
        }
    }
}
