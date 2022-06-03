pub struct FloatingLabelAlignment(pub f32);

impl FloatingLabelAlignment {
    // Aligns the floating label to the center of an InputDecorator.
    pub const CENTER: FloatingLabelAlignment = FloatingLabelAlignment(0.0);

    // Align the floating label on the leading edge of the InputDecorator.
    pub const START: FloatingLabelAlignment = FloatingLabelAlignment(-1.0);

}