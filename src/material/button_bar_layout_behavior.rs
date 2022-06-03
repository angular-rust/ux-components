#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonBarLayoutBehavior {
    Constrained = 0,
    Padded = 1,
}

impl Default for ButtonBarLayoutBehavior {
    fn default() -> Self {
        Self::Constrained
    }
}
