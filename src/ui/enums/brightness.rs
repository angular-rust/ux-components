#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Brightness {
    Dark = 0,
    Light = 1,
}

impl Default for Brightness {
    fn default() -> Self {
        Self::Dark
    }
}
