#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceholderAlignment {
    Baseline = 0,
    AboveBaseline = 1,
    BelowBaseline = 2,
    Top = 3,
    Bottom = 4,
    Middle = 5,
}

impl Default for PlaceholderAlignment {
    fn default() -> Self {
        Self::Baseline
    }
}