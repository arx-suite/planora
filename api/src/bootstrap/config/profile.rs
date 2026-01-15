#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Development,
    Release,
}

impl Profile {
    #[inline]
    pub fn is_release(&self) -> bool {
        matches!(self, Self::Release)
    }

    #[inline]
    pub fn is_development(&self) -> bool {
        matches!(self, Self::Release)
    }
}
