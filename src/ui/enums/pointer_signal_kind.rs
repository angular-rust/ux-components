#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerSignalKind {
    None = 0,
    Scroll = 1,
    Unknown = 2,
}
