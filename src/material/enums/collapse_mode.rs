pub enum CollapseMode {
    // The background widget will scroll in a parallax fashion.
    Parallax = 0,
    // The background widget pin in place until it reaches the min extent.
    Pin = 1,
    // The background widget will act as normal with no collapsing effect.
    None = 2,
}

impl Default for CollapseMode {
    fn default() -> Self {
        Self::Parallax
    }
}