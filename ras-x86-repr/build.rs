use proc_macro2::TokenStream;
use quote::quote;
use ras_x86_csv::*;
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::Path;

const INST_CSV: &str = "../ras-x86-csv/x86-csv/x86.csv";

fn main() {
    let inst_csv = Path::new(env!("CARGO_MANIFEST_DIR")).join(INST_CSV);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", inst_csv.to_str().unwrap());
    generate_instruction_repr(inst_csv);
}

fn generate_instruction_repr(inst_csv: impl AsRef<Path>) {
    let mut rdr = csv::Reader::from_reader(File::open(inst_csv).unwrap());
    let mut mnemonics = HashSet::new();

    for rec in rdr.records() {
        let rec = rec.unwrap();

        // XXX: forget about special instructions for now
        if matches!(rec.get(CsvHeader::FeatureFlags as usize), Some(flags) if !flags.is_empty()) {
            continue;
        }

        let (mnemonic, ..) = parse_mnemonic(rec.get(CsvHeader::Instruction as usize).unwrap());
        mnemonics.insert(mnemonic);
    }

    let mnemonics = mnemonics.into_iter().collect::<Vec<_>>();
    let variants = mnemonics
        .iter()
        .map(|mnemonic| mnemonic.parse::<TokenStream>().unwrap())
        .collect::<Vec<TokenStream>>();

    let content = quote! {
        //! This file was autogenerated by build.rs.
        use std::hash::Hash;
        use serde::{Serialize, Deserialize};
        use std::str::FromStr;

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub enum Mnemonic {
            #(#variants),*
        }

        impl FromStr for Mnemonic {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #mnemonics => Ok(Mnemonic::#variants), )*
                    s => Err(format!("unknown mnemonic: {}", s))
                }
            }
        }
    };

    let mnemonic_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/mnemonic.rs");
    fs::write(mnemonic_file, content.to_string()).unwrap();
}
