use crate::ui::Radius;

/*
add(BorderRadiusGeometry other) → BorderRadiusGeometry
Returns the sum of two BorderRadiusGeometry objects. [...]
override

copyWith({Radius? topLeft, Radius? topRight, Radius? bottomLeft, Radius? bottomRight}) → BorderRadius
Returns a copy of this BorderRadius with the given fields replaced with the new values.

resolve(TextDirection? direction) → BorderRadius
Convert this instance into a BorderRadius, so that the radii are expressed for specific physical corners (top-left, top-right, etc) rather than in a direction-dependent manner. [...]
override

subtract(BorderRadiusGeometry other) → BorderRadiusGeometry
Returns the difference between two BorderRadiusGeometry objects. [...]
override

toRRect(Rect rect) → RRect
Creates an RRect from the current border radius and a Rect.
*/

pub struct BorderRadius {
    pub top_left: Radius,
    pub top_right: Radius,
    pub bottom_left: Radius,
    pub bottom_right: Radius,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            top_left: Default::default(),
            top_right: Default::default(),
            bottom_left: Default::default(),
            bottom_right: Default::default(),
        }
    }
}
