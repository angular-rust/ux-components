#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextBaseline {
    Alphabetic = 0,
    Ideographic = 1,
}

impl Default for TextBaseline {
    fn default() -> Self {
        Self::Alphabetic
    }
}
