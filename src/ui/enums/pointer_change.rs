#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerChange {
    Cancel = 0,
    Add = 1,
    Remove = 2,
    Hover = 3,
    Down = 4,
    Move = 5,
    Up = 6,
}
