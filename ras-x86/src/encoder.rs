use crate::instruction::{Immediate, Operand};
use crate::register::Register;
use crate::repr::instruction::InstructionRepr;

#[derive(Default)]
pub struct Encoder {
    pub out: Vec<u8>,
}

impl Encoder {
    // XXX have a separate emit_ function for 2 operand instructions
    pub fn encode(&mut self, repr: &InstructionRepr, operands: Vec<Operand>) {
        match operands.len() {
            0 => self.encode_no_operands(repr),
            1 => self.encode_1_operand(repr, &operands[0]),
            //2 if self.direction() == OperationDirection::SrcDst => {
            //self.encode_2_operands(repr, &operands[0], &operands[1])
            //}
            //2 => self.encode_2_operands(repr, &operands[1], &operands[0]),
            2 => self.encode_2_operands(repr, &operands[0], &operands[1]),
            n => unimplemented!("{} operands", n),
        }
    }

    pub fn encode_no_operands(&mut self, instr_repr: &InstructionRepr) {
        if let Some(rex_prefix) = instr_repr.rex_prefix {
            self.out.push(rex_prefix.into());
        }

        self.out.push(instr_repr.opcode);
    }

    pub fn encode_1_operand(&mut self, _instr_repr: &InstructionRepr, _operand: &Operand) {}

    pub fn encode_2_operands(
        &mut self,
        instr_repr: &InstructionRepr,
        src_op: &Operand,
        dst_op: &Operand,
    ) {
        self.encode_no_operands(instr_repr);

        if dbg!(instr_repr).has_modrm() {
            // XXX which operand goes into RM? which operand goes into REG? it depends on the
            // instruction operand encoding
            let modrm_reg = if let Some(opcode_ext) = instr_repr.opcode_extension {
                opcode_ext
            } else {
                dst_op.reg_num()
            };

            self.out.push(modrm(
                0b11, // XXX todo
                modrm_reg,
                src_op.reg_num(),
            ))
        }

        // Do we need to encode an immediate?
        if let Some(imm) = src_op.immediate() {
            self.encode_imm(imm, dst_op.size());
        }
    }

    /// Encode `imm` to have the specified `size`, sign-extending if needed.
    fn encode_imm(&mut self, imm: Immediate, size: usize) {
        match (imm, size) {
            (Immediate::Imm8(imm), 8) => self.out.push(imm),
            (Immediate::Imm16(imm), 16) => self.out.extend(&imm.to_le_bytes()),
            (Immediate::Imm32(imm), 32) => self.out.extend(&imm.to_le_bytes()),
            (imm, size) => unimplemented!("imm={:?} size={}", imm, size),
        }
    }
}

enum OperationDirection {
    SrcDst,
    DstSrc,
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

    let index = *index as u8;
    let base = *base as u8;

    // XXX is this right?
    ((scale & 0b11) << 6) + ((index & 0b111) << 3) + base
}
