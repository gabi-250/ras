use super::instruction_repr::INSTR_REPRS;
use super::mnemonic::Mnemonic;
use super::register::Register;
use std::hash::Hash;

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

pub struct Operands(pub [Option<Operand>; 4]);

impl From<Vec<Operand>> for Operands {
    fn from(args: Vec<Operand>) -> Self {
        assert!(args.len() <= 4);

        let mut args = args.into_iter();
        let operands: [Option<Operand>; 4] = [args.next(), args.next(), args.next(), args.next()];

        Self(operands)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OperandMode {
    Al(u8),
    Imm(u8),
    Rm(u8),
    None,
}

impl From<&str> for OperandMode {
    fn from(op: &str) -> Self {
        match op {
            "AL" => OperandMode::Al(8),
            "AX" => OperandMode::Al(16),
            "EAX" => OperandMode::Al(32),
            "RAX" => OperandMode::Al(64),
            "r/m8" => OperandMode::Rm(8),
            "r/m16" => OperandMode::Rm(16),
            "r/m32" => OperandMode::Rm(32),
            "r/m64" => OperandMode::Rm(64),
            "r8" => OperandMode::Rm(8),
            "r16" => OperandMode::Rm(16),
            "r32" => OperandMode::Rm(32),
            "r64" => OperandMode::Rm(64),
            "imm8" => OperandMode::Imm(8),
            "imm16" => OperandMode::Imm(16),
            "imm32" => OperandMode::Imm(32),
            _ => OperandMode::None,
        }
    }
}

impl From<&Operand> for OperandMode {
    fn from(op: &Operand) -> Self {
        match op {
            Operand::Register(reg) => OperandMode::Rm(reg.size() as u8),
            Operand::Immediate(imm) => OperandMode::Imm(imm.size() as u8),
            _ => OperandMode::None,
        }
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
