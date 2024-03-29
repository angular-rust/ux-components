#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointMode {
    Points = 0,
    Lines = 1,
    Polygon = 2,
}

impl Default for PointMode {
    fn default() -> Self {
        Self::Points
    }
}