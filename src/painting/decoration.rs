// createBoxPainter([VoidCallback onChanged]) -> BoxPainter
// Returns a BoxPainter that will paint this decoration.
// @factory
//
// debugAssertIsValid() -> bool
// In debug mode, throws an exception if the object is not in a valid configuration. Otherwise, returns true.
//
// getClipPath(Rect rect, TextDirection textDirection) -> Path
// Returns a closed Path that describes the outer edge of this decoration.
//
// hitTest(Size size, Offset position, {TextDirection? textDirection}) -> bool
// Tests whether the given point, on a rectangle of a given size, would be considered to hit the decoration or not. For example, if the decoration only draws a circle, this function might return true if the point was inside the circle and false otherwise.
//
// lerpFrom(Decoration? a, double t) -> Decoration?
// Linearly interpolates from another Decoration (which may be of a different class) to this.
// @protected
//
// lerpTo(Decoration? b, double t) -> Decoration?
// Linearly interpolates from this to another Decoration (which may be of a different class).
// @protected
//
// toStringShort() -> String
// A brief description of this object, usually just the runtimeType and the hashCode.
// override

pub struct Decoration;

impl Default for Decoration {
    fn default() -> Self {
        Self {}
    }
}
