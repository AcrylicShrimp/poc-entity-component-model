pub type BitmapType = u64;

pub struct Bitmap {
    bits: u64,
    index: u32,
}

impl Bitmap {
    pub(crate) const BITS_PER_WORD: u32 = u64::BITS;

    pub fn new(index: u32) -> Self {
        Self { index, bits: 0 }
    }

    pub fn is_full(&self) -> bool {
        self.bits == 0
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn mark_as_empty(&mut self, index: u32) {
        debug_assert!(index < Self::BITS_PER_WORD);
        self.bits |= 1 << index;
    }

    pub fn mark_as_filled(&mut self, index: u32) {
        debug_assert!(index < Self::BITS_PER_WORD);
        self.bits &= !(1 << index);
    }

    pub fn find_first_empty_index(&self) -> u32 {
        debug_assert!(!self.is_full());
        self.bits.trailing_zeros()
    }
}
