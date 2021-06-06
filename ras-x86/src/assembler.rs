use crate::context::Label;
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

pub use crate::context::{InstructionPointer, LabelId};

pub struct Assembler {
    mode: Mode,
    encoder: Encoder,
    items: Vec<Item>,
    labels: HashMap<LabelId, Label>,
    label_ptrs: HashMap<LabelId, InstructionPointer>,
}

impl Assembler {
    pub fn new_long(items: Vec<impl Into<Item>>, labels: HashMap<LabelId, Label>) -> Self {
        let mode = Mode::Long;

        Self {
            mode,
            encoder: Encoder::new(mode),
            items: items.into_iter().map(Into::into).collect(),
            labels,
            label_ptrs: Default::default(),
        }
    }

    pub fn assemble(&mut self) -> RasResult<()> {
        assert_eq!(self.mode, Mode::Long);

        for item in &self.items {
            if let Item::Label(label) = item {
                let entry = self.label_ptrs.entry(*label);
                if matches!(entry, Entry::Occupied(_)) {
                    return Err(RasError::DuplicateLabel(*label));
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

        for (label_id, index) in &self.label_ptrs {
            let label = self
                .labels
                .get(label_id)
                .ok_or(RasError::UnknownLabel(*label_id))?;
            let sym = Symbol {
                name: label.name().as_bytes().to_vec(),
                value: *index as u64,
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
    Label(LabelId),
    Instruction(Instruction),
}

impl From<Instruction> for Item {
    fn from(inst: Instruction) -> Item {
        Item::Instruction(inst)
    }
}

impl From<LabelId> for Item {
    fn from(label: LabelId) -> Item {
        Item::Label(label)
    }
}
