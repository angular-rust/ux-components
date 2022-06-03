#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrokeJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}

impl Default for StrokeJoin {
    fn default() -> Self {
        Self::Miter
    }
}