#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexFit {
    // The child is forced to fill the available space.
    // The Expanded widget assigns this kind of FlexFit to its child.
    Tight = 0,
    // The child can be at most as large as the available space (but is allowed to be smaller).
    // The Flexible widget assigns this kind of FlexFit to its child.
    Lose = 1,
}

impl Default for FlexFit {
    fn default() -> Self {
        Self::Tight
    }
}
