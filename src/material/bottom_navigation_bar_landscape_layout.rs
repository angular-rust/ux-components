pub enum BottomNavigationBarLandscapeLayout {
    Spread = 0,
    Centered = 1,
    Linear = 2,
}

impl Default for BottomNavigationBarLandscapeLayout {
    fn default() -> Self {
        Self::Spread
    }
}
