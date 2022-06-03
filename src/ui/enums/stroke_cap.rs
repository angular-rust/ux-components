#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrokeCap {
    Butt = 0,
    Round = 1,
    Square = 2,
}

impl Default for StrokeCap {
    fn default() -> Self {
        Self::Butt
    }
}