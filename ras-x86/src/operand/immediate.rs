use crate::error::ParseError;
use crate::{RasError, RasResult};

use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Immediate {
    Imm8(i8),
    Imm16(i16),
    Imm32(i32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImmediateSize {
    Imm8,
    Imm16,
    Imm32,
}

impl Immediate {
    pub fn size(&self) -> u32 {
        match self {
            Self::Imm8(_) => 8,
            Self::Imm16(_) => 16,
            Self::Imm32(_) => 32,
        }
    }

    pub fn sign_extend(self, size: ImmediateSize) -> RasResult<Self> {
        use Immediate::*;
        let imm = match (self, size) {
            (Imm8(imm), ImmediateSize::Imm8) => Imm8(imm),
            (Imm8(imm), ImmediateSize::Imm16) => Imm16(imm as i16),
            (Imm8(imm), ImmediateSize::Imm32) => Imm32(imm as i32),
            (Imm16(imm), ImmediateSize::Imm16) => Imm16(imm),
            (Imm16(imm), ImmediateSize::Imm32) => Imm32(imm as i32),
            (Imm32(imm), ImmediateSize::Imm32) => Imm32(imm),
            (imm, size) => {
                return Err(RasError::SignExtend(format!(
                    "immediate {:?} to size {:?}",
                    imm, size
                )));
            }
        };

        Ok(imm)
    }
}

impl TryFrom<&[u8]> for Immediate {
    type Error = ParseError;

    fn try_from(imm: &[u8]) -> Result<Self, Self::Error> {
        let imm = String::from_utf8_lossy(imm);
        let imm = imm.as_ref();
        if let Ok(imm) = imm.parse::<i8>() {
            Ok(Immediate::Imm8(imm))
        } else if let Ok(imm) = imm.parse::<i16>() {
            Ok(Immediate::Imm16(imm))
        } else if let Ok(imm) = imm.parse::<i32>() {
            Ok(Immediate::Imm32(imm))
        } else {
            Err(ParseError::InvalidImmediate(imm.into()))
        }
    }
}
