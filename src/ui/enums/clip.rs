#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clip {
    None = 0,
    HardEdge = 1,
    AntiAlias = 2,
    AntiAliasWithSaveLayer = 3,
}

impl Default for Clip {
    fn default() -> Self {
        Self::None
    }
}
