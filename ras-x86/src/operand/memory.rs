use crate::error::{ParseError, ParseErrorKind};
use crate::operand::{Immediate, Register};
use crate::symbol::SymbolId;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Memory {
    Sib {
        /// XXX
        segment_override: Option<Register>,
        /// Any GPR.
        base: Option<Register>,
        /// Any GPR except ESP/RSP.
        index: Option<Register>,
        /// The multiplier (one of 1, 2, 4, or 8).
        scale: Scale,
        /// Usually an 8-, 16-, or 32-bit value, although some rare instructions take a 64-bit
        /// displacement.
        displacement: Option<i64>,
    },
    Relative(MemoryRel),
    /// Only valid for MOV instructions
    Moffs(Moffs),
}

/// The scale used in a SIB expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Byte = 0,
    Word = 0b01,
    Double = 0b10,
    Quad = 0b11,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Byte
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryRel {
    Absolute(Immediate),
    Label(SymbolId),
}

impl Memory {
    pub fn sib(
        segment_override: Option<Register>,
        base: Option<Register>,
        index: Option<Register>,
        scale: Scale,
        displacement: Option<i64>,
    ) -> Self {
        Self::Sib {
            segment_override,
            base,
            index,
            scale,
            displacement,
        }
    }

    pub fn relative(mem_rel: MemoryRel) -> Self {
        Self::Relative(mem_rel)
    }

    pub fn moffs(moffs: Moffs) -> Self {
        Self::Moffs(moffs)
    }

    pub fn is_sib(&self) -> bool {
        matches!(&self, Memory::Sib { .. })
    }

    pub fn is_relative(&self) -> bool {
        matches!(&self, Memory::Relative(_))
    }

    pub fn is_moffs(&self) -> bool {
        matches!(&self, Memory::Moffs(_))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Moffs {
    Moffs8(u8),
    Moffs16(u16),
    Moffs32(u32),
    Moffs64(u64),
}

impl Moffs {
    pub fn size(&self) -> u32 {
        match self {
            Self::Moffs8(_) => 8,
            Self::Moffs16(_) => 16,
            Self::Moffs32(_) => 32,
            Self::Moffs64(_) => 64,
        }
    }
}

impl TryFrom<&[u8]> for Moffs {
    type Error = ParseError;

    fn try_from(moffs: &[u8]) -> Result<Self, Self::Error> {
        let moffs = String::from_utf8_lossy(moffs);
        let moffs = moffs.as_ref();
        if let Ok(moffs) = moffs.parse::<u8>() {
            Ok(Moffs::Moffs8(moffs))
        } else if let Ok(moffs) = moffs.parse::<u16>() {
            Ok(Moffs::Moffs16(moffs))
        } else if let Ok(moffs) = moffs.parse::<u32>() {
            Ok(Moffs::Moffs32(moffs))
        } else if let Ok(moffs) = moffs.parse::<u64>() {
            Ok(Moffs::Moffs64(moffs))
        } else {
            Err(ParseError::new(ParseErrorKind::InvalidMemoryOffset(
                moffs.into(),
            )))
        }
    }
}
