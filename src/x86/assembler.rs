use super::encoder::Encoder;
use super::instruction::Instruction;
use super::Mode;

pub struct Assembler {
    mode: Mode,
    instrs: Vec<Instruction>,
    encoder: Encoder,
}

impl Assembler {
    pub fn new_long(instrs: Vec<Instruction>) -> Self {
        Self {
            mode: Mode::Long,
            instrs,
            encoder: Default::default(),
        }
    }

    pub fn assemble(mut self) -> Vec<u8> {
        assert_eq!(self.mode, Mode::Long);

        for instr in self.instrs {
            instr.encode(&mut self.encoder);
        }

        self.encoder.out
    }
}
