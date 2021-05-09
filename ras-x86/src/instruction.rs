use crate::encoder::Encoder;
use crate::operand::Operand;
use crate::repr::instruction::InstructionRepr;
use crate::repr::mnemonic::Mnemonic;
use crate::{RasError, RasResult};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref INSTR_REPRS: HashMap<Mnemonic, Vec<InstructionRepr>> = {
        let inst_map = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("bin/map")).unwrap();

        bincode::deserialize(&inst_map).unwrap()
    };
}

pub struct Instruction {
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, operands: Vec<Operand>) -> Self {
        assert!(operands.len() <= 4);

        Self { mnemonic, operands }
    }

    pub fn encode(&self, enc: &mut Encoder) -> RasResult<()> {
        let variants = (*INSTR_REPRS).get(&self.mnemonic).unwrap();

        // Find the best instruction encoding (always choose the encoding with the smallest operand
        // sizes).
        let inst_repr = variants
            .into_iter()
            .filter(|variant| Self::matches(variant, &self.operands))
            .reduce(|inst_a, inst_b| {
                let mut found_better = false;
                for (op_repr_a, op_repr_b) in inst_a.operands.iter().zip(inst_b.operands.iter()) {
                    if op_repr_b.size() > op_repr_a.size() {
                        return inst_a;
                    } else if op_repr_b.size() < op_repr_a.size() {
                        found_better = true;
                    }
                }

                if found_better {
                    inst_b
                } else {
                    inst_a
                }
            });

        enc.encode(
            inst_repr.ok_or(RasError::MissingInstructionRepr(self.mnemonic))?,
            &self.operands,
        )?;
        Ok(())
    }

    /// Check if the operands can be encoded according to this `InstructionRepr`.
    pub fn matches(repr: &InstructionRepr, operands: &[Operand]) -> bool {
        if repr.operands.len() != operands.len() {
            return false;
        }

        operands
            .iter()
            .zip(repr.operands.iter())
            .all(|(op, op_enc)| op.can_encode(op_enc))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::register::{RAX, RBX, RCX};

    macro_rules! encode_instr {
        ($opcode:ident, $($operands:expr),*) => {{
            let mut enc = Encoder::default();
            Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            ).encode(&mut enc).unwrap();

            enc.out
        }}
    }

    #[test]
    fn register_add() {
        assert_eq!(
            encode_instr!(ADD, Operand::Register(*RAX), Operand::Register(*RCX)),
            vec![0x48, 0x01, 0xc8]
        );

        assert_eq!(
            encode_instr!(ADD, Operand::Register(*RAX), Operand::Register(*RBX)),
            vec![0x48, 0x01, 0xd8]
        );
    }
}
