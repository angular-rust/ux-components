#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAffinity {
    Upstream = 0,
    Downstream = 1,
}

impl Default for TextAffinity {
    fn default() -> Self {
        Self::Upstream
    }
}