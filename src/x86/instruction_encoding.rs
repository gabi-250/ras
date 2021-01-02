use super::instruction::{modrm, sib, Operand, OperandMode, Operands};

const REX_W: u8 = 0x48;

pub(crate) fn emit_add(operands: Operands) -> Vec<u8> {
    let Operands {
        mode,
        operand1,
        operand2,
        operand3,
    } = operands;

    let mut out = vec![];

    match mode {
        OperandMode::Register64Register64 => {
            out.push(REX_W);
            // opcode
            out.push(0x01);
            out.push(modrm(
                0b11,
                operand2.unwrap().reg_num(),
                operand1.unwrap().reg_num(),
            ))
        }
        _ => unimplemented!(),
    }

    out
}

pub(crate) fn emit_nop(operands: Operands) -> Vec<u8> {
    unimplemented!()
}

pub(crate) fn emit_xor(operands: Operands) -> Vec<u8> {
    unimplemented!()
}
