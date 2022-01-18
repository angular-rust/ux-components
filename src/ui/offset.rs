/// Offset(double dx, double dy)
/// Creates an offset. The first argument sets dx, the horizontal component, and the second sets dy, the vertical component.
/// const
/// Offset.fromDirection(double direction, [double distance = 1.0])
/// Creates an offset from its direction and distance. [...]

pub struct Offset;

impl Default for Offset {
    fn default() -> Self {
        Self {}
    }
}
