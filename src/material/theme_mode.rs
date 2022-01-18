#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    System = 0,
    Light = 1,
    Dark = 2,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::System
    }
}
