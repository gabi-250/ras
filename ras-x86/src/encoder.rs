use crate::assembler::{SymbolId, SymbolOffset};
use crate::error::RasError;
use crate::operand::{
    Immediate, ImmediateSize, Memory, MemoryRel, Operand, Register, RegisterNum, Scale,
};
use crate::repr::{EncodingBytecode, InstructionRepr, OperandRepr, Prefix};
use crate::symbol::Symbol;
use crate::Mode;

use std::collections::HashMap;

const SIB_INDEX_NONE: u8 = 0b100;

/// A range of bytes to patch: `[offset, offset + size)`
struct Fixup {
    offset: SymbolOffset,
    size: u64,
}

/// The instruction encoder.
#[derive(Default)]
pub(crate) struct Encoder {
    /// The output buffer where assembled instructions are written.
    pub out: Vec<u8>,
    /// The `Mode` to assemble instructions in.
    pub mode: Mode,
    /// A mapping from symbol -> its occurrences in the code.
    ///
    /// Each non-extern symbol occurrence needs to be patched up with a concrete value by the
    /// assembler.
    rel_jmp_fixups: HashMap<SymbolId, Vec<Fixup>>,
}

impl Encoder {
    pub(crate) fn new(mode: Mode) -> Self {
        Self {
            out: Default::default(),
            mode,
            rel_jmp_fixups: Default::default(),
        }
    }

    /// Returns the current length of the text section.
    pub(crate) fn current_offset(&self) -> SymbolOffset {
        self.out.len() as u64
    }

    /// Returns `true` if the specified instruction can be encoded in the current mode.
    pub(crate) fn is_encodable(&self, repr: &InstructionRepr) -> bool {
        repr.is_valid_in_mode(&self.mode)
    }

    pub(crate) fn encode(
        &mut self,
        repr: &InstructionRepr,
        operands: &[Operand],
    ) -> Result<(), RasError> {
        match operands.len() {
            0 => self.encode_no_operands(repr),
            1 => self.encode_1_operand(repr, &operands[0]),
            2 => self.encode_2_operands(repr, &operands[0], &operands[1]),
            n => unimplemented!("{} operands", n),
        }
    }

    pub(crate) fn fixup_symbol_references(
        &mut self,
        sym_tab: &HashMap<SymbolId, Symbol>,
    ) -> Result<(), RasError> {
        for (symbol_id, symbol) in sym_tab {
            if let Some(offset) = symbol.offset {
                // Patch any symbolic references (e.g. jmp label)
                if let Some(fixups) = self.rel_jmp_fixups.remove(&symbol_id.to_string()) {
                    for fixup in fixups {
                        let start = fixup.offset as usize;
                        let end = (fixup.offset + fixup.size) as usize;
                        let offset =
                            (offset as i32 - (fixup.offset + fixup.size) as i32).to_le_bytes();
                        self.out.splice(start..end, offset.iter().cloned());
                    }
                }
            }
        }
        // If there are still some unresolved symbols, return an error if any of them are not
        // marked as external:
        let undefined_symbols: Vec<String> = self
            .rel_jmp_fixups
            .keys()
            .map(|symbol_id| symbol_id.to_string())
            .filter(|symbol_id| {
                !sym_tab
                    .get(symbol_id)
                    .map(|sym| sym.is_global())
                    .unwrap_or_default()
            })
            .collect();

        if !undefined_symbols.is_empty() {
            return Err(RasError::UndefinedSymbols(undefined_symbols));
        }
        Ok(())
    }

    /// Encode an instruction with no operands.
    fn encode_no_operands(&mut self, inst_repr: &InstructionRepr) -> Result<(), RasError> {
        for code in &inst_repr.encoding.bytecode {
            match code {
                EncodingBytecode::Rex(rex_prefix) => {
                    self.out.push((*rex_prefix).into());
                }
                EncodingBytecode::Opcode(opcode) => self.out.push(*opcode),
                _ => unimplemented!("encoding: {:?}", code),
            }
        }
        Ok(())
    }

