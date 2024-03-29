use std::convert::TryFrom;
use std::ops::Deref;

use crate::error::{ParseError, ParseErrorKind};

macro_rules! decl_reg {
    ($name64:ident, $name32:ident, $name16:ident $(, $name8lo:ident $(, $name8hi:ident)?)? - $reg_name:ident) => {
        lazy_static! {
            pub static ref $name64: Register = Register::Register64(RegisterNum::$reg_name);
            pub static ref $name32: Register = Register::Register32(RegisterNum::$reg_name);
            pub static ref $name16: Register = Register::Register16(RegisterNum::$reg_name);
            $(
                pub static ref $name8lo: Register = Register::Register8Lo(RegisterNum::$reg_name);
                $(pub static ref $name8hi: Register = Register::Register8Hi(RegisterNum::$reg_name);)?
            )?
        }
    }
}

pub(crate) mod reg_defs {
    use super::*;
    use lazy_static::lazy_static;

    decl_reg!(RAX, EAX, AX, AL, AH - Rax);
    decl_reg!(RBX, EBX, BX, BL, BH - Rbx);
    decl_reg!(RCX, ECX, CX, CL, CH - Rcx);
    decl_reg!(RDX, EDX, DX, DL, DH - Rdx);
    decl_reg!(RDI, EDI, DI, DIL - Rdi);
    decl_reg!(RSI, ESI, SI, SIL - Rsi);
    decl_reg!(RBP, EBP, BP, BPL - Rbp);
    decl_reg!(RSP, ESP, SP - Rsp);
}

use reg_defs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Register8Hi(RegisterNum),
    Register8Lo(RegisterNum),
    Register16(RegisterNum),
    Register32(RegisterNum),
    Register64(RegisterNum),
}

impl Register {
    pub fn size(&self) -> u32 {
        use Register::*;

        match *self {
            Register8Hi(_) | Register8Lo(_) => 8,
            Register16(_) => 16,
            Register32(_) => 32,
            Register64(_) => 64,
        }
    }
}

impl Deref for Register {
    type Target = RegisterNum;

    fn deref(&self) -> &Self::Target {
        use Register::*;

        match self {
            Register8Hi(r) | Register8Lo(r) | Register16(r) | Register32(r) | Register64(r) => r,
        }
    }
}

impl TryFrom<&[u8]> for Register {
    type Error = ParseError;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let reg = match &s.to_ascii_lowercase()[..] {
            b"rax" => *RAX,
            b"eax" => *EAX,
            b"ax" => *AX,
            b"ah" => *AH,
            b"al" => *AL,
            b"rbx" => *RBX,
            b"ebx" => *EBX,
            b"bx" => *BX,
            b"bh" => *BH,
            b"bl" => *BL,
            b"rcx" => *RCX,
            b"ecx" => *ECX,
            b"cx" => *CX,
            b"ch" => *CH,
            b"cl" => *CL,
            b"rdx" => *RDX,
            b"edx" => *EDX,
            b"dx" => *DX,
            b"dh" => *DH,
            b"dl" => *DL,
            b"rdi" => *RDI,
            b"edi" => *EDI,
            b"di" => *DI,
            b"dil" => *DIL,
            b"rsi" => *RSI,
            b"esi" => *ESI,
            b"si" => *SI,
            b"sil" => *SIL,
            b"rbp" => *RBP,
            b"ebp" => *EBP,
            b"bp" => *BP,
            b"bpl" => *BPL,
            b"rsp" => *RSP,
            b"esp" => *ESP,
            b"sp" => *SP,
            s => {
                return Err(ParseError::new(ParseErrorKind::InvalidRegister(
                    String::from_utf8_lossy(s).into(),
                )))
            }
        };

        Ok(reg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterNum {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rbx = 3,
    Rsp = 4,
    Rbp = 5,
    Rsi = 6,
    Rdi = 7,
    // XXX
    //R8 = 0,
    //R9 = 1,
    //R10 = 2,
    //R11 = 3,
    //R12 = 4,
    //R13 = 5,
    //R14 = 6,
    //R15 = 7,
}
