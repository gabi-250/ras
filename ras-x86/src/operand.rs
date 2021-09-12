mod immediate;
mod memory;
pub(crate) mod register;

use crate::repr::operand::{OperandKind, OperandRepr};

pub use immediate::{Immediate, ImmediateSize};
pub use memory::{Memory, MemoryRel, Scale};
pub use register::{Register, RegisterNum};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
    Memory(Memory),
}

impl Operand {
    pub fn is_register(&self) -> bool {
        matches!(self, Operand::Register(_))
    }

    pub fn is_immediate(&self) -> bool {
        matches!(self, Operand::Immediate(_))
    }

    pub fn is_memory(&self) -> bool {
        matches!(self, Operand::Memory(_))
    }

    pub fn reg_num(&self) -> u8 {
        match self {
            Operand::Register(reg) => **reg as u8,
            _ => 0,
        }
    }

    pub fn immediate(&self) -> Option<Immediate> {
        match self {
            Operand::Immediate(imm) => Some(*imm),
            _ => None,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Operand::Register(reg) => reg.size(),
            Operand::Immediate(imm) => imm.size(),
            Operand::Memory(_) => 64, // XXX
        }
    }

    pub fn is_exact_match(&self, op: &OperandRepr) -> bool {
        self.size() == op.size()
    }

    /// Check if an operand is compatible with a particular operand encoding.
    pub fn can_encode(&self, op: &OperandRepr) -> bool {
        if !self.is_memory() && self.size() > op.size() {
            return false;
        }

        if self.is_register() && self.size() != op.size() {
            return false;
        }

        // RAX/EAX/AX/AH/AL
        if op.kind == OperandKind::Al {
            if let Operand::Register(reg) = self {
                return **reg == RegisterNum::Rax;
            }
        }

        match (self, op.kind) {
            (Operand::Register(_), OperandKind::ModRmRegMem)
            | (Operand::Register(_), OperandKind::ModRmReg)
            | (Operand::Immediate(_), OperandKind::Imm) => true,
            (Operand::Memory(m), OperandKind::ModRmRegMem) if m.is_sib() => true,
            (Operand::Memory(m), OperandKind::Moffs) if m.is_moffs() => true,
            // Be pessimistic and always use the largest (rel32) encoding for jump/call
            // instructions:
            (Operand::Memory(m), OperandKind::Rel32) if m.is_relative() => true,
            _ => false,
        }
    }
}
