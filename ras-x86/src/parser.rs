use crate::assembler::Item;
use crate::error::ParseError;
use crate::instruction::Instruction;
use crate::operand::{Immediate, Memory, Moffs, Operand, Register, Scale};
use crate::Mnemonic;
use crate::{RasError, RasResult};

use std::convert::TryFrom;
use std::str::FromStr;

/// Parse assembly code written in AT&T syntax.
pub fn parse_asm(input: &str) -> RasResult<Vec<Item>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.trim())
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> RasResult<Item> {
    if let Some(label) = input.strip_suffix(':') {
        Ok(Item::Label(label.into()))
    } else {
        parse_inst(input)
    }
}

fn parse_inst(input: &str) -> RasResult<Item> {
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

    pub fn parse(mut self) -> RasResult<Vec<Operand>> {
        let mut operands = vec![];
        loop {
            let operand = self.parse_single_operand()?;
            operands.push(operand);

            if !self.skip_until_next_operand() {
                break;
            }
        }

        // AT&T syntax reverses the order of the operands:
        operands.reverse();

        Ok(operands)
    }

    fn parse_single_operand(&mut self) -> RasResult<Operand> {
        if self.pos >= self.input.len() {
            return Err(ParseError::UnexpectedEof.into());
        }
        match self.input[self.pos] {
            b'%' => self.parse_register().map(Operand::Register),
            b'$' => self.parse_immediate().map(Operand::Immediate),
            b'0'..=b'9' | b'(' => self.parse_memory().map(Operand::Memory),
            c => Err(ParseError::UnexpectedChar(c.into()).into()),
        }
    }

    fn parse_register(&mut self) -> RasResult<Register> {
        self.advance_or_eof()?;
        let start = self.pos;
        self.skip_while_alpha();
        Register::try_from(&self.input[start..self.pos]).map_err(RasError::from)
    }

    fn parse_immediate(&mut self) -> RasResult<Immediate> {
        self.advance_or_eof()?;
        let start = self.pos;
        self.skip_while_num();
        Immediate::try_from(&self.input[start..self.pos]).map_err(RasError::from)
    }

    fn parse_memory(&mut self) -> RasResult<Memory> {
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

    fn parse_sib(&mut self, displacement: Option<i64>) -> RasResult<Memory> {
        if self.pos >= self.input.len() {
            return Err(ParseError::UnexpectedEof.into());
        }
        self.skip_whitespace();
        let mut base = self.maybe_parse_sib_register()?;
        let has_index_comma = self.consume_char(b',').is_ok();
        self.skip_whitespace();
        if self.input[self.pos] == b')' {
            return Ok(Memory::sib(None, base, None, Scale::Byte, None));
        }

        // missing comma
        if !has_index_comma {
            return Err(ParseError::UnexpectedChar(self.input[self.pos].into()).into());
        }
        let (mut index, has_scale_comma) = if self.input[self.pos] == b'%' {
            let index = self.maybe_parse_sib_register()?;
            let has_scale_comma = self.consume_char(b',').is_ok();
            self.skip_whitespace();
            (index, has_scale_comma)
        } else {
            (None, has_index_comma)
        };
        if index.is_none() {
            std::mem::swap(&mut base, &mut index);
        }
        if self.input[self.pos] == b')' {
            return Ok(Memory::sib(None, base, index, Scale::Byte, None));
        }

        let maybe_scale = self.input[self.pos];
        let scale = match maybe_scale {
            b')' => {
                self.pos += 1;
                return Ok(Memory::sib(None, base, index, Scale::Byte, displacement));
            }
            b'1' | b'2' | b'4' | b'8' if !has_scale_comma => {
                return Err(ParseError::UnexpectedChar(maybe_scale.into()).into())
            }
            b'1' => Scale::Byte,
            b'2' => Scale::Word,
            b'4' => Scale::Double,
            b'8' => Scale::Quad,
            _ => return Err(ParseError::UnexpectedChar(maybe_scale.into()).into()),
        };
        self.advance_or_eof()?;
        self.consume_char(b')')?;

        Ok(Memory::sib(None, base, index, scale, displacement))
    }

    fn maybe_parse_sib_register(&mut self) -> RasResult<Option<Register>> {
        let reg = match self.input[self.pos] {
            b'%' => Some(self.parse_register()?),
            b',' => None,
            c => return Err(ParseError::UnexpectedChar(c.into()).into()),
        };
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Err(ParseError::UnexpectedEof.into());
        }
        Ok(reg)
    }

    fn advance_or_eof(&mut self) -> RasResult<()> {
        self.pos += 1;
        if self.pos >= self.input.len() {
            return Err(ParseError::UnexpectedEof.into());
        }
        Ok(())
    }

    fn consume_char(&mut self, b: u8) -> RasResult<()> {
        if self.input[self.pos] != b {
            return Err(ParseError::UnexpectedChar(self.input[self.pos].into()).into());
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
    use crate::{i, imm16, imm32, imm8, reg, RasError, RAX, RCX};

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
            parse_line("pop %").unwrap_err(),
            RasError::Parse(ParseError::UnexpectedEof)
        );
        assert_eq!(
            parse_line("pop %rex").unwrap_err(),
            RasError::Parse(ParseError::InvalidRegister("rex".into()))
        );
    }

    #[test]
    fn invalid_mnemonic() {
        assert_eq!(
            parse_line("").unwrap_err(),
            RasError::Parse(ParseError::InvalidMnemonic("".into()))
        );
        assert_eq!(
            parse_line("plop").unwrap_err(),
            RasError::Parse(ParseError::InvalidMnemonic("plop".into()))
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
