#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaintingStyle {
    Fill = 0,
    Stroke = 1,
}

impl Default for PaintingStyle {
    fn default() -> Self {
        Self::Fill
    }
}