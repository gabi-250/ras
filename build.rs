use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const INSTR_CSV: &str = "x86-csv/x86.csv";

#[repr(u8)]
enum CsvHeader {
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

fn main() {
    generate_instruction_repr(format!(
        "{}/src/x86/instruction_repr.rs",
        env!["CARGO_MANIFEST_DIR"]
    ));
}

fn generate_instruction_repr(path: impl AsRef<Path>) {
    let mut rdr = csv::Reader::from_reader(
        File::open(Path::new(env!("CARGO_MANIFEST_DIR")).join(INSTR_CSV)).unwrap(),
    );
    let mut mnemonics = HashSet::new();
    let insert_instrs: Vec<String> = vec![];
    let mut build_instrs = TokenStream::new();

    for rec in rdr.records() {
        let rec = rec.unwrap();
        let (mnemonic, operand_mode) =
            parse_mnemonic(rec.get(CsvHeader::Instruction as usize).unwrap());

        let (opcode, rex_prefix) = parse_instr(rec.get(CsvHeader::Opcode as usize).unwrap());

        {
            let mnemonic = mnemonic.parse::<TokenStream>().unwrap();
            let modes_tok = quote! { vec![] };
            let rex_prefix_tok = match rex_prefix {
                None => quote! { None },
                Some(s) => quote! { Some(std::str::FromStr::from_str(#s).unwrap()) },
            };

            build_instrs.extend(quote! {
                instrs.insert(
                    (Mnemonic::#mnemonic, #operand_mode),
                    InstructionRepr {
                        opcode: #opcode,
                        modrm: true,
                        sib: false,
                        rex_prefix: #rex_prefix_tok,
                        modes: vec![] // XXX
                    }
                );
            });
        }

        mnemonics.insert(mnemonic);
    }

    let mut variants = TokenStream::new();
    for mnemonic in mnemonics {
        let mnemonic = mnemonic.parse::<TokenStream>().unwrap();
        variants.extend(quote! {
            #mnemonic,
        });
    }

    let content = quote! {
        use std::hash::Hash;

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum Mnemonic {
            #variants
        }

    };

    let path = format!("{}/src/x86/mnemonic.rs", env!["CARGO_MANIFEST_DIR"]);
    let mut mnemonic_file = File::create(path).unwrap();
    write!(mnemonic_file, "{}", content).unwrap();

    let content = quote! {
        use crate::x86::Mnemonic;
        use crate::x86::instruction::OperandMode;
        use crate::x86::instruction_encoding::InstructionRepr;
        use std::collections::HashMap;

        lazy_static::lazy_static! {
            pub(crate) static ref INSTR_REPRS: HashMap<(Mnemonic, OperandMode), InstructionRepr> = {
                let mut instrs = HashMap::new();
                #build_instrs
                instrs
            };
        }
    };

    let path = format!("{}/src/x86/instruction_repr.rs", env!["CARGO_MANIFEST_DIR"]);
    let mut mnemonic_file = File::create(path).unwrap();
    write!(mnemonic_file, "{}", content).unwrap();
}

fn quote_option<T: ToTokens>(opt: Option<T>) -> TokenStream {
    match opt {
        Some(s) => quote! { Some(#s) },
        None => quote! { None },
    }
}

fn parse_mnemonic(mnemonic: &str) -> (String, TokenStream) {
    let mnemonic_idx = mnemonic.find(" ").unwrap_or(mnemonic.len());
    let (mnemonic, operands) = mnemonic.split_at(mnemonic_idx);

    let mut operands = operands.split(",").map(|s| s.trim());

    let operand1 = quote_option(operands.next());
    let operand2 = quote_option(operands.next());
    let operand3 = quote_option(operands.next());
    let operand4 = quote_option(operands.next());

    // There can't be more than 4 operands
    assert!(operands.next().is_none());

    let operand_mode = quote! {
        OperandMode::from((#operand1, #operand2, #operand3, #operand4))
    };

    (mnemonic.to_string(), operand_mode)
}

fn parse_instr(instr: &str) -> (u8, Option<&str>) {
    let mut instr = instr.split(" ");
    let mut opcode = 0;
    let mut rex_prefix = None;

    loop {
        match instr.next() {
            Some(op) if u8::from_str_radix(op, 16).is_ok() => {
                opcode = u8::from_str_radix(op, 16).unwrap();

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

    (opcode, rex_prefix)
}

#[allow(unused)]
fn parse_operands(
    operand1: &str,
    operand2: &str,
    operand3: &str,
    operand4: &str,
) -> (TokenStream, bool, bool) {
    unimplemented!()
}
