use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(NonZeroU32);

impl ObjectId {
    pub(crate) fn new(id: NonZeroU32) -> Self {
        Self(id)
    }
}
