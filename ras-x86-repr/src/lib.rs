pub mod instruction;
pub mod mnemonic;
pub mod operand;
pub mod prefix;

use serde::{Deserialize, Serialize};

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
