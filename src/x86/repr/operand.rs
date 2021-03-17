use crate::x86::instruction::Operand;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OperandRepr {
    pub kind: OperandKind,
    pub size: usize,
}

impl OperandRepr {
    pub fn new(kind: OperandKind, size: usize) -> Self {
        Self { kind, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Check if an operand is compatible with a particular operand encoding.
    pub fn can_encode(&self, op: &Operand) -> bool {
        if op.size() > self.size() {
            return false;
        }

        return matches!(
            (op, self.kind),
            (Operand::Memory, OperandKind::ModRmRegMem) |
            (Operand::Register(_), OperandKind::ModRmRegMem) |
            (Operand::Register(_), OperandKind::ModRmReg) |
            (Operand::Immediate(_), OperandKind::Imm)
        );
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
