use crate::x86::instruction::Operand;
use crate::x86::prefix::RexPrefix;
use crate::x86::register::Register;
use crate::x86::repr::{OperandKind, OperandRepr};

#[derive(Debug)]
pub(crate) struct InstructionRepr {
    pub opcode: u8,
    pub sib: bool,
    pub rex_prefix: Option<RexPrefix>,
    pub opcode_extension: Option<u8>,
    pub operands: Vec<OperandRepr>,
}

impl InstructionRepr {
    // XXX have a separate emit_ function for 2 operand instructions
    pub(crate) fn emit_instr(&self, operands: Vec<Operand>) -> Vec<u8> {
        let mut out = vec![];

        if let Some(rex_prefix) = self.rex_prefix {
            out.push(rex_prefix.into());
        }

        out.push(self.opcode);

        if self.has_modrm() {
            let operand1 = &operands[0];
            let operand2 = &operands[1];

            // XXX which operand goes into RM? which operand goes into REG? it depends on the
            // instruction operand encoding
            let modrm_reg = if let Some(opcode_ext) = self.opcode_extension {
                opcode_ext
            } else {
                operand2.reg_num()
            };

            out.push(modrm(
                0b11, // XXX todo
                modrm_reg,
                operand1.reg_num(),
            ))
        }

        out
    }

    fn has_modrm(&self) -> bool {
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

/// The value of the ModR/M byte.
pub(crate) fn modrm(md: u8, rm: u8, reg: u8) -> u8 {
    ((md & 0b11) << 6) + ((rm & 0b111) << 3) + reg
}

/// The scale used in a SIB expression.
#[allow(unused)]
pub(crate) enum Scale {
    Byte = 0,
    Word,
    Double,
    Quad,
}

/// The value of the SIB byte. From the Intel manual:
///   * The scale field specifies the scale factor.
///   * The index field specifies the register number of the index register.
///   * The base field specifies the register number of the base register.
#[allow(unused)]
pub(crate) fn sib(scale: Option<Scale>, index: Register, base: Register) -> u8 {
    // Table 2-3. 32-Bit Addressing Forms with the SIB Byte
    let scale = match scale {
        Some(Scale::Byte) | None => 0,
        Some(Scale::Word) => 0b01,
        Some(Scale::Double) => 0b10,
        Some(Scale::Quad) => 0b11,
    };

    let index = index.reg_num();
    let base = base.reg_num();

    // XXX is this right?
    ((scale & 0b11) << 6) + ((index & 0b111) << 3) + base
}
