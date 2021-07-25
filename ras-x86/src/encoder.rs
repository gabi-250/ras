use crate::assembler::{InstructionPointer, SymbolId};
use crate::error::RasError;
use crate::operand::{Immediate, Memory, MemoryRel, Operand, Scale};
use crate::register::{Register, RegisterNum};
use crate::repr::{EncodingBytecode, InstructionRepr, OperandRepr, Prefix};
use crate::symbol::Symbol;
use crate::Mode;

use std::collections::HashMap;

const SIB_INDEX_NONE: u8 = 0b100;

pub(crate) struct Fixup {
    pub offset: InstructionPointer,
    pub size: u64,
}

#[derive(Default)]
pub(crate) struct Encoder {
    pub out: Vec<u8>,
    pub mode: Mode,
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

    pub(crate) fn instruction_pointer(&self) -> InstructionPointer {
        self.out.len() as u64
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

    fn encode_no_operands(&mut self, inst_repr: &InstructionRepr) -> Result<(), RasError> {
        for code in &inst_repr.encoding.bytecode {
            match code {
                EncodingBytecode::Rex(rex_prefix) => {
                    self.out.push((*rex_prefix).into());
                }
                EncodingBytecode::Opcode(opcode) => self.out.push(*opcode),
                _ => unreachable!("invalid code: {:?}", code),
            }
        }

        Ok(())
    }

    fn encode_1_operand(
        &mut self,
        inst_repr: &InstructionRepr,
        operand: &Operand,
    ) -> Result<(), RasError> {
        let (reg_op, reg_memory_op) = match operand {
            Operand::Register(_) => (Some(operand), None),
            Operand::Memory(_) => (None, Some(operand)),
            _ => (None, None),
        };

        for code in &inst_repr.encoding.bytecode {
            match code {
                EncodingBytecode::Rex(rex_prefix) => {
                    self.out.push((*rex_prefix).into());
                }
                EncodingBytecode::Prefix(prefix) => self.out.push(*prefix),
                EncodingBytecode::Opcode(opcode) => {
                    if self.needs_operand_size_prefix(&inst_repr, operand.size()) {
                        self.out.push(Prefix::OperandSize.into());
                    }
                    self.out.push(*opcode)
                }
                EncodingBytecode::ModRm => {
                    assert!(reg_op.is_some() || reg_memory_op.is_some());

                    self.encode_modrm_sib_bytes(reg_op, reg_memory_op, None)?;
                }
                EncodingBytecode::ModRmWithReg(modrm_reg) => {
                    assert!(reg_op.is_some() || reg_memory_op.is_some());

                    self.encode_modrm_sib_bytes(reg_op, reg_memory_op, Some(*modrm_reg))?;
                }
                EncodingBytecode::Cd => {
                    if let Some(Operand::Memory(Memory::Relative(rel))) = reg_memory_op {
                        let operand_repr = inst_repr.operands[0];
                        self.encode_rel_memory_offset(&rel, operand_repr);
                    }
                }
                EncodingBytecode::Ib | EncodingBytecode::Iw | EncodingBytecode::Id => {
                    // Do we need to encode an immediate?
                    if let Some(imm) = operand.immediate() {
                        self.encode_imm(imm);
                    }
                }
                _ => unimplemented!("encoding: {:?}", code),
            }
        }

        Ok(())
    }

    fn encode_2_operands(
        &mut self,
        inst_repr: &InstructionRepr,
        dst_op: &Operand,
        src_op: &Operand,
    ) -> Result<(), RasError> {
        let op_size = if dst_op.is_memory() {
            src_op.size()
        } else if src_op.is_memory() {
            dst_op.size()
        } else {
            std::cmp::max(dst_op.size(), src_op.size())
        };

        let (reg_op, reg_memory_op) = match (src_op, dst_op) {
            (Operand::Register(_), Operand::Memory(_)) => (Some(src_op), Some(dst_op)),
            (Operand::Register(_), Operand::Register(_)) => (Some(src_op), Some(dst_op)),
            (Operand::Memory(_), Operand::Register(_)) => (Some(dst_op), Some(src_op)),
            (Operand::Memory(_), _) => (None, Some(src_op)),
            (_, Operand::Register(_)) => (Some(dst_op), None),
            (_, Operand::Memory(_)) => (None, Some(dst_op)),
            _ => unreachable!(
                "instruction repr has ModRM byte but none of the operands are register/memory"
            ),
        };

        for code in &inst_repr.encoding.bytecode {
            match code {
                EncodingBytecode::Rex(rex_prefix) => {
                    self.out.push((*rex_prefix).into());
                }
                EncodingBytecode::Prefix(prefix) => self.out.push(*prefix),
                EncodingBytecode::Opcode(opcode) => {
                    if self.needs_operand_size_prefix(&inst_repr, op_size) {
                        self.out.push(Prefix::OperandSize.into());
                    }
                    self.out.push(*opcode)
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
                        self.encode_rel_memory_offset(&rel, operand_repr);
                    }
                }
                EncodingBytecode::Ib | EncodingBytecode::Iw | EncodingBytecode::Id => {
                    // Do we need to encode an immediate?
                    if let Some(imm) = src_op.immediate() {
                        self.encode_imm(imm);
                    }
                }
                _ => unimplemented!("encoding: {:?}", code),
            }
        }

        Ok(())
    }

    fn encode_modrm_sib_bytes(
        &mut self,
        reg_op: Option<&Operand>,
        reg_memory_op: Option<&Operand>,
        modrm_reg: Option<u8>,
    ) -> Result<(), RasError> {
        let (modrm_reg, rm) = if let Some(modrm_reg) = modrm_reg {
            (
                modrm_reg,
                reg_op.map(|reg| reg.reg_num()).unwrap_or_default(),
            )
        } else {
            (
                reg_op.map(|reg| reg.reg_num()).unwrap_or_default(),
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
            let (base, index, is_disp32) = match (base.is_some(), index.is_some(), scale) {
                (true, _, _) => (base, index, false),
                (false, true, Scale::Byte) => (index, base, false),
                (false, false, _) => {
                    // disp32 case
                    (base, index, true)
                }
                _ => (base, index, false),
            };

            let (modifier, displacement) = match (is_disp32, displacement) {
                // In GNU as, expressions with missing base and index registers with no
                // displacement are the same as a 32-bit displacement of 0 (e.g. movb $0x2,(,2)
                // is the same as movb $0x2, 0)
                (true, v) => (0b00, Some(v.unwrap_or(0).to_le_bytes().to_vec())),
                // disp8
                (false, Some(v)) if v.next_power_of_two() < 256 => {
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
            MemoryRel::Immediate(_imm) => unimplemented!("imm memory offset"),
            MemoryRel::Label(symbol_id) => {
                let size = (operand_repr.size() / 8) as usize;
                let fixup = Fixup {
                    offset: self.instruction_pointer(),
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

    /// Encode `imm` to have the specified `size`, sign-extending if needed.
    fn encode_imm(&mut self, imm: Immediate) {
        // XXX: MOV r64, imm64 supports 64-bit immediates
        // Can't use more than 32 bits for the immediate.
        match imm {
            Immediate::Imm8(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
            Immediate::Imm16(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
            Immediate::Imm32(imm) => self.out.extend(imm.to_le_bytes().to_vec()),
        }
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

        (Some(base), None, modifier)
            if (modifier == 0b01 || modifier == 0b10) && matches!(**base, RegisterNum::Rsp) =>
        {
            Some(sib(scale as u8, SIB_INDEX_NONE, sib_base(*base)))
        }
        (Some(_), None, _) => None,
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
