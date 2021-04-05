use ras_x86::assembler::Assembler;
use ras_x86::instruction::{Instruction, Operand};
use ras_x86::mnemonic::Mnemonic;
use ras_x86::register::{RAX, RBX, RCX};

fn main() {
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

    println!("{:x?}", Assembler::new_long(instrs).assemble());
}
