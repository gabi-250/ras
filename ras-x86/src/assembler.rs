use crate::encoder::Encoder;
use crate::instruction::Instruction;
use crate::{Mode, RasError, RasResult};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use object::endian::Endianness;
use object::write::{Object, StandardSection, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, SymbolFlags, SymbolKind, SymbolScope};

pub use crate::symbol::{InstructionPointer, SymbolId};

pub struct Assembler {
    mode: Mode,
    encoder: Encoder,
    items: Vec<Item>,
    sym_tab: HashMap<SymbolId, InstructionPointer>,
}

impl Assembler {
    pub fn new_long(items: Vec<impl Into<Item>>) -> Self {
        let mode = Mode::Long;

        Self {
            mode,
            encoder: Encoder::new(mode),
            items: items.into_iter().map(Into::into).collect(),
            sym_tab: Default::default(),
        }
    }

    pub fn assemble(&mut self) -> RasResult<()> {
        assert_eq!(self.mode, Mode::Long);

        for item in &self.items {
            if let Item::Label(label) = item {
                let entry = self.sym_tab.entry(label.to_string());
                if matches!(entry, Entry::Occupied(_)) {
                    return Err(RasError::DuplicateLabel(label.to_string()));
                } else {
                    entry.or_insert(self.encoder.instruction_pointer());
                }
            }
        }

        for item in &self.items {
            if let Item::Instruction(inst) = item {
                inst.encode(&mut self.encoder)?;
            }
        }

        Ok(())
    }

    pub fn dump_out(&self) -> &[u8] {
        &self.encoder.out
    }

    pub fn write_obj(&self, file: impl AsRef<str>) -> RasResult<()> {
        const ALIGN: u64 = 8;

        let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
        let text_section_id = obj.section_id(StandardSection::Text);
        obj.append_section_data(text_section_id, &self.encoder.out, ALIGN);

        for (sym_id, _sym) in &self.sym_tab {
            let sym = Symbol {
                name: sym_id.as_bytes().to_vec(),
                value: 0, // XXX
                kind: SymbolKind::Label,
                scope: SymbolScope::Dynamic, // XXX
                weak: false,
                section: SymbolSection::Section(text_section_id),
                flags: SymbolFlags::None,
                size: 0,
            };
            obj.add_symbol(sym);
        }

        File::create(file.as_ref())?.write(&obj.write()?)?;
        Ok(())
    }
}

pub enum Item {
    Label(SymbolId),
    Instruction(Instruction),
}

impl From<Instruction> for Item {
    fn from(inst: Instruction) -> Item {
        Item::Instruction(inst)
    }
}

impl From<SymbolId> for Item {
    fn from(label: SymbolId) -> Item {
        Item::Label(label)
    }
}
