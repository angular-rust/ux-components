#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathFillType {
    NonZero = 0,
    EvenOdd = 1,
}

impl Default for PathFillType {
    fn default() -> Self {
        Self::NonZero
    }
}