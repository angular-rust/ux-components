pub enum ShowValueIndicator {
    OnlyForDiscrete = 0,
    OnlyForContinuous = 1,
    Always = 2,
    Never = 3,
}

impl Default for ShowValueIndicator {
    fn default() -> Self {
        Self::OnlyForDiscrete
    }
}
