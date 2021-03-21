use crate::x86::encoder::Encoder;
use crate::x86::instruction::Operand;
use crate::x86::prefix::RexPrefix;
use crate::x86::repr::{OperandKind, OperandRepr};
use std::str::FromStr;

#[derive(Debug)]
pub struct InstructionRepr {
    pub opcode: u8,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub opcode_extension: Option<u8>,
    pub operands: Vec<OperandRepr>,
}

impl InstructionRepr {
    pub fn new(
        opcode: u8,
        sib: bool,
        rex_prefix: Option<&str>,
        opcode_extension: Option<u8>,
        operands: Vec<OperandRepr>,
    ) -> Self {
        Self {
            opcode,
            sib,
            rex_prefix: rex_prefix.map(|prefix| RexPrefix::from_str(prefix).unwrap()),
            opcode_extension,
            operands,
        }
    }

    // XXX have a separate emit_ function for 2 operand instructions
    pub fn encode(&self, enc: &mut Encoder, operands: Vec<Operand>) {
        match operands.len() {
            0 => enc.encode_no_operands(&self),
            1 => enc.encode_1_operand(&self, &operands[0]),
            2 => enc.encode_2_operands(&self, &operands[0], &operands[1]),
            n => unimplemented!("{} operands", n),
        }
    }

    pub fn has_modrm(&self) -> bool {
        for op in &self.operands {
            if matches!(op.kind, OperandKind::ModRmReg | OperandKind::ModRmRegMem) {
                return true;
            }
        }

        false
    }

    /// Check if the operands can be encoded according to this `InstructionRepr`.
    pub fn matches(&self, operands: &[Operand]) -> bool {
        if self.operands.len() != operands.len() {
            return false;
        }

        operands
            .iter()
            .zip(self.operands.iter())
            .all(|(op, op_enc)| op_enc.can_encode(op))
    }
}
