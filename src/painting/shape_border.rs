/*
add(ShapeBorder other, {bool reversed = false}) → ShapeBorder?
Attempts to create a new object that represents the amalgamation of this border and the other border. [...]
@protected

getInnerPath(Rect rect, {TextDirection? textDirection}) → Path
Create a Path that describes the inner edge of the border. [...]

getOuterPath(Rect rect, {TextDirection? textDirection}) → Path
Create a Path that describes the outer edge of the border. [...]

lerpFrom(ShapeBorder? a, double t) → ShapeBorder?
Linearly interpolates from another ShapeBorder (possibly of another class) to this. [...]
@protected

lerpTo(ShapeBorder? b, double t) → ShapeBorder?
Linearly interpolates from this to another ShapeBorder (possibly of another class). [...]
@protected

paint(Canvas canvas, Rect rect, {TextDirection? textDirection}) → void
Paints the border within the given Rect on the given Canvas. [...]

scale(double t) → ShapeBorder
Creates a copy of this border, scaled by the factor t. [...]

toString() → String
A string representation of this object. [...]
override
*/

pub struct ShapeBorder;

impl Default for ShapeBorder {
    fn default() -> Self {
        Self {}
    }
}
