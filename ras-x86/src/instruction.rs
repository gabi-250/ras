use crate::encoder::Encoder;
use crate::register::{Register, RegisterNum};
use crate::repr::instruction::InstructionRepr;
use crate::repr::mnemonic::Mnemonic;
use crate::repr::operand::{OperandKind, OperandRepr};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref INSTR_REPRS: HashMap<Mnemonic, Vec<InstructionRepr>> = {
        let inst_map = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("bin/map")).unwrap();

        bincode::deserialize(&inst_map).unwrap()
    };
}

pub struct Instruction {
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, operands: Vec<Operand>) -> Self {
        assert!(operands.len() <= 4);

        Self { mnemonic, operands }
    }

    pub fn encode(self, enc: &mut Encoder) {
        let variants = (*INSTR_REPRS).get(&self.mnemonic).unwrap();

        // Find the best instruction encoding (always choose the encoding with the smallest operand
        // sizes).
        let inst_repr = variants
            .into_iter()
            .filter(|variant| Self::matches(variant, &self.operands))
            .reduce(|inst_a, inst_b| {
                let mut found_better = false;
                for (op_repr_a, op_repr_b) in inst_a.operands.iter().zip(inst_b.operands.iter()) {
                    if op_repr_b.size() > op_repr_a.size() {
                        return inst_a;
                    } else if op_repr_b.size() < op_repr_a.size() {
                        found_better = true;
                    }
                }

                if found_better {
                    inst_b
                } else {
                    inst_a
                }
            });

        enc.encode(
            inst_repr.expect("instruction repr not found"),
            self.operands,
        );
    }

    /// Check if the operands can be encoded according to this `InstructionRepr`.
    pub fn matches(repr: &InstructionRepr, operands: &[Operand]) -> bool {
        if repr.operands.len() != operands.len() {
            return false;
        }

        operands
            .iter()
            .zip(repr.operands.iter())
            .all(|(op, op_enc)| op.can_encode(op_enc))
    }
}

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

        if let Operand::Register(reg) = self {
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::register::{RAX, RBX, RCX};

    macro_rules! encode_instr {
        ($opcode:ident, $($operands:expr),*) => {{
            let mut enc = Encoder::default();
            Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            ).encode(&mut enc);

            enc.out
        }}
    }

    #[test]
    fn register_add() {
        assert_eq!(
            encode_instr!(ADD, Operand::Register(*RAX), Operand::Register(*RCX)),
            vec![0x48, 0x01, 0xc8]
        );

        assert_eq!(
            encode_instr!(ADD, Operand::Register(*RAX), Operand::Register(*RBX)),
            vec![0x48, 0x01, 0xd8]
        );
    }
}
