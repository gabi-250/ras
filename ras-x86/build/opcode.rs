use ras_x86_repr::{EncodingBytecode, InstructionEncoding, RexPrefix};
use std::str::FromStr;

pub fn parse_opcode_column(inst: &str) -> InstructionEncoding {
    let inst = inst.as_bytes();
    let mut bytecode = vec![];
    let (is_np, inst) = parse_np(inst);
    let inst = parse_mandatory_prefix(inst, &mut bytecode);
    let inst = parse_rex_prefix(inst, &mut bytecode);
    let inst = parse_opcode(inst, &mut bytecode);
    let _ = parse_modrm(inst, &mut bytecode);

    // TODO parse the remaining parts of the column

    InstructionEncoding::new(bytecode, is_np)
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

fn parse_mandatory_prefix<'a>(inst: &'a [u8], bytecode: &mut Vec<EncodingBytecode>) -> &'a [u8] {
    let i = if let Some(i) = inst.iter().position(|c| is_separator(*c)) {
        i
    } else {
        return inst;
    };

    let (prefix, inst) = match &inst[..i] {
        b"66" => (Some(0x66), skip_separators(&inst[i..], &is_separator)),
        b"F2" => (Some(0xF2), skip_separators(&inst[i..], &is_separator)),
        b"F3" => (Some(0xF3), skip_separators(&inst[i..], &is_separator)),
        _ => (None, inst),
    };

    if let Some(prefix) = prefix {
        bytecode.push(EncodingBytecode::Prefix(prefix));
    }

    inst
}

fn parse_rex_prefix<'a>(inst: &'a [u8], bytecode: &mut Vec<EncodingBytecode>) -> &'a [u8] {
    let i = if let Some(i) = inst.iter().position(|c| is_separator(*c)) {
        i
    } else {
        return inst;
    };

    match RexPrefix::from_str(std::str::from_utf8(&inst[..i]).unwrap()) {
        Ok(prefix) => {
            bytecode.push(EncodingBytecode::Rex(prefix));
            skip_separators(&inst[i..], &is_separator)
        }
        _ => inst,
    }
}

fn parse_opcode<'a>(mut inst: &'a [u8], bytecode: &mut Vec<EncodingBytecode>) -> &'a [u8] {
    loop {
        if inst.len() < 2 {
            break;
        }

        let maybe_op = std::str::from_utf8(&inst[0..2]).unwrap();
        // Not an opcode after all...
        if let Ok(op) = EncodingBytecode::from_str(&maybe_op) {
            bytecode.push(op);
            inst = skip_separators(&inst[2..], &is_separator);
            break;
        }

        let op = match u8::from_str_radix(maybe_op, 16) {
            Ok(op) => {
                // skip over the separator
                inst = skip_separators(&inst[2..], &is_separator);
                op
            }
            Err(_) => break,
        };

        if inst.len() >= 2 {
            match &inst[..2] {
                b"rb" => {
                    inst = &inst[2..];
                    bytecode.push(EncodingBytecode::OpcodeRb(op));
                }
                b"rw" => {
                    inst = &inst[2..];
                    bytecode.push(EncodingBytecode::OpcodeRw(op));
                }
                b"rd" => {
                    inst = &inst[2..];
                    bytecode.push(EncodingBytecode::OpcodeRd(op));
                }
                b"ro" => {
                    inst = &inst[2..];
                    bytecode.push(EncodingBytecode::OpcodeRo(op));
                }
                _ => bytecode.push(EncodingBytecode::Opcode(op)),
            }
        } else {
            bytecode.push(EncodingBytecode::Opcode(op));
        }
    }
    inst
}

fn parse_modrm<'a>(mut inst: &'a [u8], bytecode: &mut Vec<EncodingBytecode>) -> &'a [u8] {
    if inst.len() >= 2 && inst[0] == b'/' {
        if inst[1] >= b'0' && inst[1] <= b'9' {
            // The `/digit` case: the digit is the opcode extension encoded in the reg field of
            // the ModR/M byte.
            let ext = inst[1] - b'0';
            bytecode.push(EncodingBytecode::ModRmWithReg(ext));
            inst = &inst[2..];
        } else if inst[1] == b'r' {
            // The `/r` case: the ModR/M byte of the instruction contains a register operand and
            // an r/m operand
            bytecode.push(EncodingBytecode::ModRm);
            inst = &inst[2..];
        }
    }

    if inst.len() > 2 {
        inst = skip_separators(&inst, &is_separator);

        if inst.len() >= 2 {
            let maybe_op = std::str::from_utf8(&inst[0..2]).unwrap();
            if let Ok(op) = EncodingBytecode::from_str(&maybe_op) {
                bytecode.push(op);
                inst = skip_separators(&inst[2..], &is_separator);
            }
        }
    }

    inst
}
