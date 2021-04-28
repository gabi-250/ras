use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperandRepr {
    pub kind: OperandKind,
    pub size: u32,
}

impl OperandRepr {
    pub fn new(kind: OperandKind, size: u32) -> Self {
        Self { kind, size }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
