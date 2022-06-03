#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipOp {
    Difference = 0,
    Intersect = 1,
}

impl Default for ClipOp {
    fn default() -> Self {
        Self::Difference
    }
}