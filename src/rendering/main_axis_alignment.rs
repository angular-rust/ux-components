pub enum MainAxisAlignment {
    Start = 0,
    End = 1,
    Center = 2,
    SpaceBetween = 3,
    SpaceAround = 4,
    SpaceEvenly = 5,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}
