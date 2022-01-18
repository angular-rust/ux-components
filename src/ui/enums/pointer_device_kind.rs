#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerDeviceKind {
    Touch = 0,
    Mouse = 1,
    Stylus = 2,
    InvertedStylus = 3,
    Unknown = 4,
}
