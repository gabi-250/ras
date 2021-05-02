pub mod assembler;
pub mod encoder;
pub mod instruction;
pub mod register;

pub use ras_x86_repr as repr;
pub use repr::mnemonic;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    /// 16-bit real mode.
    Real,
    /// 32-bit protected mode.
    Protected,
    /// 64-bit long mode.
    Long,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Long
    }
}

#[cfg(test)]
mod tests {
    use super::assembler::Assembler;
    use super::instruction::{Immediate, Instruction, Operand, Scale};
    use super::mnemonic::Mnemonic;
    use super::register::{AL, AX, EAX, EBX, RAX, RBP, RBX, RCX};

    macro_rules! assert_encoding_eq {
        ($expected:expr, $opcode:ident, $($operands:expr),*) => {{
            let instr = Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            );
            assert_eq!(&$expected[..], &Assembler::new_long(vec![instr]).assemble());
        }}
    }

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
        assert_encoding_eq!(
            [0x34, 0x02],
            XOR,
            Operand::Register(*AL),
            Operand::Immediate(Immediate::Imm8(2))
        );
    }

    #[test]
    fn xor_ax_imm8() {
        assert_encoding_eq!(
            // XXX: This could be encoded more efficiently by using the XOR AX, imm16 variant of
            // the instruction instead
            //[0x66, 0x35, 0x02, 0x00],
            [0x66, 0x83, 0b11110000, 0x2],
            XOR,
            Operand::Register(*AX),
            Operand::Immediate(Immediate::Imm8(2))
        );
    }

    #[test]
    fn xor_ax_imm16() {
        assert_encoding_eq!(
            [0x66, 0x35, 0x01, 0x01],
            XOR,
            Operand::Register(*AX),
            Operand::Immediate(Immediate::Imm16(0x101))
        );
    }

    #[test]
    fn xor_eax_imm32() {
        assert_encoding_eq!(
            // XXX: This could be encoded more efficiently by using the XOR r/m32, imm8 variant of
            // the instruction instead (0xffffffff is 32-bit -1, but we could represent also it as
            // an 8-bit value).
            //[0x83, 0xf0, 0xff],
            [0x35, 0xff, 0xff, 0xff, 0xff],
            XOR,
            Operand::Register(*EAX),
            Operand::Immediate(Immediate::Imm32(0xffffffff))
        );
    }

    #[test]
    fn xor_rax_imm32() {
        assert_encoding_eq!(
            [0x48, 0x35, 0x00, 0x00, 0x01, 0x00],
            XOR,
            Operand::Register(*RAX),
            Operand::Immediate(Immediate::Imm32(0x10000))
        );
    }

    #[test]
    fn add_ebx_imm8() {
        assert_encoding_eq!(
            [0x83, 0b11000011, 0x2],
            ADD,
            Operand::Register(*EBX),
            Operand::Immediate(Immediate::Imm8(0x2))
        );
    }

    #[test]
    fn mov_imm8_memory_indirect() {
        // XXX use the REX.X prefix to encode r15
        //   42 c6 04 3b 00          movb   $0x0,(%rbx,%r15,1)
        //   c6 04 2b 00             movb   $0x0,(%rbx,%rbp,1)
        assert_encoding_eq!(
            [0xc6, 0b100, 0b111011, 2],
            MOV,
            Operand::Memory {
                segment_override: None,
                base: Some(*RBX),
                index: Some(*RBP),
                scale: Some(Scale::Byte),
                displacement: None
            },
            Operand::Immediate(Immediate::Imm8(0x2))
        );
    }

    //   XXX
    //   33 54 24 10             xor    0x10(%rsp),%edx
    //   48 8d 5c 03 01          lea    0x1(%rbx,%rax,1),%rbx
}
