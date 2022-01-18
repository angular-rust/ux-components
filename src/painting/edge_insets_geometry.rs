// add(EdgeInsetsGeometry other) -> EdgeInsetsGeometry
// Returns the sum of two EdgeInsetsGeometry objects.
//
// along(Axis axis) -> double
// The total offset in the given direction.
//
// clamp(EdgeInsetsGeometry min, EdgeInsetsGeometry max) -> EdgeInsetsGeometry
// Returns a new EdgeInsetsGeometry object with all values greater than or equal to min, and less than or equal to max.
//
// deflateSize(Size size) -> Size
// Returns a new size that is smaller than the given size by the amount of inset in the horizontal and vertical directions.
//
// inflateSize(Size size) -> Size
// Returns a new size that is bigger than the given size by the amount of inset in the horizontal and vertical directions.
//
// resolve(TextDirection? direction) -> EdgeInsets
// Convert this instance into an EdgeInsets, which uses literal coordinates (i.e. the left coordinate being explicitly a
// distance from the left, and the right coordinate being explicitly a distance from the right).
//
// subtract(EdgeInsetsGeometry other) -> EdgeInsetsGeometry
// Returns the difference between two EdgeInsetsGeometry objects.
//
// toString() -> String
// A string representation of this object.

pub struct EdgeInsetsGeometry;

impl Default for EdgeInsetsGeometry {
    fn default() -> Self {
        Self {}
    }
}
