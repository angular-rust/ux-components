#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathOperation {
    Difference = 0,
    Intersect = 1,
    ReverseDifference = 4,
    Union = 2,
    Xor = 3,
}

impl Default for PathOperation {
    fn default() -> Self {
        Self::Difference
    }
}