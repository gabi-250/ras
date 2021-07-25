use object::endian::Endianness;
use object::write::{Object, SectionId, StandardSection, Symbol as ObjSymbol, SymbolSection};
use object::{Architecture, BinaryFormat, SymbolFlags, SymbolKind, SymbolScope};

use crate::symbol::Symbol;
use crate::{Mode, RasResult};

pub struct ObjectWriter {
    obj: Object,
    text_section_id: SectionId,
}

impl ObjectWriter {
    pub fn new(mode: Mode) -> Self {
        let mut obj = Object::new(BinaryFormat::Elf, Self::arch(mode), Endianness::Little);
        let text_section_id = obj.section_id(StandardSection::Text);

        Self {
            obj,
            text_section_id,
        }
    }

    pub fn append_text_section(&mut self, text_section: &[u8]) {
        const ALIGN: u64 = 8;

        self.obj
            .append_section_data(self.text_section_id, &text_section, ALIGN);
    }

    pub fn add_text_symbol(&mut self, sym_id: &str, sym: &Symbol) {
        // If the symbol is defined in this compilation unit...
        if let Some(offset) = sym.offset {
            let sym = ObjSymbol {
                name: sym_id.as_bytes().to_vec(),
                // If the symbol defined in a section, then this is the section offset of the
                // symbol.
                value: offset,
                kind: SymbolKind::Label,
                scope: Self::sym_scope(sym),
                weak: sym.is_weak(),
                section: SymbolSection::Section(self.text_section_id),
                flags: SymbolFlags::None,
                size: 0,
            };
            self.obj.add_symbol(sym);
        }
    }

    pub fn write(&mut self) -> RasResult<Vec<u8>> {
        Ok(self.obj.write()?)
    }

    fn arch(mode: Mode) -> Architecture {
        match mode {
            Mode::Real => Architecture::I386,
            Mode::Protected => Architecture::I386,
            Mode::Long => Architecture::X86_64,
        }
    }

    fn sym_scope(sym: &Symbol) -> SymbolScope {
        if sym.is_global() {
            SymbolScope::Dynamic
        } else {
            SymbolScope::Compilation
        }
    }
}
