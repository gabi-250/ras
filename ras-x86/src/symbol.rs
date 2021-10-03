pub type SymbolId = String;
pub type SymbolOffset = u64;

#[derive(Debug, Clone)]
pub struct Symbol {
    #[allow(unused)]
    pub(crate) ty: SymbolType,
    /// The offset of this symbol in the text section.
    pub(crate) offset: Option<SymbolOffset>,
    /// The attributes of the symbol.
    pub(crate) attrs: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum SymbolType {
    Byte,
    Word,
    Double,
    Quad,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum SymbolAttribute {
    Global = 0b01,
    Weak = 0b10,
}

impl Symbol {
    pub fn new_decl(ty: SymbolType, attrs: u8) -> Self {
        Self {
            ty,
            offset: None,
            attrs,
        }
    }

    pub(crate) fn new(ty: SymbolType, offset: SymbolOffset, attrs: u8) -> Self {
        Self {
            ty,
            offset: Some(offset),
            attrs,
        }
    }

    pub fn offset(&self) -> &Option<SymbolOffset> {
        &self.offset
    }

    pub fn is_defined(&self) -> bool {
        self.offset.is_some()
    }

    pub fn is_global(&self) -> bool {
        (self.attrs & SymbolAttribute::Global as u8) == 1
    }

    pub fn is_weak(&self) -> bool {
        (self.attrs & SymbolAttribute::Weak as u8) == 1
    }
}
