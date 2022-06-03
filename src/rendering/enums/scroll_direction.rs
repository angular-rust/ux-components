#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScrollDirection {
    // No scrolling is underway.
    Idle = 0,

    // Scrolling is happening in the positive scroll offset direction.
    // For example, for the GrowthDirection.forward part of a vertical AxisDirection.down list, 
    // this means the content is moving up, exposing lower content.
    Forward = 1,
    // Scrolling is happening in the negative scroll offset direction.
    // For example, for the GrowthDirection.forward part of a vertical AxisDirection.down list, 
    // this means the content is moving down, exposing earlier content.
    Reverse = 2
}

impl Default for ScrollDirection {
    fn default() -> Self {
        Self::Idle
    }
}