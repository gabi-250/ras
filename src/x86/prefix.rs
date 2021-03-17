use std::str::FromStr;

/// REX bits: 0100WRXB
const REX: u8 = 0b1000000;
const REX_W: u8 = 0b1001000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RexPrefix {
    None,
    W,
    R,
    X,
    B,
}

impl Into<u8> for RexPrefix {
    fn into(self) -> u8 {
        match self {
            RexPrefix::W => REX | REX_W,
            RexPrefix::None => REX,
            prefix => unimplemented!("prefix={:?}", prefix),
        }
    }
}

impl FromStr for RexPrefix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "REX" => Ok(RexPrefix::None),
            "REX.W" => Ok(RexPrefix::W),
            "REX.R" => Ok(RexPrefix::R),
            "REX.X" => Ok(RexPrefix::X),
            "REX.B" => Ok(RexPrefix::B),
            s => Err(format!("failed to parse REX prefix: {}", s)),
        }
    }
}
