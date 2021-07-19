use ras_x86::assembler::{Assembler, Item};
use ras_x86::instruction::Instruction;
use ras_x86::mnemonic::Mnemonic;
use ras_x86::operand::{Immediate, Operand};
use ras_x86::register::RAX;
use ras_x86::symbol::{Symbol, SymbolAttribute, SymbolType};
use ras_x86::RasResult;

use std::env;

fn main() -> RasResult<()> {
    let mut args = env::args();
    let out_file = if args.len() < 2 {
        eprintln!("Usage: {} <obj file name>", args.next().unwrap());
        std::process::exit(1);
    } else {
        args.skip(1).next().unwrap()
    };

    let insts = vec![
        Item::Instruction(Instruction::new(
            Mnemonic::MOV,
            vec![
                Operand::Register(*RAX),
                Operand::Immediate(Immediate::Imm32(103)),
            ],
        )),
        Item::Instruction(Instruction::new(Mnemonic::RET, vec![])),
        Item::Label("test".to_string()),
        Item::Instruction(Instruction::new(
            Mnemonic::MOV,
            vec![
                Operand::Register(*RAX),
                Operand::Immediate(Immediate::Imm32(102)),
            ],
        )),
        Item::Instruction(Instruction::new(
            Mnemonic::ADD,
            vec![
                Operand::Register(*RAX),
                Operand::Immediate(Immediate::Imm32(102)),
            ],
        )),
        Item::Instruction(Instruction::new(Mnemonic::RET, vec![])),
    ];

    let mut asm = Assembler::new_long(
        insts,
        &[(
            "test".into(),
            Symbol::new_decl(SymbolType::Quad, SymbolAttribute::Global as u8),
        )],
    );
    asm.assemble()?;
    asm.write_obj(out_file)?;

    Ok(())
}
