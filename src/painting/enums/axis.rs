#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Horizontal = 0,
    Vertical = 1,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Horizontal
    }
}
