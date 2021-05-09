use ras_x86::assembler::Assembler;
use ras_x86::instruction::Instruction;
use ras_x86::mnemonic::Mnemonic;
use ras_x86::operand::Operand;
use ras_x86::register::{RAX, RBX, RCX};
use ras_x86::RasResult;

fn main() -> RasResult<()> {
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

    let mut asm = Assembler::new_long(instrs);
    asm.assemble()?;
    asm.write_obj("test.o")?;

    Ok(())
}
