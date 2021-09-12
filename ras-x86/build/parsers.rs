use ras_x86_repr::{EncodingBytecode, OperandRepr};
use std::str::{self, FromStr};

pub type ParseResult<T> = Result<T, ParseError>;
pub type ParseError = String;

pub fn hex_byte() -> impl Fn(&str) -> ParseResult<(u8, &str)> {
    move |input| {
        if input.len() < 2 {
            return Err(format!("invalid hex byte: {}", input));
        }
        // e.g. "cb" should not be parsed as a hex byte
        if input[..2].chars().any(char::is_lowercase) {
            return Err("hex digits must be uppercase chars".into());
        }

        match u8::from_str_radix(&input[..2], 16) {
            Ok(num) => Ok((num, &input[2..])),
            Err(_) => Err(format!("invalid hex byte: {}", input)),
        }
    }
}

pub fn encoding_bytecode() -> impl Fn(&str) -> ParseResult<(EncodingBytecode, &str)> {
    move |input| {
        if input.len() < 2 {
            return Err(format!("invalid hex byte: {}", input));
        }
        EncodingBytecode::from_str(&input[..2]).map(|bytecode| (bytecode, &input[2..]))
    }
}

pub fn opt<'i, P, O>(p: P) -> impl Fn(&'i str) -> ParseResult<(Option<O>, &'i str)>
where
    P: Fn(&'i str) -> ParseResult<(O, &'i str)>,
{
    move |input| match p(input) {
        Ok((out, input)) => Ok((Some(out), input)),
        Err(_) => Ok((None, input)),
    }
}

pub fn seq<'i, P1, P2, O1, O2>(
    p1: P1,
    p2: P2,
) -> impl Fn(&'i str) -> ParseResult<((O1, O2), &'i str)>
where
    P1: Fn(&'i str) -> ParseResult<(O1, &'i str)>,
    P2: Fn(&'i str) -> ParseResult<(O2, &'i str)>,
{
    move |input1| {
        let (out1, input2) = p1(input1)?;
        let (out2, rest) = p2(input2)?;
        Ok(((out1, out2), rest))
    }
}

pub fn alt<'i, P1, P2, O>(p1: P1, p2: P2) -> impl Fn(&'i str) -> ParseResult<(O, &'i str)>
where
    P1: Fn(&'i str) -> ParseResult<(O, &'i str)>,
    P2: Fn(&'i str) -> ParseResult<(O, &'i str)>,
{
    move |input| match p1(input) {
        Ok(res) => Ok(res),
        Err(_) => p2(input),
    }
}

pub fn lit(literal: &'static str) -> impl Fn(&str) -> ParseResult<(&'static str, &str)> {
    move |input| match input.strip_prefix(literal) {
        Some(rest) => Ok((literal, rest)),
        None => Err(format!("unmatched prefix: {}", literal)),
    }
}

pub fn until<F>(is_end_char: F) -> impl Fn(&str) -> ParseResult<(&str, &str)>
where
    F: Fn(char) -> bool,
{
    move |input| match input.chars().position(|c| is_end_char(c)) {
        Some(i) => Ok((&input[..i], &input[i..])),
        None => Ok((input, "")),
    }
}

pub fn operand_repr<F>(is_end_char: F) -> impl Fn(&str) -> ParseResult<(OperandRepr, &str)>
where
    F: Fn(char) -> bool,
{
    move |input| {
        let until_sep = until(|c| is_end_char(c));
        let (maybe_repr, rest) = until_sep(input)?;
        OperandRepr::from_str(maybe_repr).map(|repr| (repr, rest))
    }
}

pub fn tok<'i, P, O, F>(p: P, is_whitespace: F) -> impl Fn(&'i str) -> ParseResult<(O, &'i str)>
where
    P: Fn(&'i str) -> ParseResult<(O, &'i str)>,
    F: Fn(char) -> bool,
{
    move |input| {
        let (out, rest) = p(input)?;
        let consume_whitespace = until(|c| !is_whitespace(c));
        let (_, rest) = consume_whitespace(rest)?;
        Ok((out, rest))
    }
}

pub fn map<'i, P, F, O1, O2>(p: P, map_fn: F) -> impl Fn(&'i str) -> ParseResult<(O2, &'i str)>
where
    P: Fn(&'i str) -> ParseResult<(O1, &'i str)>,
    F: Fn(O1) -> O2,
{
    move |input| p(input).map(|(result, rest)| (map_fn(result), rest))
}

pub fn repeat_until<'i, P1, P2, O1, O2>(
    p1: P1,
    p2: P2,
) -> impl Fn(&'i str) -> ParseResult<((Vec<O1>, Option<O2>), &'i str)>
where
    P1: Fn(&'i str) -> ParseResult<(O1, &'i str)>,
    P2: Fn(&'i str) -> ParseResult<(O2, &'i str)>,
{
    move |mut input| {
        let mut results = vec![];
        loop {
            if let Ok((out2, rest)) = p2(input) {
                return Ok(((results, Some(out2)), rest));
            }

            match p1(input) {
                Ok((out, rest)) => {
                    input = rest;
                    results.push(out);
                }
                _ => return Ok(((results, None), input)),
            }
        }
    }
}

pub fn repeat<'i, P, O>(p: P) -> impl Fn(&'i str) -> ParseResult<(Vec<O>, &'i str)>
where
    P: Fn(&'i str) -> ParseResult<(O, &'i str)>,
{
    move |mut input| {
        let mut results = vec![];
        loop {
            match p(input) {
                Ok((out, rest)) => {
                    input = rest;
                    results.push(out);
                }
                _ => return Ok((results, input)),
            }
        }
    }
}
