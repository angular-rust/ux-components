pub enum SnackBarBehavior {
    Fixed = 0,
    Floating = 1,
}

impl Default for SnackBarBehavior {
    fn default() -> Self {
        Self::Fixed
    }
}
