#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyEventType {
    Down = 0,
    Up = 1,
    Repeat = 2,
}

impl Default for KeyEventType {
    fn default() -> Self {
        Self::Down
    }
}