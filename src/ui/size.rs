// width, height

#[derive(Debug, Clone, PartialEq)]
pub struct Size(pub f32, pub f32);

impl Size {
    // Creates a square Size whose width and height are twice the given dimension.
    pub fn from_radius(radius: f32) -> Self {
        Self(radius * 2.0, radius * 2.0)
    }

    // Creates a Size with the given height and an infinite width.
    pub fn from_height(height: f32) -> Self {
        Self(f32::INFINITY, height)
    }

    // Creates a Size with the given width and an infinite height.
    pub fn from_width(width: f32) -> Self {
        Self(width, f32::INFINITY)
    }

    // Creates a square Size whose width and height are the given dimension.
    pub fn square(dimension: f32) -> Self {
        Self(dimension, dimension)
    }
}
impl Default for Size {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
