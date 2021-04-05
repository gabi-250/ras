use lazy_static::lazy_static;
use std::ops::Deref;

lazy_static! {
    pub static ref RAX: Register = Register::Register64(RegisterNum::Rax);
    pub static ref EAX: Register = Register::Register32(RegisterNum::Rax);
    pub static ref AX: Register = Register::Register16(RegisterNum::Rax);
    pub static ref AH: Register = Register::Register8Hi(RegisterNum::Rax);
    pub static ref AL: Register = Register::Register8Lo(RegisterNum::Rax);
    pub static ref RBX: Register = Register::Register64(RegisterNum::Rbx);
    pub static ref EBX: Register = Register::Register32(RegisterNum::Rbx);
    pub static ref BX: Register = Register::Register16(RegisterNum::Rbx);
    pub static ref BH: Register = Register::Register8Hi(RegisterNum::Rbx);
    pub static ref BL: Register = Register::Register8Lo(RegisterNum::Rbx);
    pub static ref RCX: Register = Register::Register64(RegisterNum::Rcx);
    pub static ref ECX: Register = Register::Register32(RegisterNum::Rcx);
    pub static ref CX: Register = Register::Register16(RegisterNum::Rcx);
    pub static ref CH: Register = Register::Register8Hi(RegisterNum::Rcx);
    pub static ref CL: Register = Register::Register8Lo(RegisterNum::Rcx);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    Register8Hi(RegisterNum),
    Register8Lo(RegisterNum),
    Register16(RegisterNum),
    Register32(RegisterNum),
    Register64(RegisterNum),
}

impl Register {
    pub fn size(&self) -> usize {
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
