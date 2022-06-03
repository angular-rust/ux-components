#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonTextTheme {
    Normal = 0,
    Accent = 1,
    Primary = 2,
}

impl Default for ButtonTextTheme {
    fn default() -> Self {
        Self::Normal
    }
}
