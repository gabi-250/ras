use std::ops::Deref;

pub type SymbolId = String;
pub type InstructionPointer = u64;

#[derive(Debug, Clone)]
pub struct Symbol {
    ty: SymbolType,
    is_global: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum SymbolType {
    Byte,
    Word,
    Double,
    Quad,
}

impl Symbol {
    pub fn global(ty: SymbolType) -> Self {
        Self {
            ty,
            is_global: true,
        }
    }

    pub fn local(ty: SymbolType) -> Self {
        Self {
            ty,
            is_global: false,
        }
    }

    pub fn is_global(&self) -> bool {
        self.is_global
    }
}
