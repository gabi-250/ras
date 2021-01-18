use super::instruction_repr::INSTR_REPRS;
use super::mnemonic::Mnemonic;
use super::register::{Register, RegisterNum};
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
        (*INSTR_REPRS)
            .get(&(self.mnemonic, self.operands.mode))
            .unwrap()
            .emit_instr(self.operands)
    }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum Immediate {
    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
}

pub struct Operands {
    pub(crate) operand1: Option<Operand>,
    pub(crate) operand2: Option<Operand>,
    pub(crate) operand3: Option<Operand>,
    pub(crate) mode: OperandMode,
}

impl From<Vec<Operand>> for Operands {
    fn from(mut args: Vec<Operand>) -> Self {
        let (operand1, operand2, operand3, mode) = match args.len() {
            0 => (None, None, None, OperandMode::None),
            1 => {
                let operand1 = args.remove(0);
                let mode = OperandMode::from(&operand1);

                (Some(operand1), None, None, mode)
            }
            2 => {
                let operand2 = args.remove(1);
                let operand1 = args.remove(0);
                let mode = OperandMode::from((&operand1, &operand2));

                (Some(operand1), Some(operand2), None, mode)
            }
            3 => {
                let operand3 = args.remove(2);
                let operand2 = args.remove(1);
                let operand1 = args.remove(0);
                let mode = OperandMode::from((&operand1, &operand2, &operand3));

                (Some(operand1), Some(operand2), Some(operand3), mode)
            }
            _ => panic!("too many operands"),
        };

        Self {
            operand1,
            operand2,
            operand3,
            mode,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OperandMode {
    None,
    /// AL, imm8
    AlImm8,
    /// AX, imm16
    AxImm16,
    /// EAX, imm32
    EaxImm32,
    /// RAX, imm32
    RaxImm32,
    /// r/m8, imm8
    Rm8Imm8,
    /// r/m8, imm8
    RexRm8Imm8,
    /// r/m16, imm16
    Rm16Imm16,
    /// r/m32, imm32
    Rm32Imm32,
    /// r/m64, imm32
    Rm64Imm32,
    /// r/m16, imm8
    Rm16Imm8,
    /// r/m32, imm8
    Rm32Imm8,
    /// r/m64, imm8
    Rm64Imm8,
    /// r/m8, r8
    Rm8R8,
    /// r/m8, r8
    RexRm8R8,
    /// r/m16, r16
    Rm16R16,
    /// r/m32, r32
    Rm32R32,
    /// r/m64, r64
    Rm64R64,
    /// r8, r/m8
    R8Rm8,
    /// r8, r/m8
    RexR8Rm8,
    /// r16, r/m16
    R16Rm16,
    /// r32, r/m32
    R32Rm32,
    /// r64, r/m64
    R64Rm64,
}

impl From<(Option<&str>, Option<&str>, Option<&str>, Option<&str>)> for OperandMode {
    fn from(
        (operand1, operand2, operand3, operand4): (
            Option<&str>,
            Option<&str>,
            Option<&str>,
            Option<&str>,
        ),
    ) -> Self {
        match (operand1, operand2, operand3, operand4) {
            (None, None, None, None) => OperandMode::None,
            (Some("AL"), Some("imm8"), None, None) => OperandMode::AlImm8,
            (Some("AX"), Some("imm16"), None, None) => OperandMode::AxImm16,
            (Some("EAX"), Some("imm32"), None, None) => OperandMode::EaxImm32,
            (Some("RAX"), Some("imm32"), None, None) => OperandMode::RaxImm32,
            (Some("r/m8"), Some("imm8"), None, None) => OperandMode::Rm8Imm8,
            (Some("r/m16"), Some("imm16"), None, None) => OperandMode::Rm16Imm16,
            (Some("r/m32"), Some("imm32"), None, None) => OperandMode::Rm32Imm32,
            (Some("r/m64"), Some("imm32"), None, None) => OperandMode::Rm64Imm32,
            (Some("r/m16"), Some("imm8"), None, None) => OperandMode::Rm16Imm8,
            (Some("r/m32"), Some("imm8"), None, None) => OperandMode::Rm32Imm8,
            (Some("r/m64"), Some("imm8"), None, None) => OperandMode::Rm64Imm8,
            (Some("r/m8"), Some("r8"), None, None) => OperandMode::Rm8R8,
            (Some("r/m16"), Some("r16"), None, None) => OperandMode::Rm16R16,
            (Some("r/m32"), Some("r32"), None, None) => OperandMode::Rm32R32,
            (Some("r/m64"), Some("r64"), None, None) => OperandMode::Rm64R64,
            _ => OperandMode::None,
        }
    }
}

impl From<&Operand> for OperandMode {
    fn from(op: &Operand) -> Self {
        unimplemented!()
    }
}

impl From<(&Operand, &Operand)> for OperandMode {
    fn from(ops: (&Operand, &Operand)) -> Self {
        match ops {
            (Operand::Register(r1), Operand::Register(r2))
                if r1.size() == 64 && r2.size() == 64 =>
            {
                OperandMode::Rm64R64
            }
            _ => unimplemented!(),
        }
    }
}

impl From<(&Operand, &Operand, &Operand)> for OperandMode {
    fn from(ops: (&Operand, &Operand, &Operand)) -> Self {
        unimplemented!()
    }
}

/// The scale used in a SIB expression.
pub(crate) enum Scale {
    Byte = 0,
    Word,
    Double,
    Quad,
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
