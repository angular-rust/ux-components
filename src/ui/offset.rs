pub struct Offset {
    // // The angle of this offset as radians clockwise from the positive x-axis, 
    // // in the range -pi to pi, assuming positive values of the x-axis go to the 
    // // right and positive values of the y-axis go down.
    // pub direction: f32,
    
    // // The magnitude of the offset.
    // pub distance: f32,
    
    // // The square of the magnitude of the offset.
    // pub distance_squared: f32,
    
    // The x component of the offset.
    pub dx: f32,
    
    // The y component of the offset.
    pub dy: f32,
    
    // // Whether both components are finite (neither infinite nor NaN).
    // pub is_finite: bool,
    
    // // Returns true if either component is double.infinity, 
    // // and false if both are finite (or negative infinity, or NaN).
    // pub is_infinite: bool,
}

impl Offset {
    // Creates an offset. The first argument sets dx, the horizontal component, and the second sets dy, the vertical component.
    // Offset(double dx, double dy)

    // Creates an offset from its direction and distance.
    // Offset.fromDirection(double direction, [double distance = 1.0])

}

impl Default for Offset {
    fn default() -> Self {
        Self {
            // direction: Default::default(),
            // distance: Default::default(),
            // distance_squared: Default::default(),
            dx: Default::default(),
            dy: Default::default(),
            // is_finite: Default::default(),
            // is_infinite: Default::default(),
        }
    }
}
