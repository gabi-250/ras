mod instruction;
mod instruction_map;
mod operand;

pub(crate) use instruction::InstructionRepr;
pub(crate) use instruction_map::INSTR_REPRS;
pub(crate) use operand::{OperandKind, OperandRepr};
