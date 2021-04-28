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

        let inst = variants
            .into_iter()
            .find(|variant| Self::matches(variant, &self.operands))
            .expect("failed to encode instruction");

        enc.encode(inst, self.operands);
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
    Memory, // XXX
}

impl Operand {
    pub fn reg_num(&self) -> u8 {
        match self {
            Operand::Register(reg) => **reg as u8,
            _ => 0,
        }
    }

    pub fn is_immediate(&self) -> bool {
        matches!(self, Operand::Immediate(_))
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

    /// Check if an operand is compatible with a particular operand encoding.
    pub fn can_encode(&self, op: &OperandRepr) -> bool {
        if self.size() > op.size() {
            return false;
        }

        // RAX/EAX/AX/AH/AL
        if op.kind == OperandKind::Al {
            if let Operand::Register(reg) = self {
                return **reg == RegisterNum::Rax;
            }
        }

        return matches!(
            (self, op.kind),
            (Operand::Memory, OperandKind::ModRmRegMem)
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
    fn emit_add() {
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
