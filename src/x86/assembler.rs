use super::instruction::Instruction;
use super::Mode;

pub struct Assembler {
    mode: Mode,
    instrs: Vec<Instruction>,
}

impl Assembler {
    pub fn new_long(instrs: Vec<Instruction>) -> Self {
        Self {
            mode: Mode::Long,
            instrs,
        }
    }

    pub fn assemble(self) -> Vec<u8> {
        let mut out = vec![];

        for instr in self.instrs {
            out.extend_from_slice(&instr.encode());
        }

        out
    }
}