    /// Encode a single operand instruction.
    fn encode_1_operand(
        &mut self,
        inst_repr: &InstructionRepr,
        operand: &Operand,
    ) -> Result<(), RasError> {
        let (reg_op, reg_memory_op, imm_op) = match operand {
            Operand::Register(reg) => (Some(reg), None, None),
            Operand::Memory(_) => (None, Some(operand), None),
            Operand::Immediate(imm) => (None, None, Some(imm)),
        };
        for code in &inst_repr.encoding.bytecode {
            self.handle_opcode(
                code,
                inst_repr,
                reg_op,
                reg_memory_op,
                imm_op,
                operand.size(),
            )?;
        }
        Ok(())
    }

    /// Encode a two-operand instruction.
    fn encode_2_operands(
        &mut self,
        inst_repr: &InstructionRepr,
        op1: &Operand,
        op2: &Operand,
    ) -> Result<(), RasError> {
        let op_size = if op1.is_memory() {
            op2.size()
        } else if op2.is_memory() {
            op1.size()
        } else {
            std::cmp::max(op1.size(), op2.size())
        };
        let (reg_op, reg_memory_op, imm_op) = match (op2, op1) {
            (Operand::Register(reg), Operand::Memory(_)) => (Some(reg), Some(op1), None),
            (Operand::Register(reg), Operand::Register(_)) => (Some(reg), Some(op1), None),
            (Operand::Register(reg), Operand::Immediate(imm)) => (Some(reg), None, Some(imm)),
            (Operand::Memory(_), Operand::Register(reg)) => (Some(reg), Some(op2), None),
            (Operand::Memory(_), Operand::Immediate(imm)) => (None, Some(op2), Some(imm)),
            (Operand::Memory(_), _) => (None, Some(op2), None),
            (Operand::Immediate(imm), Operand::Memory(_)) => (None, Some(op1), Some(imm)),
            (Operand::Immediate(imm), Operand::Register(reg)) => (Some(reg), None, Some(imm)),
            _ => unreachable!(
                "instruction repr has ModRM byte but none of the operands are register/memory"
            ),
        };
        for code in &inst_repr.encoding.bytecode {
            self.handle_opcode(code, inst_repr, reg_op, reg_memory_op, imm_op, op_size)?;
        }
        Ok(())
    }

    fn handle_opcode(
        &mut self,
        code: &EncodingBytecode,
        inst_repr: &InstructionRepr,
        reg_op: Option<&Register>,
        reg_memory_op: Option<&Operand>,
        imm_op: Option<&Immediate>,
        op_size: u32,
    ) -> Result<(), RasError> {
        match code {
            EncodingBytecode::Rex(rex_prefix) => self.out.push((*rex_prefix).into()),
            EncodingBytecode::Prefix(prefix) => self.out.push(*prefix),
            EncodingBytecode::Opcode(opcode) => {
                if self.needs_operand_size_prefix(inst_repr, op_size) {
                    self.out.push(Prefix::OperandSize.into());
                }
                self.out.push(*opcode)
            }
            EncodingBytecode::OpcodeRw(opcode)
            | EncodingBytecode::OpcodeRb(opcode)
            | EncodingBytecode::OpcodeRd(opcode) => {
                if self.needs_operand_size_prefix(inst_repr, op_size) {
                    self.out.push(Prefix::OperandSize.into());
                }
                match reg_op {
                    Some(reg_op) => self.encode_reg_in_opcode(*opcode, reg_op),
                    None => unreachable!("invalid operand"),
                }
            }
            EncodingBytecode::ModRm => {
                self.encode_modrm_sib_bytes(reg_op, reg_memory_op, None)?;
            }
            EncodingBytecode::ModRmWithReg(modrm_reg) => {
                self.encode_modrm_sib_bytes(reg_op, reg_memory_op, Some(*modrm_reg))?;
            }
            EncodingBytecode::Cd => {
                if let Some(Operand::Memory(Memory::Relative(rel))) = reg_memory_op {
                    let operand_repr = inst_repr.operands[0];
                    self.encode_rel_memory_offset(rel, operand_repr);
                }
            }
            EncodingBytecode::Ib => {
                if let Some(imm) = imm_op {
                    self.encode_imm(*imm)
                } else {
                    unreachable!("missing immediate operand for opcode {:?}", code);
                }
            }
            EncodingBytecode::Iw => {
                if let Some(imm) = imm_op {
                    self.encode_imm(imm.sign_extend(ImmediateSize::Imm16)?)
                } else {
                    unreachable!("missing immediate operand for opcode {:?}", code);
                }
            }
            EncodingBytecode::Id => {
                if let Some(imm) = imm_op {
                    self.encode_imm(imm.sign_extend(ImmediateSize::Imm32)?)
                } else {
                    unreachable!("missing immediate operand for opcode {:?}", code);
                }
            }
            _ => unimplemented!("encoding: {:?}", code),
        }
        Ok(())
    }

