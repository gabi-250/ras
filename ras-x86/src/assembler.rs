use crate::encoder::Encoder;
use crate::instruction::Instruction;
use crate::object::ObjectWriter;
use crate::{Mode, RasError, RasResult};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Write;

pub use crate::symbol::{Symbol, SymbolId, SymbolOffset, SymbolType};

pub struct Assembler {
    /// The instruction encoder.
    encoder: Encoder,
    /// The instructions to encode.
    items: Vec<Item>,
    /// The symbols used in the assembly program.
    sym_tab: HashMap<SymbolId, Symbol>,
}

impl Assembler {
    pub fn long_mode() -> Self {
        Self {
            encoder: Encoder::new(Mode::Long),
            items: Default::default(),
            sym_tab: Default::default(),
        }
    }

    pub fn items<I: Into<Item>>(mut self, items: Vec<I>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    pub fn symbols(mut self, symbols: &[(SymbolId, Symbol)]) -> Self {
        self.sym_tab.extend(symbols.iter().cloned());
        self
    }

    /// Get the assembled instructions.
    pub fn dump_text(mut self) -> RasResult<Vec<u8>> {
        self.assemble()?;
        Ok(self.encoder.out)
    }

    /// Write an object file with the assembled instructions into the specified `writer`.
    pub fn write_obj(mut self, mut writer: impl Write) -> RasResult<()> {
        self.assemble()?;
        let mut obj = ObjectWriter::new(self.encoder.mode);
        // write the assembled instructions in the .text section of the object file
        obj.append_text_section(&self.encoder.out);
        // emit all the symbols
        for (sym_id, sym) in &self.sym_tab {
            obj.add_text_symbol(sym_id, sym);
        }
        // write the object file
        writer.write_all(&obj.write()?)?;
        Ok(())
    }

    fn assemble(&mut self) -> RasResult<()> {
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
                        entry.get_mut().offset = Some(self.encoder.current_offset());
                    }
                    Entry::Vacant(entry) => {
                        let sym = Symbol::new(
                            SymbolType::Quad,
                            self.encoder.current_offset(),
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
