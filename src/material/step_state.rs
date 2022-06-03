pub enum StepState {
    Indexed = 0,
    Editing = 1,
    Complete = 2,
    Disabled = 3,
    Error = 4,
}

impl Default for StepState {
    fn default() -> Self {
        Self::Indexed
    }
}
