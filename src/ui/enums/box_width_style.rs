#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxWidthStyle {
    Tight = 0,
    Max = 1,
}

impl Default for BoxWidthStyle {
    fn default() -> Self {
        Self::Tight
    }
}
