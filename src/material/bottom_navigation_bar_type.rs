pub enum BottomNavigationBarType {
    Fixed = 0,
    Shifting = 1,
}

impl Default for BottomNavigationBarType {
    fn default() -> Self {
        Self::Fixed
    }
}
