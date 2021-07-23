use ras_x86_repr::{InstructionEncoding, RexPrefix};
use std::str::FromStr;

pub fn parse_opcode_column(inst: &str) -> InstructionEncoding {
    let inst = inst.as_bytes();
    let (is_np, inst) = parse_np(inst);
    let (mandatory_prefix, inst) = parse_mandatory_prefix(inst);
    let (rex_prefix, inst) = parse_rex_prefix(inst);
    let (opcode, inst) = parse_opcode(inst);
    let (opcode_ext, _) = parse_opcode_extension(inst);

    // TODO parse the remaining parts of the column

    InstructionEncoding::new(
        opcode,
        false, // sib,
        rex_prefix,
        mandatory_prefix,
        opcode_ext,
        is_np,
    )
}

fn is_separator(c: u8) -> bool {
    c == b' ' || c == b'+'
}

fn skip_separators(s: &[u8], is_sep: impl Fn(u8) -> bool) -> &[u8] {
    let mut i = 0;
    while i < s.len() && is_sep(s[i]) {
        i += 1;
    }
    &s[i..]
}

fn parse_np(inst: &[u8]) -> (bool, &[u8]) {
    let i = if let Some(i) = inst.iter().position(|c| is_separator(*c)) {
        i
    } else {
        return (false, inst);
    };

    if &inst[..i] == b"NP" {
        (true, skip_separators(&inst[i..], &is_separator))
    } else {
        (false, inst)
    }
}

fn parse_mandatory_prefix(inst: &[u8]) -> (Option<u8>, &[u8]) {
    let i = if let Some(i) = inst.iter().position(|c| is_separator(*c)) {
        i
    } else {
        return (None, inst);
    };

    match &inst[..i] {
        b"66" => (Some(0x66), skip_separators(&inst[i..], &is_separator)),
        b"F2" => (Some(0xF2), skip_separators(&inst[i..], &is_separator)),
        b"F3" => (Some(0xF3), skip_separators(&inst[i..], &is_separator)),
        _ => (None, inst),
    }
}

fn parse_rex_prefix(inst: &[u8]) -> (Option<RexPrefix>, &[u8]) {
    let i = if let Some(i) = inst.iter().position(|c| is_separator(*c)) {
        i
    } else {
        return (None, inst);
    };

    match RexPrefix::from_str(std::str::from_utf8(&inst[..i]).unwrap()) {
        Ok(prefix) => (Some(prefix), skip_separators(&inst[i..], &is_separator)),
        _ => (None, inst),
    }
}

fn parse_opcode(mut inst: &[u8]) -> (Vec<u8>, &[u8]) {
    const ENTRY_METADATA: &[&str] = &["cb", "cw", "cd", "cp", "co", "ct"];

    let mut opcode = vec![];
    loop {
        if inst.len() < 2 || opcode.len() == 3 {
            break;
        }

        let maybe_op = std::str::from_utf8(&inst[0..2]).unwrap();
        // Not an opcode after all...
        if ENTRY_METADATA.contains(&maybe_op) {
            inst = skip_separators(&inst[2..], &is_separator);
            break;
        }

        match u8::from_str_radix(maybe_op, 16) {
            Ok(op) => {
                opcode.push(op);
                // skip over the separator
                inst = skip_separators(&inst[2..], &is_separator);
            }
            Err(_) => break,
        }
    }
    (opcode, inst)
}

fn parse_opcode_extension(inst: &[u8]) -> (Option<u8>, &[u8]) {
    if inst.len() >= 2 && inst[0] == b'/' {
        if inst[1] >= b'0' && inst[1] <= b'9' {
            let ext = inst[1] - b'0';
            return (Some(ext), &inst[2..]);
        }
    }
    (None, inst)
}
