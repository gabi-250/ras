use crate::encoder::Encoder;
use crate::instruction::Instruction;
use crate::{Mode, RasError, RasResult};

use std::collections::hash_map::Entry;
use std::collections::HashMap;

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
        Self {
            mode: Mode::Long,
            encoder: Default::default(),
            items: items.into_iter().map(Into::into).collect(),
            labels: Default::default(),
        }
    }

    pub fn assemble(mut self) -> RasResult<Vec<u8>> {
        assert_eq!(self.mode, Mode::Long);

        // XXX run a second pass an resolve labels
        for item in self.items {
            match item {
                Item::Instruction(inst) => {
                    inst.encode(&mut self.encoder)?;
                }
                Item::Label(label) => {
                    let entry = self.labels.entry(label);
                    if matches!(entry, Entry::Occupied(_)) {
                        return Err(RasError::DuplicateLabel(label));
                    } else {
                        entry.or_insert(self.encoder.instruction_pointer());
                    }
                }
            }
        }

        Ok(self.encoder.out)
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
