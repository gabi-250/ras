use std::ops::Deref;

pub type LabelId = usize;
pub type InstructionPointer = usize;

#[derive(Debug, Clone)]
pub struct Label {
    id: LabelId,
    name: String,
    is_global: bool,
}

impl Label {
    pub fn global(id: LabelId, name: String) -> Self {
        Self {
            id,
            name,
            is_global: true,
        }
    }

    pub fn local(id: LabelId, name: String) -> Self {
        Self {
            id,
            name,
            is_global: false,
        }
    }

    pub fn is_global(&self) -> bool {
        self.is_global
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Deref for Label {
    type Target = LabelId;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
