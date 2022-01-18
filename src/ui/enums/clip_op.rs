#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipOp {
    Difference = 0,
    Intersect = 1,
}
