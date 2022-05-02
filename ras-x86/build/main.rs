mod csv_util;
mod instruction;
mod opcode;
mod parsers;

use csv_util::CsvHeader;
use instruction::parse_instruction_column;
use opcode::parse_opcode_column;
use parsers::ParseResult;

use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use proc_macro2::TokenStream;
use quote::quote;
use ras_x86_repr::{InstructionRepr, Mode};

const INST_CSV: &str = "./x86-csv/x86.csv";
const INST_MAP: &str = "bin/map";
const RUSTFMT_BIN: &str = "rustfmt";

fn main() -> ParseResult<()> {
    let inst_csv = Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_CSV);
    println!("cargo:rerun-if-changed={}", inst_csv.to_str().unwrap());

    let mut rdr = csv::Reader::from_reader(File::open(&inst_csv).unwrap());
    let mut mnemonics = HashSet::new();
    let mut insts: HashMap<String, Vec<InstructionRepr>> = Default::default();

    for rec in rdr.records() {
        let rec = rec.unwrap();

        // XXX: forget about special instructions for now
        if matches!(rec.get(CsvHeader::FeatureFlags as usize), Some(flags) if !flags.is_empty()) {
            continue;
        }

        let (mnemonic, operands) = parse_instruction_column(get_header!(rec, Instruction))?;
        let inst_enc = parse_opcode_column(get_header!(rec, Opcode))?;

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

        let inst_repr = InstructionRepr::new(inst_enc, operands, modes);
        insts.entry(mnemonic.clone()).or_default().push(inst_repr);

        mnemonics.insert(mnemonic);
    }

    let mut insts = insts.into_iter().collect::<Vec<(_, _)>>();
    insts.sort_by_key(|inst| inst.0.clone());

    let inst_map = Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_MAP);
    fs::write(inst_map, bincode::serialize(&insts).unwrap()).unwrap();

    generate_mnemonic_enum(mnemonics);

    Ok(())
}

fn generate_mnemonic_enum(mnemonics: HashSet<String>) {
    let mut mnemonics = mnemonics.into_iter().collect::<Vec<_>>();
    mnemonics.sort();

    let variants = mnemonics
        .iter()
        .map(|mnemonic| mnemonic.parse::<TokenStream>().unwrap())
        .collect::<Vec<TokenStream>>();

    let content = quote! {
        //! This file was autogenerated by build.rs.
        use serde::{Serialize, Deserialize};
        use std::hash::Hash;
        use std::str::FromStr;

        use crate::error::{ParseError, ParseErrorKind};

        #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
        pub enum Mnemonic {
            #(#variants),*
        }

        impl FromStr for Mnemonic {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let upper = s.to_ascii_uppercase();
                match upper.as_str() {
                    #( #mnemonics => Ok(Mnemonic::#variants), )*
                    _ => Err(ParseError::new(ParseErrorKind::InvalidMnemonic(s.to_string())))
                }
            }
        }
    };

    let mnemonic_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/mnemonic.rs");
    fs::write(&mnemonic_file, content.to_string()).unwrap();
    Command::new(RUSTFMT_BIN)
        .arg(mnemonic_file)
        .spawn()
        .expect("failed to run rustfmt");
}

pub fn is_valid_mode(mode_rec: &str) -> bool {
    mode_rec == "Valid"
}
