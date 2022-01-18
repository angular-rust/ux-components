// Radius.circular(double radius)
// Constructs a circular radius. x and y will have the same radius value.
// const
//
// Radius.elliptical(double x, double y)
// Constructs an elliptical radius with the given radii.

pub struct Radius(f32, f32);

impl Default for Radius {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
