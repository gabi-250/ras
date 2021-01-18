pub mod assembler;
pub mod instruction;
pub mod instruction_encoding;
pub mod instruction_repr;
pub mod mnemonic;
pub mod register;

pub use mnemonic::Mnemonic;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
}

#[cfg(test)]
mod tests {
    use super::assembler::Assembler;
    use super::instruction::{Instruction, Operand};
    use super::mnemonic::Mnemonic;
    use super::register::{RAX, RBX, RCX};

    #[test]
    fn check_add() {
        let instrs = vec![
            Instruction::new(
                Mnemonic::ADD,
                vec![Operand::Register(*RAX), Operand::Register(*RCX)],
            ),
            Instruction::new(
                Mnemonic::ADD,
                vec![Operand::Register(*RBX), Operand::Register(*RAX)],
            ),
        ];

        assert_eq!(
            vec![0x48, 0x01, 0xc8, 0x48, 0x01, 0xc3],
            Assembler::new_long(instrs).assemble()
        );
    }
}
