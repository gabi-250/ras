use crate::assembler::SymbolTable;
use crate::encoder::Encoder;
use crate::mnemonic::Mnemonic;
use crate::operand::Operand;
use crate::repr::instruction::InstructionRepr;
use crate::{RasError, RasResult};
use std::str::FromStr;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const MAX_OPERAND_COUNT: usize = 4;
const INST_MAP: &str = "inst_map.json";

lazy_static! {
    pub static ref INSTR_REPRS: HashMap<Mnemonic, Vec<InstructionRepr>> = {
        let inst_map = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_MAP)).unwrap();
        let map: Vec<(String, Vec<InstructionRepr>)> = serde_json::from_slice(&inst_map).unwrap();

        map.into_iter()
            .map(|(mnemonic, repr)| (Mnemonic::from_str(&mnemonic).unwrap(), repr))
            .collect()
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, operands: Vec<Operand>) -> Self {
        assert!(operands.len() <= MAX_OPERAND_COUNT);

        Self { mnemonic, operands }
    }

    pub(crate) fn encode(&self, enc: &mut Encoder, _sym_tab: &SymbolTable) -> RasResult<()> {
        let variants = (*INSTR_REPRS).get(&self.mnemonic).unwrap();

        // Find the best instruction encoding (always choose the encoding with the smallest operand
        // sizes).
        let mut instructions = variants
            .iter()
            .filter(|variant| enc.is_encodable(variant) && self.encodable_with(variant))
            .collect::<Vec<_>>();

        // Sort the instructions by their estimated encoding length:
        instructions.sort();

        // Pick the best encoding:
        // TODO: use sym_tab to determine the operand size for relative offset operands
        let shortest_repr = instructions
            .first()
            .ok_or(RasError::MissingInstructionRepr(self.mnemonic))?;

        enc.encode(shortest_repr, &self.operands)
    }

    /// Check if the operands of this instruction can be encoded by the specified `InstructionRepr`.
    fn encodable_with(&self, repr: &InstructionRepr) -> bool {
        if self.operands.len() != repr.operands.len() {
            return false;
        }

        self.operands
            .iter()
            .zip(repr.operands.iter())
            .all(|(op, op_enc)| op.can_encode(op_enc))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{RAX, RBX, RCX};

    macro_rules! encode_instr {
        ($opcode:ident, $($operands:expr),*) => {{
            let mut enc = Encoder::default();

            Instruction::new(
                Mnemonic::$opcode,
                vec![$($operands,)*]
            ).encode(&mut enc, &Default::default()).unwrap();

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
