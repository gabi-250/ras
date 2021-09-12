use crate::encoder::Encoder;
use crate::instruction::Instruction;
use crate::object::ObjectWriter;
use crate::{Mode, RasError, RasResult};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Write;

pub use crate::symbol::{InstructionPointer, Symbol, SymbolId, SymbolType};

pub struct Assembler {
    mode: Mode,
    encoder: Encoder,
    items: Vec<Item>,
    sym_tab: HashMap<SymbolId, Symbol>,
}

impl Assembler {
    pub fn new_long<I: Into<Item>>(items: Vec<I>, symbols: &[(SymbolId, Symbol)]) -> Self {
        let mode = Mode::Long;

        Self {
            mode,
            encoder: Encoder::new(mode),
            items: items.into_iter().map(Into::into).collect(),
            sym_tab: symbols.iter().cloned().collect(),
        }
    }

    pub fn assemble(&mut self) -> RasResult<()> {
        assert_eq!(self.mode, Mode::Long);

        for item in &self.items {
            match item {
                Item::Instruction(inst) => {
                    inst.encode(&mut self.encoder)?;
                }
                Item::Label(label) => match self.sym_tab.entry(label.to_string()) {
                    Entry::Occupied(entry) if entry.get().is_defined() => {
                        return Err(RasError::DuplicateLabel(label.to_string()));
                    }
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().offset = Some(self.encoder.instruction_pointer());
                    }
                    Entry::Vacant(entry) => {
                        let sym = Symbol::new(
                            SymbolType::Quad,
                            self.encoder.instruction_pointer(),
                            Default::default(),
                        );
                        entry.insert(sym);
                    }
                },
            }
        }

        self.encoder.fixup_symbol_references(&self.sym_tab)?;

        Ok(())
    }

    pub fn dump_out(&self) -> &[u8] {
        &self.encoder.out
    }

    pub fn write_obj(&self, mut writer: impl Write) -> RasResult<()> {
        let mut obj = ObjectWriter::new(self.mode);

        obj.append_text_section(&self.encoder.out);

        for (sym_id, sym) in &self.sym_tab {
            obj.add_text_symbol(sym_id, sym);
        }

        writer.write_all(&obj.write()?)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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
