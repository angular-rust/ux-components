#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextOverflow {
    Clip = 0,
    Fade = 1,
    Ellipsis = 2,
    Visible = 3,
}

impl Default for TextOverflow {
    fn default() -> Self {
        Self::Clip
    }
}
