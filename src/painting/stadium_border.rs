use super::{BorderSide, EdgeInsetsGeometry, OutlinedBorder, ShapeBorder};

pub struct StadiumBorder {
    // The widths of the sides of this border represented as an EdgeInsets.
    pub dimensions: Box<dyn EdgeInsetsGeometry>,

    // The border outline's color and weight.
    pub side: BorderSide,
}

impl StadiumBorder {
    // add(ShapeBorder other, {bool reversed = false}) → ShapeBorder?
    // Attempts to create a new object that represents the amalgamation of this border and the other border.
    // @protected, inherited
    // copyWith({BorderSide? side, BorderRadiusGeometry? borderRadius}) → RoundedRectangleBorder
    // Returns a copy of this RoundedRectangleBorder with the given fields replaced with the new values.
    // override
    // getInnerPath(Rect rect, {TextDirection? textDirection}) → Path
    // Create a Path that describes the inner edge of the border.
    // override
    // getOuterPath(Rect rect, {TextDirection? textDirection}) → Path
    // Create a Path that describes the outer edge of the border.
    // override
    // lerpFrom(ShapeBorder? a, double t) → ShapeBorder?
    // Linearly interpolates from another ShapeBorder (possibly of another class) to this.
    // override
    // lerpTo(ShapeBorder? b, double t) → ShapeBorder?
    // Linearly interpolates from this to another ShapeBorder (possibly of another class).
    // override
    // paint(Canvas canvas, Rect rect, {TextDirection? textDirection}) → void
    // Paints the border within the given Rect on the given Canvas.
    // override
    // scale(double t) → ShapeBorder
    // Creates a copy of this border, scaled by the factor t.
    // override
    // toString() → String
    // A string representation of this object.
    // override
}

impl OutlinedBorder for StadiumBorder {}

impl ShapeBorder for StadiumBorder {}
