#[derive(Debug, Clone, PartialEq)]
pub struct Radius(pub f32, pub f32);

impl Radius {
    pub const ZERO: Radius = Radius(0.0, 0.0);

    // Constructs a circular radius. x and y will have the same radius value.
    pub fn circular(radius: f32) -> Self {
        Self(radius, radius)
    }

    // Constructs an elliptical radius with the given radii.
    pub fn elliptical(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.0
    }
}

impl Default for Radius {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
