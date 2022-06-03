#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainAxisSize {
    Min = 0,
    Max = 1,
}

impl Default for MainAxisSize {
    fn default() -> Self {
        Self::Min
    }
}
