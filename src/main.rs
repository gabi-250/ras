use ras::x86::assembler::Assembler;
use ras::x86::instruction::{Instruction, Mnemonic, Operand};
use ras::x86::register::{RAX, RBX, RCX};

fn main() {
    let instrs = vec![
        Instruction::new(
            Mnemonic::Add,
            vec![Operand::Register(*RAX), Operand::Register(*RCX)],
        ),
        Instruction::new(
            Mnemonic::Add,
            vec![Operand::Register(*RBX), Operand::Register(*RAX)],
        ),
    ];

    println!("{:x?}", Assembler::new_long(instrs).assemble());
}
