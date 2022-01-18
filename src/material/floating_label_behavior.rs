pub enum FloatingLabelBehavior {
    Never = 0,
    Auto = 1,
    Always = 2,
}

impl Default for FloatingLabelBehavior {
    fn default() -> Self {
        Self::Auto
    }
}
