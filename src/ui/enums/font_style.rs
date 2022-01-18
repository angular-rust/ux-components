#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}
