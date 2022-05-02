use crate::assembler::Item;
use crate::error::{ParseError, ParseErrorKind, ParseErrorList};
use crate::instruction::Instruction;
use crate::operand::{Immediate, Memory, Moffs, Operand, Register, Scale};
use crate::Mnemonic;
use crate::ParseResult;

use std::convert::TryFrom;
use std::str::FromStr;

/// Parse assembly code written in AT&T syntax.
pub fn parse_asm(input: &str) -> Result<Vec<Item>, ParseErrorList> {
    let mut errors = vec![];
    let mut items = vec![];

    for (line, input) in input.split('\n').enumerate() {
        let input = input.trim();
        if input.is_empty() || input.starts_with('#') {
            continue;
        }

        match parse_line(input).map_err(|err| (line, err)) {
            Ok(item) => {
                if errors.is_empty() {
                    items.push(item)
                }
            }
            Err(e) => errors.push(e),
        }
    }

    if errors.is_empty() {
        Ok(items)
    } else {
        Err(errors.into())
    }
}

fn parse_line(input: &str) -> ParseResult<Item> {
    if let Some(label) = input.strip_suffix(':') {
        Ok(Item::Label(label.into()))
    } else {
        parse_instruction(input)
    }
}

fn parse_instruction(input: &str) -> ParseResult<Item> {
    let (mnemonic, operands) = match input.split_once(' ') {
        Some((mnemonic, operands)) => (
            Mnemonic::from_str(mnemonic)?,
            OperandParser::new(operands).parse()?,
        ),
        None => (Mnemonic::from_str(input)?, vec![]),
    };

    let inst = Instruction::new(mnemonic, operands);
    Ok(Item::Instruction(inst))
}

struct OperandParser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> OperandParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    pub fn parse(mut self) -> ParseResult<Vec<Operand>> {
        let mut operands = vec![];
        loop {
            let operand = self.parse_single_operand()?;
            operands.push(operand);

            if !self.skip_until_next_operand() {
                break;
            }
        }

        // Expect to reach the end of input after parsing the operands
        if self.pos != self.input.len() {
            return Err(ParseError::with_context(
                ParseErrorKind::JunkAfterExpression(
                    String::from_utf8(self.input[self.pos..].to_vec()).unwrap(),
                ),
                "invalid operand",
            ));
        }

        // AT&T syntax reverses the order of the operands:
        operands.reverse();

