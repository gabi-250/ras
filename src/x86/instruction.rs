use super::instruction_repr::INSTR_REPRS;
use super::mnemonic::Mnemonic;
use super::register::Register;

pub struct Instruction {
    mnemonic: Mnemonic,
    operands: Operands,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, args: Vec<Operand>) -> Self {
        Self {
            mnemonic,
            operands: Operands::from(args),
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let variants = (*INSTR_REPRS).get(&self.mnemonic).unwrap();

        variants
            .into_iter()
            .find(|variant| variant.matches(&self.operands))
            .unwrap()
            .emit_instr(self.operands)
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
            Operand::Register(reg) => reg.reg_num(),
            _ => 0,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Operand::Register(reg) => reg.size(),
            Operand::Immediate(imm) => imm.size(),
            _ => unimplemented!("{:#x?}", self),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
}

impl Immediate {
    pub fn size(&self) -> usize {
        match self {
            Self::Immediate8(_) => 8,
            Self::Immediate16(_) => 16,
            Self::Immediate32(_) => 32,
        }
    }
}

pub struct Operands(pub Vec<Operand>);

impl From<Vec<Operand>> for Operands {
    fn from(operands: Vec<Operand>) -> Self {
        assert!(operands.len() <= 4);

        Self(operands)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::x86::register::{RAX, RBX, RCX};

    macro_rules! encode_instr {
        ($opcode:ident, $($operands:expr),*) => {
            Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            ).encode()
        }
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
