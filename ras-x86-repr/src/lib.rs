//! The internal representation of an x86 instruction.
//!
//! The [`InstructionRepr`](instruction/struct.InstructionRepr.html) is the most important
//! structure in this crate. It has the following components:
//! * an [`InstructionEncoding`](instruction/struct.InstructionEncoding.html), which specifies how the
//!   instruction should be encoded
//! * a list of [`OperandRepr`](operand/struct.OperandRepr.html)s
//!   (the representation of its operands), and
//! * the assembly [`Mode`](enum.Mode.html)s in which its encoding is possible

pub mod instruction;
pub mod operand;
pub mod prefix;

use serde::{Deserialize, Serialize};

pub use instruction::{EncodingBytecode, InstructionEncoding, InstructionRepr};
pub use operand::{OperandKind, OperandRepr};
pub use prefix::{Prefix, RexPrefix};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    /// 16-bit real mode.
    Real,
    /// 32-bit protected mode.
    Protected,
    /// 64-bit long mode.
    Long,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Long
    }
}
