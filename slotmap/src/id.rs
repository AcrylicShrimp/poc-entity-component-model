use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlotMapId {
    index: u32,
    generation: NonZeroU32,
}

impl SlotMapId {
    pub(crate) fn new(index: u32, generation: NonZeroU32) -> Self {
        Self { index, generation }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> NonZeroU32 {
        self.generation
    }
}
