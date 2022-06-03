pub trait ShapeBorder {
    // The widths of the sides of this border represented as an EdgeInsets.
    // dimensions: EdgeInsetsGeometry

    // Attempts to create a new object that represents the amalgamation of this border and the other border.
    // add(ShapeBorder other, {bool reversed = false}) → ShapeBorder?

    // Create a Path that describes the inner edge of the border.
    // getInnerPath(Rect rect, {TextDirection? textDirection}) → Path

    // Create a Path that describes the outer edge of the border.
    // getOuterPath(Rect rect, {TextDirection? textDirection}) → Path

    // Linearly interpolates from another ShapeBorder (possibly of another class) to this.
    // lerpFrom(ShapeBorder? a, double t) → ShapeBorder?

    // Linearly interpolates from this to another ShapeBorder (possibly of another class).
    // lerpTo(ShapeBorder? b, double t) → ShapeBorder?

    // Paints the border within the given Rect on the given Canvas.
    // paint(Canvas canvas, Rect rect, {TextDirection? textDirection}) → void

    // Creates a copy of this border, scaled by the factor t.
    // scale(double t) → ShapeBorder
}

pub struct NoneShapeBorder;

impl ShapeBorder for NoneShapeBorder {}

impl Default for NoneShapeBorder {
    fn default() -> Self {
        Self
    }
}
