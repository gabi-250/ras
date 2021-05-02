use lazy_static::lazy_static;
use std::ops::Deref;

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

decl_reg!(RAX, EAX, AX, AL, AH - Rax);
decl_reg!(RBX, EBX, BX, BL, BH - Rbx);
decl_reg!(RCX, ECX, CX, CL, CH - Rcx);
decl_reg!(RDX, EDX, DX, DL, DH - Rdx);
decl_reg!(RDI, EDI, DI, DIL - Rdi);
decl_reg!(RSI, ESI, SI, SIL - Rsi);
decl_reg!(RBP, EBP, BP, BPL - Rbp);
decl_reg!(RSP, ESP, SP - Rsp);

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
