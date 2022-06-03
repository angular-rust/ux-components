#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    Rtl = 0,
    Ltr = 1,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}
