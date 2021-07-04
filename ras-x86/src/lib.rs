pub mod assembler;
pub mod context;
pub mod encoder;
pub mod error;
pub mod instruction;
pub mod operand;
pub mod register;

pub use error::RasError;
pub use ras_x86_repr as repr;
pub use repr::mnemonic;
pub use repr::Mode;

pub type RasResult<T> = Result<T, RasError>;

#[cfg(test)]
mod tests {
    use super::assembler::Assembler;
    use super::instruction::Instruction;
    use super::mnemonic::Mnemonic;
    use super::operand::{Immediate, Memory, Operand, Scale};
    use super::register::{AL, AX, EAX, EBX, EDX, RAX, RBP, RBX, RCX, RDX, RSP};

    macro_rules! assert_encoding_eq {
        ($expected:expr, $opcode:ident, $($operands:expr),*) => {{
            let instr = Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            );

            let mut asm =  Assembler::new_long(vec![instr]);
            asm.assemble().unwrap();
            assert_eq!(&$expected[..], asm.dump_out());
        }}
    }

    #[test]
    fn add_reg_reg() {
        assert_encoding_eq!(
            [0x48, 0x01, 0xc8],
            ADD,
            Operand::Register(*RAX),
            Operand::Register(*RCX)
        );

        assert_encoding_eq!(
            [0x48, 0x01, 0xc3],
            ADD,
            Operand::Register(*RBX),
            Operand::Register(*RAX)
        );
    }

    #[test]
    fn one_byte_nop() {
        assert_encoding_eq!([0x90], NOP,);
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
        //   c6 04 2b 02             movb   $0x2,(%rbx,%rbp,1)
        assert_encoding_eq!(
            [0xc6, 0b00_000_100, 0b00_101_011, 2],
            MOV,
            Operand::Memory(Memory::sib(None, Some(*RBX), Some(*RBP), Scale::Byte, None)),
            Operand::Immediate(Immediate::Imm8(0x2))
        );
    }

    #[test]
    fn mov_memory_indirect_rax() {
        assert_encoding_eq!(
            [0x48, 0x8b, 0b00_000_100, 0b00_101_011],
            MOV,
            Operand::Register(*RAX),
            Operand::Memory(Memory::sib(None, Some(*RBX), Some(*RBP), Scale::Byte, None))
        );
    }

    #[test]
    fn mov_imm8_memory_indirect_with_displacement() {
        // XXX use the REX.X prefix to encode r15
        //   42 c6 04 3b 00          movb   $0x0,(%rbx,%r15,1)
        //   c6 04 2b 02             movb   $0x2,(%rbx,%rbp,1)
        assert_encoding_eq!(
            [0xc6, 0b01_000_100, 0b01_101_011, 5, 2],
            MOV,
            //  c6 44 2b 05 02          movb   $0x2,0x5(%rbx,%rbp,1)
            Operand::Memory(Memory::sib(
                None,
                Some(*RBX),
                Some(*RBP),
                Scale::Word,
                Some(5),
            )),
            Operand::Immediate(Immediate::Imm8(0x2))
        );
    }

    #[test]
    fn mov_memory_indirect_with_displacement_rax() {
        assert_encoding_eq!(
            [0x48, 0x8b, 0b01_000_100, 0b01_101_011, 5],
            MOV,
            Operand::Register(*RAX),
            Operand::Memory(Memory::sib(
                None,
                Some(*RBX),
                Some(*RBP),
                Scale::Word,
                Some(5),
            ))
        );
    }

    #[test]
    fn xor_memory_indirect() {
        assert_encoding_eq!(
            [0x33, 0x54, 0x24, 0x10],
            XOR,
            Operand::Register(*EDX),
            Operand::Memory(Memory::sib(None, Some(*RSP), None, Scale::Byte, Some(0x10),))
        );

        // The SIB byte is not needed if the base register is RDX:
        assert_encoding_eq!(
            [0x33, 0x52, 0x10],
            XOR,
            Operand::Register(*EDX),
            Operand::Memory(Memory::sib(None, Some(*RDX), None, Scale::Byte, Some(0x10),))
        );
    }

    //   XXX
    //   33 54 24 10             xor    0x10(%rsp),%edx
    //   48 8d 5c 03 01          lea    0x1(%rbx,%rax,1),%rbx
}
