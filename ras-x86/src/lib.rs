pub mod assembler;
pub mod encoder;
pub mod error;
pub mod instruction;
mod macros;
pub mod mnemonic;
mod object;
pub mod operand;
pub mod register;
pub mod symbol;

pub use error::RasError;
pub use mnemonic::Mnemonic;
pub use ras_x86_repr as repr;
pub use repr::Mode;

pub type RasResult<T> = Result<T, RasError>;

#[cfg(test)]
mod tests {
    use super::assembler::Assembler;
    use super::operand::Scale;
    use super::register::{AL, AX, EAX, EBX, EDX, RAX, RBP, RBX, RCX, RDX, RSP};
    use crate::{i, imm16, imm32, imm8, reg, sib};

    macro_rules! assert_encoding_eq {
        ($expected:expr, $inst:expr) => {{
            let mut asm = Assembler::new_long(vec![$inst], &[]);
            asm.assemble().unwrap();
            assert_eq!(&$expected[..], asm.dump_out());
        }};
    }

    #[test]
    fn add_reg_reg() {
        assert_encoding_eq!([0x48, 0x01, 0xc8], i!(ADD, reg!(RAX), reg!(RCX)));
        assert_encoding_eq!([0x48, 0x01, 0xc3], i!(ADD, reg!(RBX), reg!(RAX)));
    }

    #[test]
    fn one_byte_nop() {
        assert_encoding_eq!([0x90], i!(NOP));
    }

    #[test]
    fn multi_byte_nop() {
        // TODO
    }

    #[test]
    fn xor_al_imm8() {
        assert_encoding_eq!([0x34, 0x02], i!(XOR, reg!(AL), imm8!(2)));
    }

    #[test]
    fn xor_ax_imm8() {
        assert_encoding_eq!(
            // XXX: This could be encoded more efficiently by using the XOR AX, imm16 variant of
            // the instruction instead
            //[0x66, 0x35, 0x02, 0x00],
            [0x66, 0x83, 0b11110000, 0x2],
            i!(XOR, reg!(AX), imm8!(2))
        );
    }

    #[test]
    fn xor_ax_imm16() {
        assert_encoding_eq!([0x66, 0x35, 0x01, 0x01], i!(XOR, reg!(AX), imm16!(0x101)));
    }

    #[test]
    fn xor_eax_imm32() {
        assert_encoding_eq!(
            // XXX: This could be encoded more efficiently by using the XOR r/m32, imm8 variant of
            // the instruction instead (0xffffffff is 32-bit -1, but we could represent also it as
            // an 8-bit value).
            //[0x83, 0xf0, 0xff],
            [0x35, 0xff, 0xff, 0xff, 0xff],
            i!(XOR, reg!(EAX), imm32!(0xffffffff))
        );
    }

    #[test]
    fn xor_rax_imm32() {
        assert_encoding_eq!(
            [0x48, 0x35, 0x00, 0x00, 0x01, 0x00],
            i!(XOR, reg!(RAX), imm32!(0x10000))
        );
    }

    #[test]
    fn add_ebx_imm8() {
        assert_encoding_eq!([0x83, 0b11000011, 0x2], i!(ADD, reg!(EBX), imm8!(0x2)));
    }

    #[test]
    fn mov_imm8_memory_indirect() {
        // XXX use the REX.X prefix to encode r15
        //   42 c6 04 3b 00          movb   $0x0,(%rbx,%r15,1)
        //   c6 04 2b 02             movb   $0x2,(%rbx,%rbp,1)
        assert_encoding_eq!(
            [0xc6, 0b00_000_100, 0b00_101_011, 2],
            i!(MOV, sib!(; ; (RBX, RBP,)), imm8!(2))
        );
    }

    #[test]
    fn mov_memory_indirect_rax() {
        assert_encoding_eq!(
            [0x48, 0x8b, 0b00_000_100, 0b00_101_011],
            i!(MOV, reg!(RAX), sib!(; ; (RBX, RBP, Scale::Byte)))
        );
    }

    #[test]
    fn mov_imm8_memory_indirect_with_displacement() {
        // XXX use the REX.X prefix to encode r15
        //   42 c6 04 3b 00          movb   $0x0,(%rbx,%r15,1)
        //   c6 04 2b 02             movb   $0x2,(%rbx,%rbp,1)
        assert_encoding_eq!(
            [0xc6, 0b01_000_100, 0b01_101_011, 5, 2],
            //  c6 44 2b 05 02          movb   $0x2,0x5(%rbx,%rbp,1)
            i!(MOV, sib!(; 5; (RBX, RBP, Scale::Word)), imm8!(2))
        );
    }

    #[test]
    fn mov_memory_indirect_with_displacement_rax() {
        assert_encoding_eq!(
            [0x48, 0x8b, 0b01_000_100, 0b01_101_011, 5],
            i!(MOV, reg!(RAX), sib!(; 5; (RBX, RBP, Scale::Word)))
        );
    }

    #[test]
    fn xor_memory_indirect() {
        assert_encoding_eq!(
            [0x33, 0x54, 0x24, 0x10],
            i!(XOR, reg!(EDX), sib!(; 0x10; (RSP,,)))
        );

        // The SIB byte is not needed if the base register is RDX:
        assert_encoding_eq!(
            [0x33, 0x52, 0x10],
            i!(XOR, reg!(EDX), sib!(; 0x10; (RDX,,)))
        );
    }

    //   XXX
    //   33 54 24 10             xor    0x10(%rsp),%edx
    //   48 8d 5c 03 01          lea    0x1(%rbx,%rax,1),%rbx
}
