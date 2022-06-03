use crate::foundation::colorspace::Color;

use super::{Offset, Paint};

pub struct Shadow {
    // The blurRadius in sigmas instead of logical pixels.
    pub blur_sigma: f32,
    // Color that the shadow will be drawn with.
    pub color: Color,     // = const Color(_kColorDefault),
    // The displacement of the shadow from the casting element.
    pub offset: Offset,   // = Offset.zero,
    // The standard deviation of the Gaussian to convolve with the shadow's shape.
    pub blur_radius: f32, // = 0.0
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            blur_sigma: Default::default(),
            color: Default::default(),
            offset: Default::default(),
            blur_radius: Default::default(),
        }
    }
}

impl Shadow {
    // Returns a new shadow with its offset and blurRadius scaled by the given factor. 
    pub fn scale(factor: f32) -> Shadow {
        todo!()
    }
    
    // Create the Paint object that corresponds to this shadow description. 
    pub fn to_paint() -> Paint {
        todo!()
    }
}