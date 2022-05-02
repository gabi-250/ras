use crate::mnemonic::Mnemonic;
use crate::symbol::SymbolId;

use object::write;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseErrorList(Vec<(usize, ParseError)>);

impl Error for ParseErrorList {}

impl Display for ParseErrorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let msg = self
            .0
            .iter()
            .map(|(line, e)| format!("{}: {}", line, e))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", msg)
    }
}

impl From<Vec<(usize, ParseError)>> for ParseErrorList {
    fn from(errors: Vec<(usize, ParseError)>) -> Self {
        Self(errors)
    }
}

#[derive(Debug)]
pub enum RasError {
    Encoding(String),
    DuplicateLabel(SymbolId),
    UndefinedSymbols(Vec<SymbolId>),
    MissingInstructionRepr(Mnemonic),
    Object(write::Error),
    Io(io::Error),
    SignExtend(String),
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
            SignExtend(err) => write!(f, "sign extend error: {}", err),
        }
    }
}

impl PartialEq for RasError {
    fn eq(&self, other: &Self) -> bool {
        use RasError::*;

        match (self, other) {
            // There's no PartialEq impl for `io::Error`s.
            (Io(_), Io(_)) => false,
            (Encoding(s1), Encoding(s2)) => s1 == s2,
            (DuplicateLabel(s1), DuplicateLabel(s2)) => s1 == s2,
            (UndefinedSymbols(s1), UndefinedSymbols(s2)) => s1 == s2,
            (MissingInstructionRepr(s1), MissingInstructionRepr(s2)) => s1 == s2,
            (Object(s1), Object(s2)) => s1 == s2,
            (SignExtend(z1), SignExtend(z2)) => z1 == z2,
            _ => false,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if !self.ctx.is_empty() {
            write!(f, "{}: ", self.ctx)?;
        }
        match &self.kind {
            ParseErrorKind::InvalidMnemonic(m) => write!(f, "unknown mnemonic '{}'", m),
            ParseErrorKind::InvalidRegister(r) => write!(f, "invalid register '{}'", r),
            ParseErrorKind::InvalidImmediate(imm) => write!(f, "invalid immediate '{}'", imm),
            ParseErrorKind::InvalidMemoryOffset(moffs) => {
                write!(f, "invalid memory offset: {}", moffs)
            }
            ParseErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseErrorKind::UnexpectedChar(c) => write!(f, "found unexpected char '{}'", c),
            ParseErrorKind::ParseInt(err) => write!(f, "{}", err),
            ParseErrorKind::JunkAfterExpression(s) => {
                write!(f, "found junk after expression {}", s)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    kind: ParseErrorKind,
    ctx: String,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> Self {
        Self {
            kind,
            ctx: "".into(),
        }
    }

    pub fn with_context(kind: ParseErrorKind, ctx: impl AsRef<str>) -> Self {
        Self {
            kind,
            ctx: ctx.as_ref().into(),
        }
    }

    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    pub fn ctx(&self) -> &str {
        &self.ctx
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError {
            kind: ParseErrorKind::ParseInt(err),
            ctx: "".into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorKind {
    InvalidMnemonic(String),
    InvalidImmediate(String),
    InvalidMemoryOffset(String),
    InvalidRegister(String),
    UnexpectedEof,
    UnexpectedChar(char),
    ParseInt(ParseIntError),
    JunkAfterExpression(String),
}
