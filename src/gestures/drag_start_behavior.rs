pub enum DragStartBehavior {
    Down = 0,
    Start = 1,
}

impl Default for DragStartBehavior {
    fn default() -> Self {
        Self::Down
    }
}
