use crate::encoder::Encoder;
use crate::instruction::Instruction;
use crate::{Mode, RasError, RasResult};

use faerie::{ArtifactBuilder, Decl, Link, Reloc, SectionKind};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use target_lexicon::triple;

pub type LabelId = usize;
pub(crate) type InstructionPointer = usize;

pub struct Assembler {
    mode: Mode,
    encoder: Encoder,
    items: Vec<Item>,
    labels: HashMap<LabelId, InstructionPointer>,
}

impl Assembler {
    pub fn new_long(items: Vec<impl Into<Item>>) -> Self {
        let mode = Mode::Long;

        Self {
            mode,
            encoder: Encoder::new(mode),
            items: items.into_iter().map(Into::into).collect(),
            labels: Default::default(),
        }
    }

    pub fn assemble(&mut self) -> RasResult<()> {
        assert_eq!(self.mode, Mode::Long);

        // XXX run a second pass an resolve labels
        for item in &self.items {
            if let Item::Label(label) = item {
                let entry = self.labels.entry(*label);
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
        let mut obj = ArtifactBuilder::new(triple!("x86_64-unknown-unknown-unknown-elf"))
            .name(file.as_ref().into())
            .finish();
        obj.declare_with(
            ".text",
            Decl::section(SectionKind::Text),
            self.encoder.out.clone(),
        )?;

        let file = File::create(file.as_ref())?;
        obj.write(file)?;
        Ok(())
    }
}

pub enum Item {
    Label(usize),
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
