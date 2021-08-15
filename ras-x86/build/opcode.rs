use crate::parsers::{
    alt, encoding_bytecode, hex_byte, lit, map, opt, repeat_until, seq, tok, ParseResult,
};
use ras_x86_repr::{EncodingBytecode, InstructionEncoding, RexPrefix};
use std::str::{self, FromStr};

pub fn parse_opcode_column(inst: &str) -> ParseResult<InstructionEncoding> {
    let mut bytecode = vec![];
    let parse_np = map(tok(opt(lit("NP")), |c| is_separator(c)), |out| {
        out.is_some()
    });
    let parse_mandatory_prefix = map(
        tok(opt(alt(lit("66"), alt(lit("F2"), lit("F3")))), |c| {
            is_separator(c)
        }),
        |prefix| {
            prefix.map(|prefix| EncodingBytecode::Prefix(u8::from_str_radix(prefix, 16).unwrap()))
        },
    );

    let (is_np, inst) = parse_np(inst)?;
    let (prefix, inst) = parse_mandatory_prefix(inst)?;
    if let Some(prefix) = prefix {
        bytecode.push(prefix);
    }

    let parse_rex_prefix = map(
        tok(
            opt(alt(
                alt(
                    lit("REX.W"),
                    alt(lit("REX.R"), alt(lit("REX.X"), lit("REX.B"))),
                ),
                lit("REX"),
            )),
            |c| is_separator(c),
        ),
        |prefix| {
            prefix.map(|prefix| {
                let prefix = RexPrefix::from_str(prefix).expect("missing REX prefix");
                EncodingBytecode::Rex(prefix)
            })
        },
    );

    let (rex_prefix, inst) = parse_rex_prefix(inst)?;

    if let Some(rex_prefix) = rex_prefix {
        bytecode.push(rex_prefix);
    }

    let parse_opcode = repeat_until(
        tok(
            alt(
                map(hex_byte(), |out| EncodingBytecode::Opcode(out)),
                encoding_bytecode(),
            ),
            |c| c == ' ',
        ),
        map(
            seq(
                tok(hex_byte(), |c| c == ' '),
                alt(lit("+rb"), alt(lit("+rd"), lit("+rw"))),
            ),
            |(out, suffix)| match suffix {
                "+rb" => EncodingBytecode::OpcodeRb(out),
                "+rd" => EncodingBytecode::OpcodeRd(out),
                "+rw" => EncodingBytecode::OpcodeRw(out),
                s => unreachable!("invalid suffix: {}", s),
            },
        ),
    );

    let ((opcodes, opcode), _) = parse_opcode(inst)?;
    bytecode.extend_from_slice(&opcodes);
    if let Some(opcode) = opcode {
        bytecode.push(opcode);
    }
    // TODO parse the remaining parts of the column
    Ok(InstructionEncoding::new(bytecode, is_np))
}

fn is_separator(c: char) -> bool {
    c == ' ' || c == '+'
}
