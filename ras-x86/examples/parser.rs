use ras_x86::assembler::Assembler;
use ras_x86::parser::parse_asm;
use ras_x86::RasResult;

use std::env;
use std::fs::{self, File};

fn main() -> RasResult<()> {
    let mut args = env::args();
    let (out_file, src_file) = if args.len() < 3 {
        eprintln!(
            "Usage: {} <obj file name> <asm source file name>",
            args.next().unwrap()
        );
        std::process::exit(1);
    } else {
        let mut args = args.skip(1);
        let out_file = args.next().unwrap();
        let asm_file = args.next().unwrap();
        (out_file, asm_file)
    };

    let asm_src = fs::read_to_string(src_file).unwrap();
    match parse_asm(&asm_src) {
        Ok(asm_src) => {
            Assembler::new_long()
                .items(asm_src)
                .write_obj(File::create(out_file)?)?;
        }
        Err(errors) => println!("{}", errors),
    }

    Ok(())
}
