use crate::operand::{OperandKind, OperandRepr};
use crate::prefix::RexPrefix;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionRepr {
    pub opcode: u8,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub opcode_extension: Option<u8>,
    pub operands: Vec<OperandRepr>,
}

impl InstructionRepr {
    pub fn new(
        opcode: u8,
        sib: bool,
        rex_prefix: Option<&str>,
        opcode_extension: Option<u8>,
        operands: Vec<OperandRepr>,
    ) -> Self {
        Self {
            opcode,
            sib,
            rex_prefix: rex_prefix.map(|prefix| RexPrefix::from_str(prefix).unwrap()),
            opcode_extension,
            operands,
        }
    }

    pub fn has_modrm(&self) -> bool {
        for op in &self.operands {
            if matches!(op.kind, OperandKind::ModRmReg | OperandKind::ModRmRegMem) {
                return true;
            }
        }

        false
    }

    pub fn direction(&self) -> OperationDirection {
        assert!(
            self.operands.len() == 2,
            "direction of operation only makes sense for two-operand instructions"
        );

        (self.opcode % 2 as u8).into()
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
