use crate::x86::encoder::Encoder;
use crate::x86::mnemonic::Mnemonic;
use crate::x86::register::Register;
use crate::x86::repr::INSTR_REPRS;

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

        variants
            .into_iter()
            .find(|variant| variant.matches(&self.operands))
            .expect("failed to encode instruction")
            .encode(enc, self.operands);
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

    pub fn immediate(&self) -> Option<Immediate> {
        match self {
            Operand::Immediate(imm) => Some(*imm),
            _ => None,
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
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
}

impl Immediate {
    pub fn size(&self) -> usize {
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
    use crate::x86::register::{RAX, RBX, RCX};

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
