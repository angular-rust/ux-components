#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDecorationStyle {
    Solid = 0,
    Double = 1,
    Dotted = 2,
    Dashed = 3,
    Wavy = 4,
}

impl Default for TextDecorationStyle {
    fn default() -> Self {
        Self::Solid
    }
}
