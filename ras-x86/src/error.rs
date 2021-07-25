use crate::mnemonic::Mnemonic;
use crate::symbol::SymbolId;

use object::write;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum RasError {
    Encoding(String),
    DuplicateLabel(SymbolId),
    UndefinedSymbols(Vec<SymbolId>),
    MissingInstructionRepr(Mnemonic),
    Object(write::Error),
    Io(io::Error),
}

impl PartialEq for RasError {
    fn eq(&self, other: &Self) -> bool {
        use RasError::*;

        match (self, other) {
            // There's no PartialEq impl for `io::Error`s.
            (Io(_), Io(_)) => true,
            (Encoding(s1), Encoding(s2)) => s1 == s2,
            (DuplicateLabel(s1), DuplicateLabel(s2)) => s1 == s2,
            (UndefinedSymbols(s1), UndefinedSymbols(s2)) => s1 == s2,
            (MissingInstructionRepr(s1), MissingInstructionRepr(s2)) => s1 == s2,
            (Object(s1), Object(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Display for RasError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use RasError::*;

        match self {
            Encoding(err) => write!(f, "encoding error: {}", err),
            DuplicateLabel(label) => write!(f, "duplicate label: {}", label),
            UndefinedSymbols(symbols) => {
                for symbol in symbols {
                    writeln!(f, "symbol {} undefined", symbol)?;
                }

                Ok(())
            }
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
