use crate::foundation::colorspace::Color;

use super::Offset;

pub struct Shadow {
    pub color: Color,     // = const Color(_kColorDefault),
    pub offset: Offset,   // = Offset.zero,
    pub blur_radius: f32, // = 0.0
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            color: Default::default(),
            offset: Default::default(),
            blur_radius: Default::default(),
        }
    }
}
