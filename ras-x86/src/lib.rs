pub mod assembler;
pub mod encoder;
pub mod error;
pub mod instruction;
mod macros;
pub mod mnemonic;
mod object;
pub mod operand;
pub mod parser;
pub mod register;
pub mod symbol;

pub use error::RasError;
pub use mnemonic::Mnemonic;
pub use ras_x86_repr as repr;
pub use repr::Mode;

pub type RasResult<T> = Result<T, RasError>;

#[cfg(test)]
mod tests {
    use crate::assembler::{Assembler, Item};
    use crate::operand::Scale;
    use crate::register::{AL, AX, BX, CX, EAX, EBX, EDX, RAX, RBP, RBX, RCX, RDX, RSP};
    use crate::symbol::{Symbol, SymbolAttribute, SymbolType};
    use crate::{i, imm16, imm32, imm8, label, reg, sib, RasError};

    macro_rules! assert_encoding_eq {
        ([$($expected:expr),*], $($inst:expr),*) => {{
            let mut asm = Assembler::new_long(vec![$($inst),*], &[]);
            asm.assemble().unwrap();
            assert_eq!(&[$($expected),*], asm.dump_out());
        }};

        ($expected_err:expr, $($inst:expr),*) => {{
            let mut asm = Assembler::new_long(vec![$($inst),*], &[]);
            assert_eq!($expected_err, asm.assemble().unwrap_err());
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
    fn add_imm_rax() {
        //assert_encoding_eq!([0x66, 0x05, 0x00, 0x01], i!(ADD, reg!(AX), imm16!(0x100)));
        assert_encoding_eq!(
            [0x05, 0x00, 0x01, 0x00, 0x00],
            i!(ADD, reg!(EAX), imm16!(0x100))
        );
    }

    #[test]
    fn xor_ax_imm16() {
        assert_encoding_eq!([0x66, 0x35, 0x01, 0x01], i!(XOR, reg!(AX), imm16!(0x101)));
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
        assert_encoding_eq!([0x83, 0xf0, 0xff], i!(XOR, reg!(EAX), imm8!(-1)));
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

    #[test]
    fn jmp_sib_memory() {
        //   ff 64 8b 01             jmpq   *0x1(%rbx,%rcx,4)
        assert_encoding_eq!(
            [0xff, 0x64, 0x8b, 0x01],
            i!(JMP, sib!(; 0x1; (RBX, RCX, Scale::Double)))
        );
    }

    #[test]
    fn jmp_pop_16bit_reg() {
        // Requires the 0x66 operand-size prefix in long mode because the operand size isn't the
        // default operand size.
        assert_encoding_eq!([0x66, 0x58], i!(POP, reg!(AX)));
        // The register number is encoded in the 3 least-significant bits of the opcode byte.
        assert_encoding_eq!([0x66, 0x5b], i!(POP, reg!(BX)));
        assert_encoding_eq!([0x66, 0x59], i!(POP, reg!(CX)));
    }

    #[test]
    fn jmp_pop_64bit_reg() {
        assert_encoding_eq!([0x58], i!(POP, reg!(RAX)));
        // The register number is encoded in the 3 least-significant bits of the opcode byte.
        assert_encoding_eq!([0x5b], i!(POP, reg!(RBX)));
        assert_encoding_eq!([0x59], i!(POP, reg!(RCX)));
    }

    #[test]
    fn jmp_local_label() {
        assert_encoding_eq!(
            // nop, followed by the rel32 version of the JMP (0xfffffffa = -6)
            [0x90, 0xe9, 0xfa, 0xff, 0xff, 0xff],
            // 0:
            Item::Label("test_label".to_string()),
            // 0:
            Item::Instruction(i!(NOP)),
            // 1:
            Item::Instruction(i!(JMP, label!("test_label".to_string())))
        );
    }

    #[test]
    fn jmp_undefined_static_symbol() {
        assert_encoding_eq!(
            RasError::UndefinedSymbols(vec!["test_label".into()]),
            Item::Instruction(i!(JMP, label!("test_label".to_string())))
        );
    }

    #[test]
    fn jmp_undefined_global_symbol() {
        let insts = vec![Item::Instruction(i!(JMP, label!("test_label".to_string())))];
        let syms = &[(
            "test_label".into(),
            Symbol::new_decl(SymbolType::Quad, SymbolAttribute::Global as u8),
        )];
        let mut asm = Assembler::new_long(insts, syms);
        asm.assemble().unwrap();

        // The jump target will be filled out by the linker:
        assert_eq!(&[0xe9, 0, 0, 0, 0], asm.dump_out());
    }

    //   XXX
    //   33 54 24 10             xor    0x10(%rsp),%edx
    //   48 8d 5c 03 01          lea    0x1(%rbx,%rax,1),%rbx
}
