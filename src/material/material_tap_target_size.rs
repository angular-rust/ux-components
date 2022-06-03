#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialTapTargetSize {
    Padded = 0,
    ShrinkWrap = 1,
}

impl Default for MaterialTapTargetSize {
    fn default() -> Self {
        Self::Padded
    }
}
