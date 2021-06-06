use ras_x86::assembler::{Assembler, Item};
use ras_x86::context::Label;
use ras_x86::instruction::Instruction;
use ras_x86::mnemonic::Mnemonic;
use ras_x86::operand::{Immediate, Operand};
use ras_x86::register::RAX;
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
        Item::Label(1),
        Item::Instruction(Instruction::new(
            Mnemonic::MOV,
            vec![
                Operand::Register(*RAX),
                Operand::Immediate(Immediate::Imm32(102)),
            ],
        )),
        Item::Instruction(Instruction::new(Mnemonic::RET, vec![])),
    ];

    let mut asm = Assembler::new_long(
        insts,
        vec![(1, Label::global(1, "test".into()))]
            .into_iter()
            .collect(),
    );
    asm.assemble()?;
    asm.write_obj(out_file)?;

    Ok(())
}
