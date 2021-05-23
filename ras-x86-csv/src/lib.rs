use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MODRM_REG_RE: Regex = Regex::new("ModRM:reg").unwrap();
    pub static ref MODRM_RM_RE: Regex = Regex::new("ModRM:r/?m").unwrap();
    pub static ref ALL_ACC_RE: Regex = Regex::new("AL/AX/EAX/RAX").unwrap();
    pub static ref ACC_OVER_16_RE: Regex = Regex::new("AX/EAX/RAX").unwrap();
    pub static ref IMM_RE: Regex = Regex::new("imm8/16/32").unwrap();
    pub static ref IMM8_RE: Regex = Regex::new("imm8").unwrap();
    pub static ref IW_RE: Regex = Regex::new("iw").unwrap();
    pub static ref MOFFS_RE: Regex = Regex::new("Moffs").unwrap();
    pub static ref OPCODE_RD_RE: Regex = Regex::new("opcode \\+ ?rd").unwrap();
    pub static ref OPCODE_EXT_RE: Regex = Regex::new(r"/(\d)").unwrap();
}

#[macro_export]
macro_rules! get_header {
    ($rec:expr, $hdr:ident) => {
        $rec.get($crate::CsvHeader::$hdr as usize).unwrap()
    };
}

#[allow(unused)]
#[repr(u8)]
pub enum CsvHeader {
    Instruction,
    Opcode,
    Valid64,
    Valid32,
    Valid16,
    FeatureFlags,
    Operand1,
    Operand2,
    Operand3,
    Operand4,
    TupleType,
    Description,
}

pub fn parse_mnemonic(mnemonic: &str) -> (String, u32, u32, u32, u32) {
    let mnemonic_idx = mnemonic.find(" ").unwrap_or(mnemonic.len());
    let (mnemonic, operands) = mnemonic.split_at(mnemonic_idx);

    let mut operands = operands.split(",").filter_map(|s| {
        let s = s.trim();

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    });

    let operand1 = operands.next().map(operand_size).unwrap_or_default();
    let operand2 = operands.next().map(operand_size).unwrap_or_default();
    let operand3 = operands.next().map(operand_size).unwrap_or_default();
    let operand4 = operands.next().map(operand_size).unwrap_or_default();

    // There can't be more than 4 operands
    assert!(operands.next().is_none());

    (mnemonic.to_string(), operand1, operand2, operand3, operand4)
}

// XXX implement me
pub fn operand_size(op: &str) -> u32 {
    if op == "AL/AX/EAX/RAX" {
        return 64;
    } else if op.ends_with("64") || op == "RAX" {
        return 64;
    } else if op.ends_with("32") || op == "EAX" {
        return 32;
    } else if op.ends_with("16") || op == "AX" {
        return 16;
    }

    8
}

/// Return (opcode, opcode_extension, REX prefix).
pub fn parse_instr(instr: &str) -> (u8, Option<u8>, Option<&str>) {
    let mut instr = instr.split(" ");
    let mut opcode = 0;
    let mut opcode_ext = None;
    let mut rex_prefix = None;

    loop {
        match instr.next() {
            Some(op) if u8::from_str_radix(op, 16).is_ok() => {
                opcode = u8::from_str_radix(op, 16).unwrap();

                if let Some(maybe_opcode_ext) = instr.next() {
                    if let Some(caps) = OPCODE_EXT_RE.captures(maybe_opcode_ext) {
                        opcode_ext = caps
                            .get(1)
                            .map(|op| u8::from_str_radix(op.as_str(), 16).unwrap());
                    }
                }

                continue;
            }
            Some(prefix) if prefix.starts_with("REX") => {
                rex_prefix = Some(prefix);

                continue;
            }
            Some("+") => continue,
            None => break,
            _ => continue, // XXX
        }
    }

    (opcode, opcode_ext, rex_prefix)
}
