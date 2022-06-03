#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterQuality {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Default for FilterQuality {
    fn default() -> Self {
        Self::None
    }
}
