pub enum StretchMode {
    // The background widget will expand to fill the extra space.
    ZoomBackground = 0,

    // The background will blur using a ImageFilter.blur effect.
    BlurBackground = 1,

    // The title will fade away as the user over-scrolls.
    FadeTitle = 2,
}

impl Default for StretchMode {
    fn default() -> Self {
        Self::ZoomBackground
    }
}