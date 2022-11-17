use crate::operand::OperandRepr;
use crate::prefix::RexPrefix;
use crate::Mode;

use serde::{Deserialize, Serialize};

use std::cmp::{Ordering, PartialOrd};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionRepr {
    /// The recipe for instruction encoding.
    pub encoding: InstructionEncoding,
    /// The representation of the operands.
    pub operands: Vec<OperandRepr>,
    /// The assembly [`Mode`](enum.Mode.html)s in which this instruction's encoding is possible.
    pub modes: Vec<Mode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionEncoding {
    /// The bytecodes describing how the instruction's encoding.
    pub bytecode: Vec<EncodingBytecode>,
    /// According to the [Intel® 64 and IA-32 architectures software developer's manual volume 2]: "Indicates
    /// the use of 66/F2/F3 prefixes (beyond those already part of the instructions opcode) are not
    /// allowed with the instruction. Such use will either cause an invalid-opcode exception (#UD)
    /// or result in the encoding for a different instruction."
    ///
    /// [Intel® 64 and IA-32 architectures software developer's manual volume 2]: https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html
    pub is_np: bool,
}

impl InstructionRepr {
    pub fn new(
        encoding: InstructionEncoding,
        operands: Vec<OperandRepr>,
        modes: Vec<Mode>,
    ) -> Self {
        Self {
            encoding,
            operands,
            modes,
        }
    }

    /// Returns `true` if the instruction is valid in the specified mode.
    pub fn is_valid_in_mode(&self, mode: &Mode) -> bool {
        self.modes.contains(mode)
    }

    /// Returns `true` if the data is full-sized.
    ///
    /// The data can be byte or full-sized, where full-sized is 16 or 32 bits. This information is
    /// extracted from the w bit of the opcode (if w = 0, the data is byte-sized).
    pub fn is_full_sized(&self) -> bool {
        // XXX do we really need this?
        true
    }

    fn cmp_size(&self, other: &Self) -> Ordering {
        let has_smaller_op_sizes = self
            .operands
            .iter()
            .zip(other.operands.iter())
            .any(|(op, other_op)| op.size().cmp(&other_op.size()) == Ordering::Less);

        if has_smaller_op_sizes {
            return Ordering::Less;
        }

        self.encoding
            .bytecode
            .len()
            .cmp(&other.encoding.bytecode.len())
    }
}

impl PartialEq for InstructionRepr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_size(other) == Ordering::Equal
    }
}

impl Eq for InstructionRepr {}

// Used for sorting `InstructionRepr`s by size.
impl PartialOrd for InstructionRepr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_size(other))
    }
}

impl Ord for InstructionRepr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_size(other)
    }
}

impl InstructionEncoding {
    pub fn new(bytecode: Vec<EncodingBytecode>, is_np: bool) -> Self {
        Self { bytecode, is_np }
    }
}

/// An instruction encoding bytecode.
///
/// Bytecodes are extracted from the `Opcode` column of the [x86-csv].
///
/// See Section "3.1.1.1 Opcode Column in the Instruction Summary Table (Instructions without VEX
/// Prefix)" of the [Intel® 64 and IA-32 architectures software developer's manual volume 2] for
/// more details.
///
/// [x86-csv]: https://github.com/GregoryComer/x86-csv/tree/c638bbbaa17f0c81abaa7e84a968335c985542fa
/// [Intel® 64 and IA-32 architectures software developer's manual volume 2]: https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingBytecode {
    Rex(RexPrefix),
    Prefix(u8),
    Opcode(u8),
    OpcodeRb(u8),
    OpcodeRw(u8),
    OpcodeRd(u8),
    OpcodeRo(u8),
    ModRm,
    ModRmWithReg(u8),
    Ib,
    Iw,
    Id,
    Cb,
    Cw,
    Cd,
    Cp,
    Co,
    Ct,
}

impl FromStr for EncodingBytecode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ib" => Ok(EncodingBytecode::Ib),
            "iw" => Ok(EncodingBytecode::Iw),
            "id" => Ok(EncodingBytecode::Id),
            "cb" => Ok(EncodingBytecode::Cb),
            "cw" => Ok(EncodingBytecode::Cw),
            "cd" => Ok(EncodingBytecode::Cd),
            "co" => Ok(EncodingBytecode::Co),
            "cp" => Ok(EncodingBytecode::Cp),
            "ct" => Ok(EncodingBytecode::Ct),
            "/r" => Ok(EncodingBytecode::ModRm),
            "/0" => Ok(EncodingBytecode::ModRmWithReg(0)),
            "/1" => Ok(EncodingBytecode::ModRmWithReg(1)),
            "/2" => Ok(EncodingBytecode::ModRmWithReg(2)),
            "/3" => Ok(EncodingBytecode::ModRmWithReg(3)),
            "/4" => Ok(EncodingBytecode::ModRmWithReg(4)),
            "/5" => Ok(EncodingBytecode::ModRmWithReg(5)),
            "/6" => Ok(EncodingBytecode::ModRmWithReg(6)),
            "/7" => Ok(EncodingBytecode::ModRmWithReg(7)),
            "/8" => Ok(EncodingBytecode::ModRmWithReg(8)),
            "/9" => Ok(EncodingBytecode::ModRmWithReg(9)),
            _ => Err(format!("failed to parse EncodingBytecode: {}", s)),
        }
    }
}
