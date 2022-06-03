#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlurStyle {
    Normal = 0,
    Solid = 1,
    Outer = 2,
    Inner = 3,
}

impl Default for BlurStyle {
    fn default() -> Self {
        Self::Normal
    }
}