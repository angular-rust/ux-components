#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileMode {
    Clamp = 0,
    Decal = 3,
    Mirror = 2,
    Repeated = 1,
}

impl Default for TileMode {
    fn default() -> Self {
        Self::Clamp
    }
}