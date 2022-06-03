use crate::ui::Radius;

use super::BorderRadiusGeometry;

pub struct BorderRadius {
    pub top_left: Radius,
    pub top_right: Radius,
    pub bottom_left: Radius,
    pub bottom_right: Radius,
}

impl BorderRadius {
    // Creates a border radius where all radii are radius.
    pub fn all(radius: Radius) -> Self {
        Self {
            top_left: radius.clone(),
            top_right: radius.clone(),
            bottom_left: radius.clone(),
            bottom_right: radius,
        }
    }

    // Creates a border radius where all radii are Radius.circular(radius).
    pub fn circular(radius: f32) -> Self {
        Self {
            top_left: Radius(radius, radius),
            top_right: Radius(radius, radius),
            bottom_left: Radius(radius, radius),
            bottom_right: Radius(radius, radius),
        }
    }

    // // Creates a horizontally symmetrical border radius where the left and right sides of the rectangle have the same radii.
    // pub fn horizontal({Radius left = Radius::ZERO, Radius right = Radius::ZERO}) -> Self {
    //     todo!()
    // }

    // // Creates a border radius with only the given non-zero values. The other corners will be right angles.
    // pub fn only({Radius top_left = Radius::ZERO, Radius top_right = Radius::ZERO, Radius bottom_left = Radius::ZERO, Radius bottom_right = Radius::ZERO}) -> Self {
    //     todo!()
    // }

    // // Creates a vertically symmetric border radius where the top and bottom sides of the rectangle have the same radii.
    // pub fn vertical({Radius top = Radius::ZERO, Radius bottom = Radius::ZERO}) -> Self {
    //     todo!()
    // }

    // Returns a copy of this BorderRadius with the given fields replaced with the new values.
    // copyWith(&self, {Radius? topLeft, Radius? topRight, Radius? bottomLeft, Radius? bottomRight}) → BorderRadius

    // Creates an RRect from the current border radius and a Rect.
    // toRRect(&self, Rect rect) → RRect
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

impl BorderRadiusGeometry for BorderRadius {
    fn add(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry> {
        todo!()
    }

    fn resolve(&self, direction: Option<crate::ui::TextDirection>) -> BorderRadius {
        todo!()
    }

    fn subtract(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry> {
        todo!()
    }
}
