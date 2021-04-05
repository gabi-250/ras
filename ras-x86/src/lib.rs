pub mod assembler;
pub mod encoder;
pub mod instruction;
pub mod register;

pub use ras_x86_repr as repr;
pub use repr::mnemonic;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
}

#[cfg(test)]
mod tests {
    use super::assembler::Assembler;
    use super::instruction::{Immediate, Instruction, Operand};
    use super::mnemonic::Mnemonic;
    use super::register::{AL, AX, EAX, RAX, RBX, RCX};

    #[test]
    fn add_reg_reg() {
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

    #[test]
    fn one_byte_nop() {
        let instrs = vec![Instruction::new(Mnemonic::NOP, vec![])];

        assert_eq!(vec![0x90], Assembler::new_long(instrs).assemble());
    }

    #[test]
    fn multi_byte_nop() {
        // TODO
    }

    #[test]
    fn xor_al_imm8() {
        let instrs = vec![Instruction::new(
            Mnemonic::XOR,
            vec![
                Operand::Register(*AL),
                Operand::Immediate(Immediate::Imm8(2)),
            ],
        )];

        assert_eq!(vec![0x34, 0x02], Assembler::new_long(instrs).assemble());
    }

    #[test]
    fn xor_ax_imm8() {
        let instrs = vec![Instruction::new(
            Mnemonic::XOR,
            vec![
                Operand::Register(*AX),
                Operand::Immediate(Immediate::Imm8(2)),
            ],
        )];

        assert_eq!(
            vec![0x66, 0x83, 0xf0, 0x02],
            Assembler::new_long(instrs).assemble()
        );
    }

    #[test]
    fn xor_ax_imm16() {
        let instrs = vec![Instruction::new(
            Mnemonic::XOR,
            vec![
                Operand::Register(*AX),
                Operand::Immediate(Immediate::Imm16(0x101)),
            ],
        )];

        assert_eq!(
            vec![0x66, 0x35, 0x01, 0x01],
            Assembler::new_long(instrs).assemble()
        );
    }

    #[test]
    fn xor_eax_imm32() {
        let instrs = vec![Instruction::new(
            Mnemonic::XOR,
            vec![
                Operand::Register(*EAX),
                Operand::Immediate(Immediate::Imm32(0xffffffff)),
            ],
        )];

        assert_eq!(
            vec![0x83, 0xf0, 0xff],
            Assembler::new_long(instrs).assemble()
        );
    }

    #[test]
    fn xor_rax_imm32() {
        let instrs = vec![Instruction::new(
            Mnemonic::XOR,
            vec![
                Operand::Register(*AX),
                Operand::Immediate(Immediate::Imm32(0x10000)),
            ],
        )];

        assert_eq!(
            vec![0x48, 0x35, 0x00, 0x00, 0x01, 0x00],
            Assembler::new_long(instrs).assemble()
        );
    }
}
