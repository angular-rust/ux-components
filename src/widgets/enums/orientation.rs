#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    // Taller than wide.
    Portrait = 0,
    // Wider than tall.
    Landscape = 1,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Portrait
    }
}
