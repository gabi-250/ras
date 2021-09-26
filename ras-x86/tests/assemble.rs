use std::env;
use std::fs;
use std::process::{Command, Stdio};

use goblin::elf::Elf;
use ras_x86::assembler::Assembler;
use ras_x86::parser::parse_asm;

const TEST_CASES: &str = "tests/asm";
const RAS_TEST_OBJ: &str = "/tmp/ras-test.o";

#[test]
fn compare_text_section_with_gas() {
    let test_dir = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), TEST_CASES);
    for path in fs::read_dir(test_dir).unwrap() {
        let path = path.unwrap().path();
        let test_file = path.file_name().unwrap().to_str().unwrap();
        if test_file.starts_with(".") {
            continue;
        }
        let asm_src = fs::read_to_string(&path).unwrap();
        let mut out = vec![];
        let mut asm = Assembler::new_long(parse_asm(&asm_src).unwrap(), &[]);
        asm.assemble().unwrap();
        asm.write_obj(&mut out).unwrap();
        let status = Command::new("as")
            .args(["-o", RAS_TEST_OBJ, path.to_str().unwrap()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect(&format!(
                "failed to assemble {} with as",
                path.to_str().unwrap()
            ));
        if !status.success() {
            panic!("failed to assemble {} with as", path.to_str().unwrap());
        }
        let expected = fs::read(RAS_TEST_OBJ).unwrap();
        let expected_text = read_text_section(&expected);
        let actual_text = read_text_section(&out);
        assert_eq!(
            actual_text, expected_text,
            "incorrect .text section for \"{}\"",
            test_file
        );
    }
}

fn read_text_section(input: &[u8]) -> &[u8] {
    let elf = Elf::parse(input).expect("failed to parse ELF file");
    let text_section = elf
        .section_headers
        .iter()
        .find(|section_hdr| {
            elf.shdr_strtab
                .get_at(section_hdr.sh_name)
                .unwrap_or_default()
                == ".text"
        })
        .expect("object file does not have a .text section");
    &input[text_section.file_range().unwrap()]
}
