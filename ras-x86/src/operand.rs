use crate::register::{Register, RegisterNum};
use crate::repr::operand::{OperandKind, OperandRepr};
pub use crate::symbol::SymbolId;

#[derive(Debug, Clone)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
    Memory(Memory),
}

#[derive(Debug, Clone)]
pub enum Memory {
    Sib {
        /// XXX
        segment_override: Option<Register>,
        /// Any GPR.
        base: Option<Register>,
        /// Any GPR except ESP/RSP.
        index: Option<Register>,
        /// The multiplier (one of 1, 2, 4, or 8).
        scale: Scale,
        /// An 8-, 16-, or 32-bit value.
        displacement: Option<u64>,
    },
    Relative(MemoryRel),
    /// Only valid for MOV instructions
    Moffs(Moffs),
}

#[derive(Debug, Clone)]
pub enum MemoryRel {
    Immediate(Immediate),
    Label(SymbolId),
}

impl Memory {
    pub fn sib(
        segment_override: Option<Register>,
        base: Option<Register>,
        index: Option<Register>,
        scale: Scale,
        displacement: Option<u64>,
    ) -> Self {
        Self::Sib {
            segment_override,
            base,
            index,
            scale,
            displacement,
        }
    }

    pub fn relative(mem_rel: MemoryRel) -> Self {
        Self::Relative(mem_rel)
    }

    pub fn moffs(moffs: Moffs) -> Self {
        Self::Moffs(moffs)
    }

    pub fn is_sib(&self) -> bool {
        matches!(&self, Memory::Sib { .. })
    }

    pub fn is_relative(&self) -> bool {
        matches!(&self, Memory::Relative(_))
    }

    pub fn is_moffs(&self) -> bool {
        matches!(&self, Memory::Moffs(_))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Moffs {
    Moffs8(u8),
    Moffs16(u16),
    Moffs32(u32),
    Moffs64(u64),
}

impl Moffs {
    pub fn size(&self) -> u32 {
        match self {
            Self::Moffs8(_) => 8,
            Self::Moffs16(_) => 16,
            Self::Moffs32(_) => 32,
            Self::Moffs64(_) => 64,
        }
    }
}

/// The scale used in a SIB expression.
#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Byte = 0,
    Word = 0b01,
    Double = 0b10,
    Quad = 0b11,
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

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
}

impl Immediate {
    pub fn size(&self) -> u32 {
        match self {
            Self::Imm8(_) => 8,
            Self::Imm16(_) => 16,
            Self::Imm32(_) => 32,
        }
    }
}
