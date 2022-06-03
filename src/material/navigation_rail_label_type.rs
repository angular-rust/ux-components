pub enum NavigationRailLabelType {
    None = 0,
    Selected = 1,
    All = 2,
}

impl Default for NavigationRailLabelType {
    fn default() -> Self {
        Self::None
    }
}
