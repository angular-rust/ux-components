#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxHeightStyle {
    Tight = 0,
    Max = 1,
    IncludeLineSpacingMiddle = 2,
    IncludeLineSpacingTop = 3,
    IncludeLineSpacingBottom = 4,
    Strut = 5,
}

impl Default for BoxHeightStyle {
    fn default() -> Self {
        Self::Tight
    }
}
