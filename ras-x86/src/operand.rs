use crate::register::{Register, RegisterNum};
use crate::repr::operand::{OperandKind, OperandRepr};

#[derive(Debug, Clone)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
    Memory {
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
        matches!(self, Operand::Memory { .. })
    }

    pub fn reg_num(&self) -> Option<u8> {
        match self {
            Operand::Register(reg) => Some(**reg as u8),
            _ => None,
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
            _ => unimplemented!("{:#x?}", self),
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

        if self.is_register() {
            if self.size() != op.size() {
                return false;
            }
        }

        // RAX/EAX/AX/AH/AL
        if op.kind == OperandKind::Al {
            if let Operand::Register(reg) = self {
                return **reg == RegisterNum::Rax;
            }
        }

        return matches!(
            (self, op.kind),
            (Operand::Memory { .. }, OperandKind::ModRmRegMem)
                | (Operand::Register(_), OperandKind::ModRmRegMem)
                | (Operand::Register(_), OperandKind::ModRmReg)
                | (Operand::Immediate(_), OperandKind::Imm)
        );
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
