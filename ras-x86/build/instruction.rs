use crate::parsers::{operand_repr, repeat, seq, tok, until, ParseResult};
use ras_x86_repr::OperandRepr;

pub fn parse_instruction_column(inst: &str) -> ParseResult<(String, Vec<OperandRepr>)> {
    let is_operand_separator = |c| c == ' ' || c == ',';
    let operand_parser = operand_repr(is_operand_separator);
    let parse_instruction = seq(
        tok(until(char::is_whitespace), char::is_whitespace),
        repeat(tok(operand_parser, is_operand_separator)),
    );
    let ((mnemonic, operands), _) = parse_instruction(inst)?;
    Ok((mnemonic.to_string(), operands))
}