    fn encode_modrm_sib_bytes(
        &mut self,
        reg_op: Option<&Register>,
        reg_memory_op: Option<&Operand>,
        modrm_reg: Option<u8>,
    ) -> Result<(), RasError> {
        let (modrm_reg, rm) = if let Some(modrm_reg) = modrm_reg {
            (modrm_reg, reg_op.map(|reg| **reg as u8).unwrap_or_default())
        } else {
            (
                reg_op.map(|reg| **reg as u8).unwrap_or_default(),
                reg_memory_op.map(|reg| reg.reg_num()).unwrap_or_default(),
            )
        };

        if let Some(Operand::Memory(Memory::Sib {
            displacement,
            base,
            index,
            scale,
            ..
        })) = reg_memory_op
        {
            let (modifier, displacement) = match (base.is_none(), displacement) {
                // In GNU as, expressions with missing base and index registers with no
                // displacement are the same as a 32-bit displacement of 0 (e.g. movb $0x2,(,2)
                // is the same as movb $0x2, 0)
                (true, v) => (0b00, Some((v.unwrap_or(0) as i32).to_le_bytes().to_vec())),
                // disp8
                (false, Some(v)) if *v < i8::MAX as i64 => {
                    (0b01, Some((*v as u8).to_le_bytes().to_vec()))
                }
                // disp32
                (false, Some(v)) => (0b10, Some(v.to_le_bytes().to_vec())),
                (false, None) => (0b00, None),
            };

            let sib = maybe_sib(base.as_ref(), index.as_ref(), *scale, modifier)?;
            let rm = if sib.is_some() {
                0b100
            } else {
                base.map(|base| *base as u8).unwrap_or(0)
            };
            self.out.push(modrm(modifier, modrm_reg, rm));
            if let Some(sib) = sib {
                self.out.push(sib);
            }
            // Encode the displacement if needed
            if let Some(displacement) = displacement {
                self.out.extend(displacement);
            }
        } else {
            self.out.push(modrm(0b11, modrm_reg, rm))
        }
        Ok(())
    }

    fn encode_rel_memory_offset(&mut self, rel: &MemoryRel, operand_repr: OperandRepr) {
        match rel {
            MemoryRel::Absolute(_imm) => unimplemented!("imm memory offset"),
            MemoryRel::Label(symbol_id) => {
                let size = (operand_repr.size() / 8) as usize;
                let fixup = Fixup {
                    offset: self.current_offset(),
                    size: size as u64,
                };
                // Store some zeroes...
                self.out.extend(vec![0; size]);
                // ...and remember that we need to fix-up this location when we resolve the label:
                self.rel_jmp_fixups
                    .entry(symbol_id.to_string())
                    .or_default()
                    .push(fixup);
            }
        }
    }

