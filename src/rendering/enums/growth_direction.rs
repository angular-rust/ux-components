#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrowthDirection {
    // This sliver's contents are ordered in the same direction as the AxisDirection.
    Forward = 0,

    // This sliver's contents are ordered in the opposite direction of the AxisDirection.
    Reverse = 1,
}

impl Default for GrowthDirection {
    fn default() -> Self {
        Self::Forward
    }
}