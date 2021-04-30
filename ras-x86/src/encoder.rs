use crate::instruction::{Immediate, Operand};
use crate::register::Register;
use crate::repr::instruction::{InstructionRepr, OperationDirection};
use crate::repr::prefix::OPERAND_SIZE_PREFIX;
use crate::Mode;

macro_rules! sext {
    ($imm:expr, $size:expr) => {{
        let imm = $imm as i32;
        let remaining = std::mem::size_of_val(&imm) as u32 * 8 - $size;
        let sext_imm = imm.wrapping_shl(remaining).wrapping_shr(remaining);

        match $size {
            8 => (sext_imm as u8).to_le_bytes().to_vec(),
            16 => (sext_imm as u16).to_le_bytes().to_vec(),
            32 => (sext_imm as u32).to_le_bytes().to_vec(),
            n => panic!("invalid imm size: {}", n),
        }
    }};
}

#[derive(Default)]
pub struct Encoder {
    pub out: Vec<u8>,
    mode: Mode,
}

impl Encoder {
    pub fn new(mode: Mode) -> Self {
        Self {
            out: Default::default(),
            mode,
        }
    }

    pub fn encode(&mut self, repr: &InstructionRepr, operands: Vec<Operand>) {
        match operands.len() {
            0 => self.encode_no_operands(repr),
            1 => self.encode_1_operand(repr, &operands[0]),
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

    pub fn encode_1_operand(&mut self, _instr_repr: &InstructionRepr, _operand: &Operand) {
        unimplemented!("single operand instruction");
    }

    pub fn encode_2_operands(
        &mut self,
        instr_repr: &InstructionRepr,
        dst_op: &Operand,
        src_op: &Operand,
    ) {
        // Encode prefixes
        if let Some(rex_prefix) = instr_repr.rex_prefix {
            self.out.push(rex_prefix.into());
        }

        let op_size = std::cmp::max(dst_op.size(), src_op.size());

        if self.needs_operand_size_prefix(instr_repr, op_size) {
            self.out.push(OPERAND_SIZE_PREFIX);
        }

        self.out.push(instr_repr.opcode);

        if instr_repr.has_modrm() {
            // XXX which operand goes into RM? which operand goes into REG? it depends on the
            // instruction operand encoding
            let modrm_reg = if let Some(opcode_ext) = instr_repr.opcode_extension {
                opcode_ext
            } else {
                src_op.reg_num().unwrap()
            };

            self.out.push(modrm(
                0b11, // XXX todo
                modrm_reg,
                dst_op.reg_num().unwrap(),
            ))
        }

        // Do we need to encode an immediate?
        if let Some(imm) = src_op.immediate() {
            self.encode_imm(imm, src_op.size());
        }
    }

    /// Check if the current instruction needs an operand-size prefix.
    ///
    /// An operand-size prefix overrides the default operand-size for a particular instruction. In
    /// 64-bit (long) mode, the default operand size is 32 bits.
    ///
    /// According the to Intel manual, this is how the effective operand size is affected by the REX.W
    /// and operand-size prefixes:
    ///
    /// REX.W Prefix            |0  |0  |0  |0  |1  |1  |1  |1
    /// Operand-Size Prefix     |N  |N  |Y  |Y  |N  |N  |Y  |Y
    /// Effective Operand Size  |32 |32 |16 |16 |64 |64 |64 |64
    fn needs_operand_size_prefix(&mut self, instr_repr: &InstructionRepr, size: u32) -> bool {
        // The REX.W prefix takes precedence over the operand-size prefix, so if the instruction
        // already has a REX.W prefix, there's no need to add the operand-size prefix.
        if instr_repr.rex_prefix.is_some() || !instr_repr.is_full_sized() {
            return false;
        }

        match self.mode {
            Mode::Long => size == 16,
            mode => unimplemented!("mode={:?}", mode),
        }
    }

    fn needs_address_size_prefix(&mut self, size: u32) -> bool {
        unimplemented!()
    }

    /// Encode `imm` to have the specified `size`, sign-extending if needed.
    fn encode_imm(&mut self, imm: Immediate, size: u32) {
        // XXX: MOV r64, imm64 supports 64-bit immediates
        // Can't use more than 32 bits for the immediate.
        match (imm, std::cmp::min(size, 32)) {
            (Immediate::Imm8(imm), size) => self.out.extend(&sext!(imm, size)),
            (Immediate::Imm16(imm), size) => self.out.extend(&sext!(imm, size)),
            (Immediate::Imm32(imm), size) => self.out.extend(&sext!(imm, size)),
        }
    }
}

/// The value of the ModR/M byte.
fn modrm(modifier: u8, rm: u8, reg: u8) -> u8 {
    ((modifier & 0b11) << 6) + ((rm & 0b111) << 3) + reg
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