        Ok(operands)
    }

    fn parse_single_operand(&mut self) -> ParseResult<Operand> {
        if self.pos >= self.input.len() {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedEof,
                "missing operand",
            ));
        }
        match self.input[self.pos] {
            b'%' => self.parse_register().map(Operand::Register),
            b'$' => self.parse_immediate().map(Operand::Immediate),
            b'0'..=b'9' | b'(' => self.parse_memory().map(Operand::Memory),
            c => Err(ParseError::with_context(
                ParseErrorKind::UnexpectedChar(c.into()),
                "invalid operand",
            )),
        }
    }

    fn parse_register(&mut self) -> ParseResult<Register> {
        self.advance_or_eof()?;
        let start = self.pos;
        self.skip_while_alpha();
        Register::try_from(&self.input[start..self.pos])
    }

    fn parse_immediate(&mut self) -> ParseResult<Immediate> {
        self.advance_or_eof()?;
        let start = self.pos;
        self.skip_while_num();
        Immediate::try_from(&self.input[start..self.pos])
    }

    fn parse_memory(&mut self) -> ParseResult<Memory> {
        let start = self.pos;
        self.skip_while_num();

        let offset = if self.pos > start {
            Some(&self.input[start..self.pos])
        } else {
            None
        };

        if self.input[self.pos] == b'(' {
            self.advance_or_eof()?;
            self.parse_sib(
                offset
                    .map(|v| String::from_utf8_lossy(v).parse::<i64>())
                    .transpose()?,
            )
        } else {
            // It's safe to unwrap here, because parse_memory is only called if the next character
            // is a digit or an opening parenthesis.
            let moffs = offset.unwrap();
            let moffs = Moffs::try_from(moffs)?;
            Ok(Memory::Moffs(moffs))
        }
    }

    fn parse_sib(&mut self, displacement: Option<i64>) -> ParseResult<Memory> {
        if self.pos >= self.input.len() {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedEof,
                "failed to parse SIB expressions",
            ));
        }
        self.skip_whitespace();
        let base = self.maybe_parse_sib_register()?;
        // (%rax)
        //      ^
        if self.consume_char(b')').is_ok() {
            return Ok(Memory::sib(None, base, None, Scale::Byte, None));
        }
        let has_index_comma = self.consume_char(b',').is_ok();
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedEof,
                "expected closing bracket for SIB expression",
            ));
        }

        // (%rax, )
        //        ^
        if self.consume_char(b')').is_ok() {
            return Ok(Memory::sib(None, base, None, Scale::Byte, None));
        }

        // Missing comma, e.g.:
        // (%rax  %rcx)
        //      ^
        if !has_index_comma {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedChar(self.input[self.pos].into()),
                "missing comma before index register",
            ));
        }
        let (index, has_scale_comma) = if self.input[self.pos] == b'%' {
            let index = self.maybe_parse_sib_register()?;
            let has_scale_comma = self.consume_char(b',').is_ok();
            self.skip_whitespace();
            (index, has_scale_comma)
        } else {
            (None, has_index_comma)
        };

        // (%rax, %rcx,  )
        //              ^
        let maybe_scale = self.input[self.pos];
        let scale = match maybe_scale {
            b')' => {
                self.pos += 1;
                return Ok(Memory::sib(None, base, index, Scale::Byte, displacement));
            }
            b'1' | b'2' | b'4' | b'8' if !has_scale_comma => {
                return Err(ParseError::with_context(
                    ParseErrorKind::UnexpectedChar(maybe_scale.into()),
                    "missing comma before scale",
                ));
            }
            b'1' => Scale::Byte,
            b'2' => Scale::Word,
            b'4' => Scale::Double,
            b'8' => Scale::Quad,
            _ => {
                return Err(ParseError::with_context(
                    ParseErrorKind::UnexpectedChar(maybe_scale.into()),
                    "invalid scale",
                ))
            }
        };
        self.advance_or_eof()?;
        self.consume_char(b')')?;

        Ok(Memory::sib(None, base, index, scale, displacement))
    }

    fn maybe_parse_sib_register(&mut self) -> ParseResult<Option<Register>> {
        let reg = match self.input[self.pos] {
            b'%' => Some(self.parse_register()?),
            b',' => None,
            c => {
                return Err(ParseError::with_context(
                    ParseErrorKind::UnexpectedChar(c.into()),
                    "expected register",
                ))
            }
        };
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedEof,
                "expected register",
            ));
        }
        Ok(reg)
    }

    fn advance_or_eof(&mut self) -> ParseResult<()> {
        self.pos += 1;
        if self.pos >= self.input.len() {
            return Err(ParseError::new(ParseErrorKind::UnexpectedEof));
        }
        Ok(())
    }

    fn consume_char(&mut self, b: u8) -> ParseResult<()> {
        if self.input[self.pos] != b {
            return Err(ParseError::with_context(
                ParseErrorKind::UnexpectedChar(self.input[self.pos].into()),
                format!("expected {}", b as char),
            ));
        }
        self.pos += 1;
        Ok(())
    }

    fn skip_while_alpha(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
    }

    fn skip_while_num(&mut self) {
        if self.input[self.pos] == b'-' {
            self.pos += 1;
        }
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn skip_until_next_operand(&mut self) -> bool {
        if self.pos >= self.input.len() {
            return false;
        }
        let has_more_operands = self.input[self.pos] == b',';
        if has_more_operands {
            self.pos += 1;
        }
        self.skip_whitespace();
        has_more_operands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{i, imm16, imm32, imm8, reg, RAX, RCX};

    #[test]
    fn no_operands() {
        let insts = "pop";
        assert_eq!(parse_line(insts).unwrap(), Item::Instruction(i!(POP)));
    }

    #[test]
    fn label() {
        assert_eq!(parse_line("pop:").unwrap(), Item::Label("pop".into()));
    }

    #[test]
    fn label_with_spaces() {
        assert_eq!(
            parse_line("hello labels:").unwrap(),
            Item::Label("hello labels".into())
        );
    }

    #[test]
    fn invalid_register() {
        assert_eq!(
            parse_line("pop %").unwrap_err().kind(),
            &ParseErrorKind::UnexpectedEof
        );
        assert_eq!(
            parse_line("pop %rex").unwrap_err().kind(),
            &ParseErrorKind::InvalidRegister("rex".into())
        );
    }

    #[test]
    fn invalid_mnemonic() {
        assert_eq!(
            parse_line("").unwrap_err().kind(),
            &ParseErrorKind::InvalidMnemonic("".into())
        );
        assert_eq!(
            parse_line("plop").unwrap_err().kind(),
            &ParseErrorKind::InvalidMnemonic("plop".into())
        );
    }

    #[test]
    fn add_register_direct() {
        assert_eq!(
            parse_line("add %rcx, %rax").unwrap(),
            Item::Instruction(i!(ADD, reg!(RAX), reg!(RCX)))
        );
    }

    #[test]
    fn xor_imm() {
        assert_eq!(
            parse_line("xor $127, $2").unwrap(),
            Item::Instruction(i!(XOR, imm8!(2), imm8!(127)))
        );
        assert_eq!(
            parse_line("xor $128, $2").unwrap(),
            Item::Instruction(i!(XOR, imm8!(2), imm16!(128)))
        );
        assert_eq!(
            parse_line("xor $65536, $2").unwrap(),
            Item::Instruction(i!(XOR, imm8!(2), imm32!(65536)))
        );
    }
}
