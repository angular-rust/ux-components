use crate::prelude::color;

use crate::foundation::colorspace::Color;

use super::BorderStyle;

// scale(double t) -> BorderSide
// Creates a copy of this border side description but with the width scaled by the factor t.
//
// toPaint() -> Paint
// Create a Paint object that, if used to stroke a line, will draw the line in this border's style.

pub struct BorderSide {
    pub color: Color,
    pub width: f32,
    pub style: BorderStyle,
}

impl BorderSide {
    // A hairline black border that is not rendered.
    pub const NONE: BorderSide = BorderSide{
        width: 0.0, 
        style: BorderStyle::None,
        color: color::BLACK,
    };
}

impl Default for BorderSide {
    fn default() -> Self {
        Self {
            color: Default::default(),
            width: Default::default(),
            style: Default::default(),
        }
    }
}
