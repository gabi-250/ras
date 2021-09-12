use crate::assembler::Item;
use crate::error::ParseError;
use crate::instruction::Instruction;
use crate::operand::{Immediate, Memory, Operand, Register};
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
            return Err(ParseError::UnexpectedEof).map_err(RasError::from);
        }

        match self.input[self.pos] {
            b'%' => {
                self.pos += 1;
                self.parse_register().map(Operand::Register)
            }
            b'$' => {
                self.pos += 1;
                self.parse_immediate().map(Operand::Immediate)
            }
            c => Err(ParseError::UnexpectedChar(c.into()).into()),
        }
    }

    fn parse_register(&mut self) -> RasResult<Register> {
        let start = self.pos;
        self.skip_while_alnum();
        Register::try_from(&self.input[start..self.pos]).map_err(RasError::from)
    }

    fn parse_immediate(&mut self) -> RasResult<Immediate> {
        let start = self.pos;
        self.skip_while_num();
        Immediate::try_from(&self.input[start..self.pos]).map_err(RasError::from)
    }

    fn parse_memory(&mut self) -> RasResult<Memory> {
        unimplemented!();
    }

    fn skip_while_alnum(&mut self) {
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

    fn skip_until_next_operand(&mut self) -> bool {
        if self.pos >= self.input.len() {
            return false;
        }

        let has_more_operands = self.input[self.pos] == b',';

        if has_more_operands {
            self.pos += 1;
        }

        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

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
            RasError::Parse(ParseError::InvalidRegister("".into()))
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