    /// Check if the current instruction needs an operand-size prefix.
    ///
    /// An operand-size prefix overrides the default operand-size for a particular instruction. In
    /// 64-bit (long) mode, the default operand size is 32 bits.
    ///
    /// According the to Intel manual, this is how the effective operand size is affected by the
    /// REX.W and operand-size prefixes:
    ///
    /// REX.W Prefix            |0  |0  |0  |0  |1  |1  |1  |1
    /// Operand-Size Prefix     |N  |N  |Y  |Y  |N  |N  |Y  |Y
    /// Effective Operand Size  |32 |32 |16 |16 |64 |64 |64 |64
    fn needs_operand_size_prefix(&mut self, inst_repr: &InstructionRepr, size: u32) -> bool {
        // The REX.W prefix takes precedence over the operand-size prefix, so if the instruction
        // already has a REX.W prefix, there's no need to add the operand-size prefix.
        if inst_repr
            .encoding
            .bytecode
            .iter()
            .any(|code| matches!(code, EncodingBytecode::Rex(_)))
            || !inst_repr.is_full_sized()
        {
            return false;
        }
        match self.mode {
            Mode::Long => size == 16,
            mode => unimplemented!("mode={:?}", mode),
        }
    }

    #[allow(unused)]
    fn needs_address_size_prefix(&mut self, size: u32) -> bool {
        unimplemented!()
    }

    /// Encode an immediate.
    fn encode_imm(&mut self, imm: Immediate) {
        // XXX: MOV r64, imm64 supports 64-bit immediates
        // Can't use more than 32 bits for the immediate.
        match imm {
            Immediate::Imm8(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
            Immediate::Imm16(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
            Immediate::Imm32(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
        }
    }

    /// Encode the register number the least significant 3 bits of the opcode byte.
    fn encode_reg_in_opcode(&mut self, opcode: u8, reg: &Register) {
        let opcode = opcode + (0b00000111 & **reg as u8);
        self.out.push(opcode);
    }
}

/// The value of the ModR/M byte.
fn modrm(modifier: u8, reg: u8, rm: u8) -> u8 {
    ((modifier & 0b11) << 6) + ((reg & 0b111) << 3) + rm
}

/// Return the SIB byte for the base-index-scale addressing mode.
///
/// See Table 2-2. 32-Bit Addressing Forms with the ModR/M Byte.
fn maybe_sib(
    base: Option<&Register>,
    index: Option<&Register>,
    scale: Scale,
    modifier: u8,
) -> Result<Option<u8>, RasError> {
    let sib = match (base, index, modifier) {
        (Some(base), None, 0b00) if matches!(**base, RegisterNum::Rsp | RegisterNum::Rbp) => {
            Some(sib(scale as u8, SIB_INDEX_NONE, sib_base(*base)))
        }
        (Some(base), None, 0b01) | (Some(base), None, 0b10)
            if matches!(**base, RegisterNum::Rsp) =>
        {
            Some(sib(scale as u8, SIB_INDEX_NONE, sib_base(*base)))
        }
        (Some(_), None, _) => None,
        (None, Some(index), _) if modifier == 0b00 => Some(sib(scale as u8, **index as u8, 0b101)),
        (Some(base), Some(index), _) => Some(sib(scale as u8, **index as u8, sib_base(*base))),
        _ => return Err(RasError::Encoding("invalid SIB expression".into())),
    };

    Ok(sib)
}

/// Return the base field of the SIB byte for the specified base register.
///
/// See Table 2-3. 32-Bit Addressing Forms with the SIB Byte.
fn sib_base(reg: Register) -> u8 {
    match *reg {
        RegisterNum::Rax
        | RegisterNum::Rcx
        | RegisterNum::Rdx
        | RegisterNum::Rbx
        | RegisterNum::Rsp
        | RegisterNum::Rsi
        | RegisterNum::Rdi => *reg as u8,
        _ => 0b101,
    }
}

/// The value of the SIB byte. From the Intel manual:
///   * The scale field specifies the scale factor.
///   * The index field specifies the register number of the index register.
///   * The base field specifies the register number of the base register.
///
/// See Table 2-3. 32-Bit Addressing Forms with the SIB Byte
fn sib(scale: u8, index: u8, base: u8) -> u8 {
    ((scale & 0b11) << 6) + ((index & 0b111) << 3) + base
}
