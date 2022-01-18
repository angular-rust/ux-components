pub enum CrossAxisAlignment {
    Start = 0,
    End = 1,
    Center = 2,
    Stretch = 3,
    Baseline = 4,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}
