use super::instruction_encoding::*;
use super::register::{Register, RegisterNum};

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
        self.mnemonic.encode(self.operands)
    }
}

pub enum Mnemonic {
    Add,
    Nop,
    Xor,
}

impl Mnemonic {
    fn encode(&self, operands: Operands) -> Vec<u8> {
        match self {
            Self::Add => emit_add(operands),
            Self::Nop => emit_nop(operands),
            Self::Xor => emit_xor(operands),
        }
    }
}

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

pub enum OperandMode {
    None,
    IndirectRegister8,
    Register64Register64,
    REXIndirectRegister8,
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
                OperandMode::Register64Register64
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

/// The value of the ModR/M byte.
pub(crate) fn modrm(md: u8, rm: u8, reg: u8) -> u8 {
    ((md & 0b11) << 6) + ((rm & 0b111) << 3) + reg
}

/// The value of the SIB byte.
pub(crate) fn sib(scale: Scale, index: Register, base: Register) -> u8 {
    0
}
