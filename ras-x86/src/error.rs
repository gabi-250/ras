use crate::context::SymbolId;
use crate::repr::mnemonic::Mnemonic;

use object::write;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum RasError {
    Encoding(String),
    DuplicateLabel(SymbolId),
    UnknownLabel(SymbolId),
    MissingInstructionRepr(Mnemonic),
    Object(write::Error),
    Io(io::Error),
}

impl Display for RasError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use RasError::*;

        match self {
            Encoding(err) => write!(f, "encoding error: {}", err),
            DuplicateLabel(label) => write!(f, "duplicate label: {}", label),
            UnknownLabel(label) => write!(f, "unknown label: {}", label),
            MissingInstructionRepr(mnemonic) => {
                write!(f, "failed to select instruction repr for {:?}", mnemonic)
            }
            Object(err) => write!(f, "{}", err),
            Io(err) => write!(f, "{}", err),
        }
    }
}

impl Error for RasError {}

impl From<write::Error> for RasError {
    fn from(err: write::Error) -> Self {
        Self::Object(err)
    }
}

impl From<io::Error> for RasError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
