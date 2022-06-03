#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisDirection {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Default for AxisDirection {
    fn default() -> Self {
        Self::Up
    }
}
