use ras_x86_csv::*;
use ras_x86_repr::instruction::InstructionRepr;
use ras_x86_repr::mnemonic::Mnemonic;
use ras_x86_repr::operand::{OperandKind, OperandRepr};
use ras_x86_repr::Mode;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::str::FromStr;

const INST_CSV: &str = "../ras-x86-csv/x86-csv/x86.csv";
const INST_MAP: &str = "bin/map";

fn main() {
    let inst_csv = Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_CSV);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", inst_csv.to_str().unwrap());

    let mut rdr = csv::Reader::from_reader(File::open(inst_csv).unwrap());

    let mut instrs: HashMap<Mnemonic, Vec<InstructionRepr>> = Default::default();
    for rec in rdr.records() {
        let rec = rec.unwrap();

        // XXX: forget about special instructions for now
        if matches!(rec.get(CsvHeader::FeatureFlags as usize), Some(flags) if !flags.is_empty()) {
            continue;
        }

        let (opcode, opcode_ext, rex_prefix) = parse_instr(get_header!(rec, Opcode));
        let (mnemonic, size1, size2, size3, size4) = parse_mnemonic(get_header!(rec, Instruction));
        let operand_encoding1 = build_operand_enc(get_header!(rec, Operand1), size1);
        let operand_encoding2 = build_operand_enc(get_header!(rec, Operand2), size2);
        let operand_encoding3 = build_operand_enc(get_header!(rec, Operand3), size3);
        let operand_encoding4 = build_operand_enc(get_header!(rec, Operand4), size4);

        let mut modes = vec![];
        if is_valid_mode(get_header!(rec, Valid16)) {
            modes.push(Mode::Real);
        }

        if is_valid_mode(get_header!(rec, Valid32)) {
            modes.push(Mode::Protected);
        }

        if is_valid_mode(get_header!(rec, Valid64)) {
            modes.push(Mode::Long);
        }

        let mnemonic = Mnemonic::from_str(&mnemonic).unwrap();
        let operands = vec![
            operand_encoding1,
            operand_encoding2,
            operand_encoding3,
            operand_encoding4,
        ]
        .into_iter()
        .filter_map(|op| op)
        .collect();

        let instr = InstructionRepr::new(opcode, false, rex_prefix, opcode_ext, operands, modes);

        instrs.entry(mnemonic).or_default().push(instr);
    }

    let inst_map = Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_MAP);
    fs::write(inst_map, bincode::serialize(&instrs).unwrap()).unwrap();
}

pub fn build_operand_enc(operand: &str, size: u32) -> Option<OperandRepr> {
    if MODRM_REG_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::ModRmReg, size));
    } else if MODRM_RM_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::ModRmRegMem, size));
    } else if ALL_ACC_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::Al, size));
    } else if ACC_OVER_16_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::Al, size));
    } else if IMM_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::Imm, size));
    } else if IMM8_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::Imm, size));
    } else if IW_RE.is_match(operand) || operand == "imm16" {
        return Some(OperandRepr::new(OperandKind::Imm, size));
    } else if OPCODE_RD_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::OpcodeRd, size));
    } else if MOFFS_RE.is_match(operand) {
        return Some(OperandRepr::new(OperandKind::MemoryOffset, size));
    } else if operand == "1" {
        return Some(OperandRepr::new(OperandKind::One, size)); // XXX
    } else if operand == "CL" {
        return Some(OperandRepr::new(OperandKind::Cl, size)); // XXX
    } else if operand == "NA" || operand == "" {
        return None;
    } else {
        unimplemented!("operand mode for {}", operand);
    }
}

pub fn is_valid_mode(mode_rec: &str) -> bool {
    mode_rec == "Valid"
}
